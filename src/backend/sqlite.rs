use crate::backend::{Backend, BackendFeature, GenericBackend};
use crate::error::Result;
use crate::query::builder::{Dialect, QueryBuilderEnum};
use crate::query::QueryValue;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub type SQLiteBackend = GenericBackend<SqlitePool>;

impl SQLiteBackend {
    pub async fn connect(url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(url).await?;
        Ok(GenericBackend::new(
            pool,
            url.to_string(),
            Dialect::SQLite,
            "SQLite",
        ))
    }
}

#[async_trait]
impl Backend for SQLiteBackend {
    fn name(&self) -> &str {
        self.name
    }

    fn connection_url(&self) -> &str {
        &self.connection_url
    }

    fn query_builder(&self) -> QueryBuilderEnum {
        QueryBuilderEnum::new(self.dialect)
    }

    async fn execute_raw(&self, sql: &str) -> Result<u64> {
        let result = sqlx::query(sql).execute(self.pool()).await?;
        Ok(result.rows_affected())
    }

    async fn execute(&self, sql: &str, params: &[QueryValue]) -> Result<u64> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = match param {
                QueryValue::Null => query.bind(Option::<i64>::None),
                QueryValue::Bool(v) => query.bind(*v),
                QueryValue::I32(v) => query.bind(*v),
                QueryValue::I64(v) => query.bind(*v),
                QueryValue::F64(v) => query.bind(*v),
                QueryValue::String(v) => query.bind(v.as_str()),
            };
        }
        let result = query.execute(self.pool()).await?;
        Ok(result.rows_affected())
    }

    async fn fetch_all(&self, sql: &str) -> Result<Vec<serde_json::Value>> {
        use sqlx::{Column, Row};
        let rows = sqlx::query(sql).fetch_all(self.pool()).await?;
        
        let results = rows
            .iter()
            .map(|row| {
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
            })
            .collect();
        
        Ok(results)
    }

    async fn fetch_all_params(&self, sql: &str, params: &[QueryValue]) -> Result<Vec<serde_json::Value>> {
        use sqlx::{Column, Row};
        let mut query = sqlx::query(sql);
        for param in params {
            query = match param {
                QueryValue::Null => query.bind(Option::<i64>::None),
                QueryValue::Bool(v) => query.bind(*v),
                QueryValue::I32(v) => query.bind(*v),
                QueryValue::I64(v) => query.bind(*v),
                QueryValue::F64(v) => query.bind(*v),
                QueryValue::String(v) => query.bind(v.as_str()),
            };
        }
        let rows = query.fetch_all(self.pool()).await?;
        
        let results = rows
            .iter()
            .map(|row| {
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
            })
            .collect();
        
        Ok(results)
    }

    async fn fetch_one(&self, sql: &str) -> Result<Option<serde_json::Value>> {
        use sqlx::{Column, Row};
        let row_opt = sqlx::query(sql).fetch_optional(self.pool()).await?;
        
        Ok(row_opt.as_ref().map(|row| {
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
        }))
    }

    async fn fetch_one_params(&self, sql: &str, params: &[QueryValue]) -> Result<Option<serde_json::Value>> {
        use sqlx::{Column, Row};
        let mut query = sqlx::query(sql);
        for param in params {
            query = match param {
                QueryValue::Null => query.bind(Option::<i64>::None),
                QueryValue::Bool(v) => query.bind(*v),
                QueryValue::I32(v) => query.bind(*v),
                QueryValue::I64(v) => query.bind(*v),
                QueryValue::F64(v) => query.bind(*v),
                QueryValue::String(v) => query.bind(v.as_str()),
            };
        }
        let row_opt = query.fetch_optional(self.pool()).await?;
        
        Ok(row_opt.as_ref().map(|row| {
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
        }))
    }

    async fn begin_transaction(&self) -> Result<crate::transaction::Transaction> {
        crate::transaction::Transaction::new_sqlite(self.pool()).await
    }

    fn supports_feature(&self, feature: BackendFeature) -> bool {
        match feature {
            BackendFeature::Transactions => true,
            BackendFeature::Savepoints => true,
            BackendFeature::Returning => true,
            BackendFeature::OnConflict => true,
            BackendFeature::CTE => true,
            BackendFeature::Window => true,
        }
    }
}