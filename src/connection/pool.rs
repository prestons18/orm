use crate::error::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_connections: usize,
    pub min_connections: usize,
    pub connection_timeout: std::time::Duration,
    pub idle_timeout: Option<std::time::Duration>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            connection_timeout: std::time::Duration::from_secs(30),
            idle_timeout: Some(std::time::Duration::from_secs(600)),
        }
    }
}

/// Connection pool
pub struct Pool {
    semaphore: Arc<Semaphore>,
    config: PoolConfig,
}

impl Pool {
    pub fn new(config: PoolConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_connections));
        Self { semaphore, config }
    }

    pub async fn acquire(&self) -> Result<PoolConnection> {
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| crate::error::Error::ConnectionError(e.to_string()))?;

        Ok(PoolConnection { _permit: permit })
    }

    pub fn config(&self) -> &PoolConfig {
        &self.config
    }
}

pub struct PoolConnection {
    _permit: tokio::sync::OwnedSemaphorePermit,
}