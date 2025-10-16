pub mod traits;
pub mod crud;

pub use traits::{Model, FromRow};
pub use crud::{ModelCrud, ModelQuery};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a value that can be stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    F64(f64),
    String(String),
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::I32(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::I64(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::F64(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<Option<String>> for Value {
    fn from(v: Option<String>) -> Self {
        v.map(Value::String).unwrap_or(Value::Null)
    }
}

/// Represents a row of data from the database
pub type Row = HashMap<String, Value>;

/// Helper to convert Value to SQL string representation
impl Value {
    /// Convert to SQL string (DEPRECATED: vulnerable to SQL injection, use to_query_value instead)
    pub fn to_sql_string(&self) -> String {
        match self {
            Value::Null => "NULL".to_string(),
            Value::Bool(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
            Value::I32(n) => n.to_string(),
            Value::I64(n) => n.to_string(),
            Value::F64(n) => n.to_string(),
            Value::String(s) => format!("'{}'", s.replace('\'', "''")),
        }
    }

    /// Convert to QueryValue for parameterized queries (safe from SQL injection)
    pub fn to_query_value(&self) -> crate::query::QueryValue {
        match self {
            Value::Null => crate::query::QueryValue::Null,
            Value::Bool(b) => crate::query::QueryValue::Bool(*b),
            Value::I32(n) => crate::query::QueryValue::I32(*n),
            Value::I64(n) => crate::query::QueryValue::I64(*n),
            Value::F64(n) => crate::query::QueryValue::F64(*n),
            Value::String(s) => crate::query::QueryValue::String(s.clone()),
        }
    }
}
