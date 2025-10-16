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

    /// Helper to bind all parameters to a query
    fn bind_params<'q>(&'q self, mut query: sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>>) -> sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>> {
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
        query
    }

    /// Helper to convert a row to JSON
    fn row_to_json(row: &sqlx::any::AnyRow) -> serde_json::Value {
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
        serde_json::Value::Object(obj)
    }

    /// Execute a SELECT query and return results as JSON values
    pub async fn fetch_all(&self) -> Result<Vec<serde_json::Value>> {
        let pool = self.pool.as_ref().ok_or_else(|| {
            Error::QueryError("No connection pool available".to_string())
        })?;

        let query = self.bind_params(sqlx::query(&self.sql));
        let rows = query.fetch_all(pool).await?;

        Ok(rows.iter().map(Self::row_to_json).collect())
    }

    /// Execute a SELECT query and return a single result
    pub async fn fetch_one(&self) -> Result<Option<serde_json::Value>> {
        let pool = self.pool.as_ref().ok_or_else(|| {
            Error::QueryError("No connection pool available".to_string())
        })?;

        let query = self.bind_params(sqlx::query(&self.sql));
        let row = query.fetch_optional(pool).await?;

        Ok(row.as_ref().map(Self::row_to_json))
    }

    /// Execute INSERT/UPDATE/DELETE and return affected rows
    pub async fn execute(&self) -> Result<u64> {
        let pool = self.pool.as_ref().ok_or_else(|| {
            Error::QueryError("No connection pool available".to_string())
        })?;

        let query = self.bind_params(sqlx::query(&self.sql));
        let result = query.execute(pool).await?;
        Ok(result.rows_affected())
    }
}