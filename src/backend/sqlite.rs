use crate::backend::{Backend, BackendFeature};
use crate::error::Result;
use crate::query::builder::{QueryBuilderEnum, SQLiteQueryBuilder};
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SQLiteBackend {
    pool: SqlitePool,
    connection_url: String,
}

impl SQLiteBackend {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(url).await?;
        Ok(Self {
            pool,
            connection_url: url.to_string(),
        })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

#[async_trait]
impl Backend for SQLiteBackend {
    fn name(&self) -> &str {
        "SQLite"
    }

    fn connection_url(&self) -> &str {
        &self.connection_url
    }

    fn query_builder(&self) -> QueryBuilderEnum {
        QueryBuilderEnum::SQLite(SQLiteQueryBuilder::new())
    }

    async fn execute_raw(&self, sql: &str) -> Result<u64> {
        let result = sqlx::query(sql).execute(&self.pool).await?;
        Ok(result.rows_affected())
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