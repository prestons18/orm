pub mod mysql;
pub mod sqlite;

use crate::error::Result;
use crate::query::builder::{Dialect, QueryBuilderEnum};
use crate::query::QueryValue;
use async_trait::async_trait;

/// Trait representing a database backend
#[async_trait]
pub trait Backend: Send + Sync + 'static {
    /// Get the backend name
    fn name(&self) -> &str;

    /// Get the connection URL
    fn connection_url(&self) -> &str;

    /// Create a query builder for this backend
    fn query_builder(&self) -> QueryBuilderEnum;

    /// Execute raw SQL (DEPRECATED - vulnerable to SQL injection, use execute instead)
    #[deprecated(note = "Use execute() with parameters for SQL injection protection")]
    async fn execute_raw(&self, sql: &str) -> Result<u64>;

    /// Execute SQL with parameters (safe from SQL injection)
    async fn execute(&self, sql: &str, params: &[QueryValue]) -> Result<u64>;

    /// Fetch all rows from a query as JSON values (DEPRECATED - vulnerable to SQL injection, use fetch_all_params)
    #[deprecated(note = "Use fetch_all_params() with parameters for SQL injection protection")]
    async fn fetch_all(&self, sql: &str) -> Result<Vec<serde_json::Value>>;

    /// Fetch all rows with parameters (safe from SQL injection)
    async fn fetch_all_params(&self, sql: &str, params: &[QueryValue]) -> Result<Vec<serde_json::Value>>;

    /// Fetch one row from a query as JSON value (DEPRECATED - vulnerable to SQL injection, use fetch_one_params)
    #[deprecated(note = "Use fetch_one_params() with parameters for SQL injection protection")]
    async fn fetch_one(&self, sql: &str) -> Result<Option<serde_json::Value>>;

    /// Fetch one row with parameters (safe from SQL injection)
    async fn fetch_one_params(&self, sql: &str, params: &[QueryValue]) -> Result<Option<serde_json::Value>>;

    /// Begin a new transaction
    async fn begin_transaction(&self) -> Result<crate::transaction::Transaction>;

    /// Check if the backend supports a specific feature
    fn supports_feature(&self, feature: BackendFeature) -> bool;
}

/// Generic backend for code reduction
pub struct GenericBackend<P> {
    pool: P,
    connection_url: String,
    dialect: Dialect,
    name: &'static str,
}

impl<P> GenericBackend<P> {
    pub fn new(pool: P, connection_url: String, dialect: Dialect, name: &'static str) -> Self {
        Self {
            pool,
            connection_url,
            dialect,
            name,
        }
    }

    pub fn pool(&self) -> &P {
        &self.pool
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendFeature {
    Transactions,
    Savepoints,
    Returning,
    OnConflict,
    CTE,
    Window,
}

/// Enum for selecting database backend
#[derive(Debug, Clone)]
pub enum DatabaseBackend {
    SQLite,
    MySQL,
}

impl DatabaseBackend {
    pub fn from_url(url: &str) -> Result<Self> {
        if url.starts_with("sqlite:") {
            Ok(DatabaseBackend::SQLite)
        } else if url.starts_with("mysql://") {
            Ok(DatabaseBackend::MySQL)
        } else {
            Err(crate::error::Error::ConfigError(
                "Unsupported database URL scheme".to_string(),
            ))
        }
    }
}