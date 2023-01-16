import { appCacheDir, resourceDir, appDataDir,runtimeDir, resolveResource, resolve } from '@tauri-apps/api/path';
import { exists, readDir, BaseDirectory } from '@tauri-apps/api/fs';
// import Database from "tauri-plugin-sql-api";
import Database from "./sqlite";

export async function init_db(){
    let db;
    try{
        // 1 连接数据库,如果不存在会自动创建
        const resource_path = await resolveResource('data');
        console.log(resource_path);
        const db_path = resource_path.replace(/^(\\\\\?\\)/,'');
        console.log(db_path);
        // 如果不写绝对路径,默认会创建在%APPDATA%/Roaming\danmuapp下
        // db = await Database.open(`sqlite:${db_path}`);
        db = await Database.open(`${db_path}`);
        console.log(db);
        // 2 检查表,如果没有任何表则创建表,并初始化基础数据
        let rst = await db.select("select * from sqlite_master");
        // console.log(rst);
        if(rst.length == 0) {
            let r = await db.execute("CREATE TABLE \"danmu_msg\" (\n\
                \"id\" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,\n\
                content TEXT,-- 弹幕内容\n\
                ts INTEGER,-- 发送时间，毫秒时间戳\n\
                lottery boolean,-- 是否为天选抽奖弹幕\n\
                emoticon_id TEXT,-- 弹幕表情\n\
                emoticon_height INTEGER,\n\
                emoticon_width INTEGER,\n\
                emoticon_url TEXT,\n\
                uid INTEGER,\n\
                uname TEXT,--用户名\n\
                face TEXT,--用户头像\n\
                upid  INTEGER,\n\
                upname  TEXT,\n\
                roomid  INTEGER,\n\
                badge_active boolean,--用户牌子是否点亮\n\
                badge_name TEXT,--牌子名称\n\
                badge_level INTEGER,--牌子等级\n\
                badge_color TEXT,--牌子颜色\n\
                gradient TEXT,--渐变色牌子，当用户长时间未消费，则会变为灰色，即 '#c0c0c0'\n\
                anchor_uid INTEGER,--主播uid\n\
                anchor_uname TEXT,--播用户名\n\
                anchor_room_id INTEGER,--主播房间号\n\
                anchor_is_same_room boolean,--是否为本直播间\n\
                rank INTEGER,--直播榜单排名\n\
                guard_level INTEGER,--大航海信息\n\
                room_admin boolean--是否房管\n\
            );");
            console.log('创建表:danmu_msg',r);
            let r2 = await db.execute("CREATE TABLE \"chatterbox\" (\n\
                \"id\" integer NOT NULL PRIMARY KEY AUTOINCREMENT,\n\
                \"roomid\" INTEGER NOT NULL,\n\
                \"msg\" TEXT,\n\
                \"enable\" boolean\n\
              );");

              console.log('创建表:chatterbox',r2);
        }else{
        // 3 检查版本是否一致,是否需要改变表结构以数据升级

        }
        // 4 返回数据库对象
        return db;
    }catch(e){
        console.error(e.message,e);
        if(db)
        db.close();
    }
}

export async function insert_danmu_msg(db,data){
    const item = {};
    item.content = data.content
    item.ts = data.timestamp
    item.lottery = data.lottery
    item.emoticon_id   = data.emoticon?.id
    item.emoticon_height   = data.emoticon?.height
    item.emoticon_width   = data.emoticon?.width
    item.emoticon_url  = data.emoticon?.url
    item.uid   = data.user.uid
    item.uname  = data.user.uname
    item.upid   = data.upid
    item.upname  = data.upname
    item.roomid  = data.roomid
    item.face   = data.user.face
    item.badge_active  = data.user.badge?.active
    item.badge_name  = data.user.badge?.name
    item.badge_level   = data.user.badge?.level 
    item.badge_color   = data.user.badge?.color
    item.gradient   = data.user.badge?.gradient
    item.anchor_uid   = data.user.badge?.anchor.uid
    item.anchor_uname   = data.user.badge?.anchor.uname
    item.anchor_room_id   = data.user.badge?.anchor.room_id
    item.anchor_is_same_room   = data.user.badge?.anchor.is_same_room
    item.rank   = data.user.identity?.rank
    item.guard_level   = data.user.identity?.guard_level
    item.room_admin   = data.user.identity?.room_admin
    // console.log(item);

    const sql = gen_insert_sql("danmu_msg",item);
    console.log(sql);
    const rst = await db.execute(sql);
    console.log("保存弹幕结果: ",rst);
}

export async function get_danmu_msg(db,filter,pageNum,pageSize){
    let sql = `select * from danmu_msg`;
    if(filter){
        sql += ` where uname like '%${filter}%' or content like '%${filter}%'`
    }
    return get_pagination_data(db,sql,pageNum,pageSize);
}
//查询话痨
export async function get_chatterbox(db,roomid){
    let sql = `select * from chatterbox where roomid='${roomid}'`;
    return db.select(sql);
}
//保存话痨
export async function save_chatterbox(db,room_id,datas){
    db.execute(`delete from chatterbox where roomid=${room_id}`)
    //实际上支持批量保存,我只是懒
    let count = 0;
    for (const key in datas) {
        let sql = gen_insert_sql('chatterbox',datas[key])
        count += await db.execute(sql);  
    }
    return count
}
// 分页查询
async function get_pagination_data(db,selectSql,pageNum,pageSize){
    const sqlCount = `SELECT COUNT(1) as count from (${selectSql})`;
    const rst_count = await db.select(sqlCount);
    console.log(rst_count)
    const count = rst_count[0].count;
    const sql = `SELECT t.* from (${selectSql}) t limit ${(pageNum - 1) * pageSize},${pageSize}`;
    console.log(sql);
    const data =  await db.select(sql);
    console.log(data);
    const rst = {
        count : count,
        rows : data
    };    
    return rst;
}

function gen_insert_sql(tableName,item){
    let column_sql_part = ""
    let value_sql_part = ""
    for(let key in item){
        const v = item[key]
        if(v !== undefined){
        column_sql_part+=(`\"${key}\",`)
            if(typeof(v) === 'string'){
                value_sql_part+=(`\"${v}\",`)
            }else if(Array.isArray(v)){
                value_sql_part+=(`'["${v.join("\",\"")}"]',`)
            }else if(typeof(v) === 'boolean'){
                value_sql_part+=(`\"${v}\",`)
            }else{
                value_sql_part+=(`${v},`)
            }
        }
    }
    column_sql_part = column_sql_part.replaceAll(/,$/g,'')
    value_sql_part = value_sql_part.replaceAll(/,$/g,'')
    const sql = `INSERT INTO "${tableName}"(${column_sql_part}) VALUES (${value_sql_part})`;
    return sql;
}