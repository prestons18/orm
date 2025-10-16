use crate::error::{Error, Result};
use serde_json;
use sqlx::{AnyPool, Column, Row};

/// Value type for query parameters
#[derive(Debug, Clone)]
pub enum QueryValue {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    F64(f64),
    String(String),
}

/// Query executor for running built queries
pub struct QueryExecutor {
    sql: String,
    params: Vec<QueryValue>,
    pool: Option<AnyPool>,
}

impl QueryExecutor {
    pub fn new(sql: String) -> Self {
        Self {
            sql,
            params: Vec::new(),
            pool: None,
        }
    }

    /// Create a new executor with a connection pool
    pub fn with_pool(sql: String, pool: AnyPool) -> Self {
        Self {
            sql,
            params: Vec::new(),
            pool: Some(pool),
        }
    }

    /// Bind a parameter to the query
    pub fn bind(mut self, value: QueryValue) -> Self {
        self.params.push(value);
        self
    }

    pub fn sql(&self) -> &str {
        &self.sql
    }

    /// Execute a SELECT query and return results as JSON values
    pub async fn fetch_all(&self) -> Result<Vec<serde_json::Value>> {
        let pool = self.pool.as_ref().ok_or_else(|| {
            Error::QueryError("No connection pool available".to_string())
        })?;

        let mut query = sqlx::query(&self.sql);
        for param in &self.params {
            query = match param {
                QueryValue::Null => query.bind(Option::<i32>::None),
                QueryValue::Bool(v) => query.bind(*v),
                QueryValue::I32(v) => query.bind(*v),
                QueryValue::I64(v) => query.bind(*v),
                QueryValue::F64(v) => query.bind(*v),
                QueryValue::String(v) => query.bind(v.as_str()),
            };
        }

        let rows = query.fetch_all(pool).await?;

        // Convert rows to JSON
        let mut results = Vec::new();
        for row in rows {
            let mut obj = serde_json::Map::new();
            for (i, column) in row.columns().iter().enumerate() {
                let column_name = column.name();
                // Try common types, fallback to string
                let value = if let Ok(v) = row.try_get::<i64, _>(i) {
                    serde_json::json!(v)
                } else if let Ok(v) = row.try_get::<f64, _>(i) {
                    serde_json::json!(v)
                } else if let Ok(v) = row.try_get::<bool, _>(i) {
                    serde_json::Value::Bool(v)
                } else if let Ok(v) = row.try_get::<String, _>(i) {
                    serde_json::Value::String(v)
                } else {
                    serde_json::Value::Null
                };
                obj.insert(column_name.to_string(), value);
            }
            results.push(serde_json::Value::Object(obj));
        }

        Ok(results)
    }

    /// Execute a SELECT query and return a single result
    pub async fn fetch_one(&self) -> Result<Option<serde_json::Value>> {
        let pool = self.pool.as_ref().ok_or_else(|| {
            Error::QueryError("No connection pool available".to_string())
        })?;

        let mut query = sqlx::query(&self.sql);
        for param in &self.params {
            query = match param {
                QueryValue::Null => query.bind(Option::<i32>::None),
                QueryValue::Bool(v) => query.bind(*v),
                QueryValue::I32(v) => query.bind(*v),
                QueryValue::I64(v) => query.bind(*v),
                QueryValue::F64(v) => query.bind(*v),
                QueryValue::String(v) => query.bind(v.as_str()),
            };
        }

        let row = query.fetch_optional(pool).await?;

        if let Some(row) = row {
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
                } else {
                    serde_json::Value::Null
                };
                obj.insert(column_name.to_string(), value);
            }
            Ok(Some(serde_json::Value::Object(obj)))
        } else {
            Ok(None)
        }
    }

    /// Execute INSERT/UPDATE/DELETE and return affected rows
    pub async fn execute(&self) -> Result<u64> {
        let pool = self.pool.as_ref().ok_or_else(|| {
            Error::QueryError("No connection pool available".to_string())
        })?;

        let mut query = sqlx::query(&self.sql);
        for param in &self.params {
            query = match param {
                QueryValue::Null => query.bind(Option::<i32>::None),
                QueryValue::Bool(v) => query.bind(*v),
                QueryValue::I32(v) => query.bind(*v),
                QueryValue::I64(v) => query.bind(*v),
                QueryValue::F64(v) => query.bind(*v),
                QueryValue::String(v) => query.bind(v.as_str()),
            };
        }

        let result = query.execute(pool).await?;
        Ok(result.rows_affected())
    }
}