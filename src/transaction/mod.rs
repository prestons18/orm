use crate::error::Result;
use sqlx::{MySqlPool, SqlitePool};

/// Enum to hold different transaction types
pub enum TransactionInner {
    SQLite(sqlx::Transaction<'static, sqlx::Sqlite>),
    MySQL(sqlx::Transaction<'static, sqlx::MySql>),
}

/// Represents a database transaction
pub struct Transaction {
    inner: Option<TransactionInner>,
}

impl Transaction {
    /// Create a new SQLite transaction
    pub(crate) async fn new_sqlite(pool: &SqlitePool) -> Result<Self> {
        let tx = pool.begin().await?;
        Ok(Self {
            inner: Some(TransactionInner::SQLite(tx)),
        })
    }

    /// Create a new MySQL transaction
    pub(crate) async fn new_mysql(pool: &MySqlPool) -> Result<Self> {
        let tx = pool.begin().await?;
        Ok(Self {
            inner: Some(TransactionInner::MySQL(tx)),
        })
    }

    /// Commit the transaction
    pub async fn commit(mut self) -> Result<()> {
        if let Some(inner) = self.inner.take() {
            match inner {
                TransactionInner::SQLite(tx) => {
                    tx.commit().await?;
                }
                TransactionInner::MySQL(tx) => {
                    tx.commit().await?;
                }
            }
        }
        Ok(())
    }

    /// Rollback the transaction
    pub async fn rollback(mut self) -> Result<()> {
        if let Some(inner) = self.inner.take() {
            match inner {
                TransactionInner::SQLite(tx) => {
                    tx.rollback().await?;
                }
                TransactionInner::MySQL(tx) => {
                    tx.rollback().await?;
                }
            }
        }
        Ok(())
    }

    /// Execute raw SQL within the transaction
    pub async fn execute(&mut self, sql: &str) -> Result<u64> {
        if let Some(inner) = &mut self.inner {
            let rows_affected = match inner {
                TransactionInner::SQLite(tx) => {
                    let result = sqlx::query(sql).execute(&mut **tx).await?;
                    result.rows_affected()
                }
                TransactionInner::MySQL(tx) => {
                    let result = sqlx::query(sql).execute(&mut **tx).await?;
                    result.rows_affected()
                }
            };
            Ok(rows_affected)
        } else {
            Err(crate::error::Error::QueryError(
                "Transaction already completed".to_string(),
            ))
        }
    }

    /// Fetch all rows from a query as JSON values
    pub async fn fetch_all(&mut self, sql: &str) -> Result<Vec<serde_json::Value>> {
        use sqlx::{Column, Row};

        if let Some(inner) = &mut self.inner {
            let results = match inner {
                TransactionInner::SQLite(tx) => {
                    let rows = sqlx::query(sql).fetch_all(&mut **tx).await?;
                    rows.iter()
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
                        .collect()
                }
                TransactionInner::MySQL(tx) => {
                    let rows = sqlx::query(sql).fetch_all(&mut **tx).await?;
                    rows.iter()
                        .map(|row| {
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
                                } else {
                                    serde_json::Value::Null
                                };
                                obj.insert(column_name.to_string(), value);
                            }
                            serde_json::Value::Object(obj)
                        })
                        .collect()
                }
            };
            Ok(results)
        } else {
            Err(crate::error::Error::QueryError(
                "Transaction already completed".to_string(),
            ))
        }
    }

    /// Fetch one row from a query as JSON value
    pub async fn fetch_one(&mut self, sql: &str) -> Result<Option<serde_json::Value>> {
        use sqlx::{Column, Row};

        if let Some(inner) = &mut self.inner {
            let result = match inner {
                TransactionInner::SQLite(tx) => {
                    let row_opt = sqlx::query(sql).fetch_optional(&mut **tx).await?;
                    row_opt.as_ref().map(|row| {
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
                }
                TransactionInner::MySQL(tx) => {
                    let row_opt = sqlx::query(sql).fetch_optional(&mut **tx).await?;
                    row_opt.as_ref().map(|row| {
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
                            } else {
                                serde_json::Value::Null
                            };
                            obj.insert(column_name.to_string(), value);
                        }
                        serde_json::Value::Object(obj)
                    })
                }
            };
            Ok(result)
        } else {
            Err(crate::error::Error::QueryError(
                "Transaction already completed".to_string(),
            ))
        }
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        // Auto-rollback on drop if transaction wasn't committed or rolled back
        // The sqlx transaction will handle this automatically
    }
}