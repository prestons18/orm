use crate::error::Result;
use crate::model::{Row, Value};
use std::collections::HashMap;

/// Core trait that all models must implement
pub trait Model: Sized + Send + Sync {
    /// The name of the database table
    fn table_name() -> &'static str;

    /// The primary key column name
    fn primary_key() -> &'static str {
        "id"
    }

    /// Get the primary key value for this instance
    fn primary_key_value(&self) -> Option<Value>;

    /// Convert model to a map of column names to values
    fn to_values(&self) -> HashMap<String, Value>;

    /// Get the column names for this model (excluding primary key if auto-increment)
    fn columns() -> Vec<&'static str>;

    /// Get all column names including primary key
    fn all_columns() -> Vec<&'static str> {
        let mut cols = vec![Self::primary_key()];
        cols.extend(Self::columns());
        cols
    }
}

/// Trait for converting database rows into model instances
pub trait FromRow: Sized {
    /// Convert a database row into a model instance
    fn from_row(row: &Row) -> Result<Self>;

    /// Convert a JSON value into a model instance
    fn from_json(value: &serde_json::Value) -> Result<Self> {
        let obj = value.as_object().ok_or_else(|| {
            crate::error::Error::SerializationError("Expected JSON object".to_string())
        })?;

        let mut row = HashMap::new();
        for (key, val) in obj {
            let value = match val {
                serde_json::Value::Null => Value::Null,
                serde_json::Value::Bool(b) => Value::Bool(*b),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        Value::I64(i)
                    } else if let Some(f) = n.as_f64() {
                        Value::F64(f)
                    } else {
                        Value::Null
                    }
                }
                serde_json::Value::String(s) => Value::String(s.clone()),
                _ => Value::Null,
            };
            row.insert(key.clone(), value);
        }

        Self::from_row(&row)
    }
}
