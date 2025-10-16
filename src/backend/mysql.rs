use crate::backend::{Backend, BackendFeature};
use crate::error::Result;
use crate::query::builder::{MySQLQueryBuilder, QueryBuilderEnum};
use async_trait::async_trait;
use sqlx::MySqlPool;

pub struct MySQLBackend {
    pool: MySqlPool,
    connection_url: String,
}

impl MySQLBackend {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = MySqlPool::connect(url).await?;
        Ok(Self {
            pool,
            connection_url: url.to_string(),
        })
    }

    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }
}

#[async_trait]
impl Backend for MySQLBackend {
    fn name(&self) -> &str {
        "MySQL"
    }

    fn connection_url(&self) -> &str {
        &self.connection_url
    }

    fn query_builder(&self) -> QueryBuilderEnum {
        QueryBuilderEnum::MySQL(MySQLQueryBuilder::new())
    }

    async fn execute_raw(&self, sql: &str) -> Result<u64> {
        let result = sqlx::query(sql).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    fn supports_feature(&self, feature: BackendFeature) -> bool {
        match feature {
            BackendFeature::Transactions => true,
            BackendFeature::Savepoints => true,
            BackendFeature::Returning => false, // MySQL 8.0+ only
            BackendFeature::OnConflict => false, // Uses INSERT ... ON DUPLICATE KEY
            BackendFeature::CTE => true,        // MySQL 8.0+
            BackendFeature::Window => true,     // MySQL 8.0+
        }
    }
}