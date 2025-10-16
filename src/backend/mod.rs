pub mod mysql;
pub mod sqlite;

use crate::error::Result;
use crate::query::builder::QueryBuilderEnum;
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

    /// Execute raw SQL
    async fn execute_raw(&self, sql: &str) -> Result<u64>;

    /// Check if the backend supports a specific feature
    fn supports_feature(&self, feature: BackendFeature) -> bool;
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
        if url.starts_with("sqlite://") {
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