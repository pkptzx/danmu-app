use std::collections::HashMap;

use crate::chrome;

#[tauri::command]
pub fn get_cookies()-> &'static str{
    let key = chrome::get_key();
    let mut result: HashMap<String, String> = HashMap::new();
    let data = chrome::get_raw_cookies(".bilibili.com");
    for (name, encrypted_value) in data.iter() {
        result.insert(name.to_owned(),chrome::decrypt_string(&key, encrypted_value));
    }    
    let json_result = serde_json::json!(result);
    Box::leak(json_result.to_string().into_boxed_str())
}