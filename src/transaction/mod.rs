use crate::error::Result;
use crate::query::QueryValue;
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

    /// Execute raw SQL within the transaction (deprecated - use execute_params)
    #[deprecated(note = "Use execute_params for SQL injection protection")]
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

    /// Execute SQL with parameters within the transaction (safe from SQL injection)
    pub async fn execute_params(&mut self, sql: &str, params: &[QueryValue]) -> Result<u64> {
        if let Some(inner) = &mut self.inner {
            let rows_affected = match inner {
                TransactionInner::SQLite(tx) => {
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
                    let result = query.execute(&mut **tx).await?;
                    result.rows_affected()
                }
                TransactionInner::MySQL(tx) => {
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
                    let result = query.execute(&mut **tx).await?;
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

    /// Fetch all rows from a query as JSON values (deprecated - use fetch_all_params)
    #[deprecated(note = "Use fetch_all_params for SQL injection protection")]
    pub async fn fetch_all(&mut self, sql: &str) -> Result<Vec<serde_json::Value>> {
        if let Some(inner) = &mut self.inner {
            let results = match inner {
                TransactionInner::SQLite(tx) => {
                    let rows = sqlx::query(sql).fetch_all(&mut **tx).await?;
                    rows.iter().map(crate::utils::sqlite_row_to_json).collect()
                }
                TransactionInner::MySQL(tx) => {
                    let rows = sqlx::query(sql).fetch_all(&mut **tx).await?;
                    rows.iter().map(crate::utils::mysql_row_to_json).collect()
                }
            };
            Ok(results)
        } else {
            Err(crate::error::Error::QueryError(
                "Transaction already completed".to_string(),
            ))
        }
    }

    /// Fetch all rows with parameters (safe from SQL injection)
    pub async fn fetch_all_params(&mut self, sql: &str, params: &[QueryValue]) -> Result<Vec<serde_json::Value>> {
        if let Some(inner) = &mut self.inner {
            let results = match inner {
                TransactionInner::SQLite(tx) => {
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
                    let rows = query.fetch_all(&mut **tx).await?;
                    rows.iter().map(crate::utils::sqlite_row_to_json).collect()
                }
                TransactionInner::MySQL(tx) => {
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
                    let rows = query.fetch_all(&mut **tx).await?;
                    rows.iter().map(crate::utils::mysql_row_to_json).collect()
                }
            };
            Ok(results)
        } else {
            Err(crate::error::Error::QueryError(
                "Transaction already completed".to_string(),
            ))
        }
    }

    /// Fetch one row from a query as JSON value (deprecated - use fetch_one_params)
    #[deprecated(note = "Use fetch_one_params for SQL injection protection")]
    pub async fn fetch_one(&mut self, sql: &str) -> Result<Option<serde_json::Value>> {
        if let Some(inner) = &mut self.inner {
            let result = match inner {
                TransactionInner::SQLite(tx) => {
                    let row_opt = sqlx::query(sql).fetch_optional(&mut **tx).await?;
                    row_opt.as_ref().map(crate::utils::sqlite_row_to_json)
                }
                TransactionInner::MySQL(tx) => {
                    let row_opt = sqlx::query(sql).fetch_optional(&mut **tx).await?;
                    row_opt.as_ref().map(crate::utils::mysql_row_to_json)
                }
            };
            Ok(result)
        } else {
            Err(crate::error::Error::QueryError(
                "Transaction already completed".to_string(),
            ))
        }
    }

    /// Fetch one row with parameters (safe from SQL injection)
    pub async fn fetch_one_params(&mut self, sql: &str, params: &[QueryValue]) -> Result<Option<serde_json::Value>> {
        if let Some(inner) = &mut self.inner {
            let result = match inner {
                TransactionInner::SQLite(tx) => {
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
                    let row_opt = query.fetch_optional(&mut **tx).await?;
                    row_opt.as_ref().map(crate::utils::sqlite_row_to_json)
                }
                TransactionInner::MySQL(tx) => {
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
                    let row_opt = query.fetch_optional(&mut **tx).await?;
                    row_opt.as_ref().map(crate::utils::mysql_row_to_json)
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