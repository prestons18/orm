pub mod pool;

use crate::backend::{Backend, DatabaseBackend};
use crate::backend::{mysql::MySQLBackend, sqlite::SQLiteBackend};
use crate::error::Result;
use crate::transaction::Transaction;
use async_trait::async_trait;

#[async_trait]
pub trait Connection: Send + Sync {
    /// Begin a new transaction
    async fn begin_transaction(&self) -> Result<Transaction>;

    /// Execute a raw SQL query
    async fn execute(&self, sql: &str) -> Result<u64>;

    /// Check if the connection is healthy
    async fn ping(&self) -> Result<()>;
}

/// Main database connection handle
pub struct Database {
    backend: Box<dyn Backend>,
}

impl Database {
    /// Connect to a database using a connection URL
    pub async fn connect(url: &str) -> Result<Self> {
        let backend_type = DatabaseBackend::from_url(url)?;

        let backend: Box<dyn Backend> = match backend_type {
            DatabaseBackend::SQLite => Box::new(SQLiteBackend::connect(url).await?),
            DatabaseBackend::MySQL => Box::new(MySQLBackend::connect(url).await?),
        };

        Ok(Self { backend })
    }

    /// Get a reference to the backend
    pub fn backend(&self) -> &dyn Backend {
        self.backend.as_ref()
    }

    /// Execute raw SQL
    pub async fn execute(&self, sql: &str) -> Result<u64> {
        self.backend.execute_raw(sql).await
    }

    /// Begin a new transaction
    pub async fn begin_transaction(&self) -> Result<Transaction> {
        self.backend.begin_transaction().await
    }
}