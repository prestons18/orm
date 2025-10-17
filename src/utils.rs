use sqlx::{Column, Row};

/// Convert a SQLite row to JSON
pub fn sqlite_row_to_json(row: &sqlx::sqlite::SqliteRow) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    for (i, column) in row.columns().iter().enumerate() {
        let column_name = column.name();
        
        let value = if let Ok(v) = row.try_get::<i64, _>(i) {
            serde_json::json!(v)
        } else if let Ok(v) = row.try_get::<f64, _>(i) {
            serde_json::json!(v)
        } else if let Ok(v) = row.try_get::<bool, _>(i) {
            serde_json::Value::Bool(v)
        } else if let Ok(v) = row.try_get::<String, _>(i) {
            serde_json::Value::String(v)
        } else if let Ok(v) = row.try_get::<Vec<u8>, _>(i) {
            serde_json::Value::String(base64_encode(&v))
        } else {
            serde_json::Value::Null
        };
        
        obj.insert(column_name.to_string(), value);
    }
    serde_json::Value::Object(obj)
}

/// Convert a MySQL row to JSON
pub fn mysql_row_to_json(row: &sqlx::mysql::MySqlRow) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    for (i, column) in row.columns().iter().enumerate() {
        let column_name = column.name();
        
        let value = if let Ok(v) = row.try_get::<i64, _>(i) {
            serde_json::json!(v)
        } else if let Ok(v) = row.try_get::<i32, _>(i) {
            serde_json::json!(v)
        } else if let Ok(v) = row.try_get::<f64, _>(i) {
            serde_json::json!(v)
        } else if let Ok(v) = row.try_get::<bool, _>(i) {
            serde_json::Value::Bool(v)
        } else if let Ok(v) = row.try_get::<String, _>(i) {
            serde_json::Value::String(v)
        } else if let Ok(v) = row.try_get::<Vec<u8>, _>(i) {
            serde_json::Value::String(base64_encode(&v))
        } else {
            serde_json::Value::Null
        };
        
        obj.insert(column_name.to_string(), value);
    }
    serde_json::Value::Object(obj)
}

/// Simple base64 encoding without external dependency
fn base64_encode(bytes: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let b1 = chunk[0];
        let b2 = chunk.get(1).copied().unwrap_or(0);
        let b3 = chunk.get(2).copied().unwrap_or(0);
        
        result.push(CHARSET[(b1 >> 2) as usize] as char);
        result.push(CHARSET[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(CHARSET[(((b2 & 0x0f) << 2) | (b3 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(CHARSET[(b3 & 0x3f) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}