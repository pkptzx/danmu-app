use serde::{ser::Serializer, Serialize};
use serde_json::{json, Value as JsonValue};
use std::{collections::HashMap, sync::Mutex};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    #[error("database {0} not opened")]
    DatabaseNotOpened(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
struct SqliteMap(Mutex<HashMap<String, rusqlite::Connection>>);

#[command]
async fn open(state: State<'_, SqliteMap>, path: String) -> Result<bool> {
    let connection = rusqlite::Connection::open(&path)?;
    state.0.lock().unwrap().insert(path.clone(), connection);
    Ok(true)
}

#[command]
async fn close(state: State<'_, SqliteMap>, path: String) -> Result<bool> {
    let mut map = state.0.lock().unwrap();
    let connection = map
        .get_mut(&path)
        .ok_or(Error::DatabaseNotOpened(path.clone()))?;
    drop(connection);
    map.remove(&path);
    Ok(true)
}

#[command]
async fn execute(
    state: State<'_, SqliteMap>,
    path: String,
    sql: String,
    values: Option<Vec<JsonValue>>,
) -> Result<usize> {
    let mut result = 0usize;
    let mut map = state.0.lock().unwrap();
    let connection = map.get_mut(&path).ok_or(Error::DatabaseNotOpened(path))?;
    let mut statement = connection.prepare(&sql)?;
    if let Some(vals) = values {
        if vals.get(0).unwrap().is_array() {
            // 批量执行
            for value in vals {
                for (i, v) in value.as_array().unwrap().iter().enumerate() {
                    statement.raw_bind_parameter(i + 1, get_type_value(&v))?;
                }
                result += statement.raw_execute()?;
            }
        } else {
            for (i, v) in vals.iter().enumerate() {
                statement.raw_bind_parameter(i + 1, get_type_value(&v))?;
            }
            result += statement.raw_execute()?;
        }
    } else {
        result += connection.execute(&sql, [])?;
    }

    Ok(result)
}

fn get_type_value(value: &JsonValue) -> rusqlite::types::Value {
    if value.is_null() {
        rusqlite::types::Value::Null
    } else if value.is_i64() {
        rusqlite::types::Value::from(value.as_i64().unwrap())
        // rusqlite::types::Value::Integer()
    } else if value.is_boolean() {
        rusqlite::types::Value::from(value.as_bool().unwrap() as i64)
    } else if value.is_f64() {
        rusqlite::types::Value::from(value.as_f64().unwrap())
    } else if value.is_string() {
        rusqlite::types::Value::from(value.as_str().unwrap().to_owned())
    } else {
        rusqlite::types::Value::from(value.as_str().unwrap().to_owned())
    }
}

#[command]
async fn select(
    state: State<'_, SqliteMap>,
    path: String,
    sql: String,
    values: Vec<JsonValue>,
) -> Result<Vec<HashMap<String, JsonValue>>> {
    let mut map = state.0.lock().unwrap();
    let connection = map.get_mut(&path).ok_or(Error::DatabaseNotOpened(path))?;
    let mut result = Vec::new();
    let mut statement = connection.prepare(&sql)?;
    for (i, v) in values.iter().enumerate() {
        statement.raw_bind_parameter(i + 1, get_type_value(&v))?;
    }
    let column_count = statement.column_count();
    let mut rows = statement.raw_query();
    while let Some(row) = rows.next()? {
        let mut data = std::collections::HashMap::default();
        for idx in 0..column_count {
            let name = row.as_ref().column_name(idx).unwrap().to_string();
            let value = row.get_ref_unwrap(idx);
            let v = match value.data_type() {
                rusqlite::types::Type::Real => json!(value.as_f64_or_null().unwrap()),
                rusqlite::types::Type::Integer => json!(value.as_i64_or_null().unwrap()),
                rusqlite::types::Type::Text => json!(value.as_str_or_null().unwrap().to_owned()),
                rusqlite::types::Type::Blob => json!(value.as_blob_or_null().unwrap()),
                _ => JsonValue::Null,
            };
            data.insert(name.to_owned(), v);
        }
        result.push(data);
    }
    Ok(result)
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("sqlite")
        .invoke_handler(tauri::generate_handler![open, close, execute, select])
        .setup(|app| {
            app.manage(SqliteMap::default());
            Ok(())
        })
        .build()
}
