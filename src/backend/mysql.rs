use crate::backend::{Backend, BackendFeature, GenericBackend};
use crate::error::Result;
use crate::query::builder::{Dialect, QueryBuilderEnum};
use async_trait::async_trait;
use sqlx::MySqlPool;

pub type MySQLBackend = GenericBackend<MySqlPool>;

impl MySQLBackend {
    pub async fn connect(url: &str) -> Result<Self> {
        let pool = MySqlPool::connect(url).await?;
        Ok(GenericBackend::new(
            pool,
            url.to_string(),
            Dialect::MySQL,
            "MySQL",
        ))
    }
}

#[async_trait]
impl Backend for MySQLBackend {
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