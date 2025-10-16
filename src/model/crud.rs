use crate::backend::Backend;
use crate::error::{Error, Result};
use crate::model::{FromRow, Model, Value};
use crate::query::builder::QueryBuilderEnum;
use crate::query::{JoinType, OrderDirection, QueryBuilder};
use crate::schema::{Column, ColumnType};
use async_trait::async_trait;

/// Query builder helper for models
pub struct ModelQuery<'a, T: Model> {
    builder: QueryBuilderEnum,
    backend: &'a dyn Backend,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Model + FromRow> ModelQuery<'a, T> {
    /// Create a new query for a model
    pub fn new(backend: &'a dyn Backend) -> Self {
        let builder = backend.query_builder();
        Self {
            builder,
            backend,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Add a WHERE clause
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.builder.where_clause(condition);
        self
    }

    /// Add a WHERE clause with parameter (safe from SQL injection)
    pub fn where_eq(mut self, column: &str, value: crate::query::QueryValue) -> Self {
        self.builder.where_eq(column, value);
        self
    }

    /// Add an ORDER BY clause
    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.builder.order_by(column, direction);
        self
    }

    /// Add a LIMIT clause
    pub fn limit(mut self, limit: u64) -> Self {
        self.builder.limit(limit);
        self
    }

    /// Add an OFFSET clause
    pub fn offset(mut self, offset: u64) -> Self {
        self.builder.offset(offset);
        self
    }

    /// Add a JOIN clause
    pub fn join(mut self, table: &str, on: &str, join_type: JoinType) -> Self {
        self.builder.join(table, on, join_type);
        self
    }

    /// Add an INNER JOIN clause
    pub fn inner_join(mut self, table: &str, on: &str) -> Self {
        self.builder.inner_join(table, on);
        self
    }

    /// Add a LEFT JOIN clause
    pub fn left_join(mut self, table: &str, on: &str) -> Self {
        self.builder.left_join(table, on);
        self
    }

    /// Add a GROUP BY clause
    pub fn group_by(mut self, columns: &[&str]) -> Self {
        self.builder.group_by(columns);
        self
    }

    /// Add a HAVING clause
    pub fn having(mut self, condition: &str) -> Self {
        self.builder.having(condition);
        self
    }

    /// Add DISTINCT
    pub fn distinct(mut self) -> Self {
        self.builder.distinct();
        self
    }

    /// Build and return the SQL query
    pub fn to_sql(&self) -> Result<String> {
        self.builder.build()
    }

    /// Execute the query and return all results
    pub async fn get(self) -> Result<Vec<T>> {
        let sql = self.builder.build()?;
        let params = self.builder.params();
        let json_rows = self.backend.fetch_all_params(&sql, params).await?;
        
        json_rows
            .iter()
            .map(|json| T::from_json(json))
            .collect()
    }

    /// Execute the query and return first result
    pub async fn first(self) -> Result<Option<T>> {
        let sql = self.builder.build()?;
        let params = self.builder.params();
        let json_row = self.backend.fetch_one_params(&sql, params).await?;
        
        match json_row {
            Some(json) => Ok(Some(T::from_json(&json)?)),
            None => Ok(None),
        }
    }
}

/// CRUD operations for models
#[async_trait]
pub trait ModelCrud: Model + FromRow {
    /// Start a query builder for this model
    fn query(backend: &dyn Backend) -> ModelQuery<'_, Self> {
        let mut query = ModelQuery::new(backend);
        let columns: Vec<Column> = Self::all_columns()
            .iter()
            .map(|name| Column::new(*name, ColumnType::Text))
            .collect();
        query.builder.select(&columns);
        query.builder.from(Self::table_name());
        query
    }
    /// Find a record by primary key
    async fn find(backend: &dyn Backend, id: Value) -> Result<Option<Self>> {
        let mut query = Self::query(backend);
        query.builder.where_eq(Self::primary_key(), id.to_query_value());
        query.builder.limit(1);
        query.first().await
    }

    /// Find all records
    async fn all(backend: &dyn Backend) -> Result<Vec<Self>> {
        Self::query(backend).get().await
    }

    /// Find records matching a condition
    async fn where_clause(backend: &dyn Backend, condition: &str) -> Result<Vec<Self>> {
        Self::query(backend)
            .where_clause(condition)
            .get()
            .await
    }

    /// Find records with ordering
    async fn order_by(backend: &dyn Backend, column: &str, direction: OrderDirection) -> Result<Vec<Self>> {
        Self::query(backend)
            .order_by(column, direction)
            .get()
            .await
    }

    /// Find records with limit
    async fn take(backend: &dyn Backend, limit: u64) -> Result<Vec<Self>> {
        Self::query(backend)
            .limit(limit)
            .get()
            .await
    }

    /// Find first record
    async fn first(backend: &dyn Backend) -> Result<Option<Self>> {
        Self::query(backend)
            .limit(1)
            .first()
            .await
    }

    /// Create a new record
    async fn create(backend: &dyn Backend, values: &Self) -> Result<Self> {
        let mut builder = backend.query_builder();
        let data = values.to_values();
        
        let columns: Vec<&str> = data.keys().map(|s| s.as_str()).collect();
        let query_values: Vec<crate::query::QueryValue> = data.values().map(|v| v.to_query_value()).collect();

        // Try using RETURNING if supported (SQLite)
        if backend.supports_feature(crate::backend::BackendFeature::Returning) {
            let all_cols: Vec<&str> = Self::all_columns();
            let sql = builder
                .insert_into(Self::table_name(), &columns)
                .values_params(&query_values)
                .returning(&all_cols)
                .build()?;
            
            let params = builder.params();
            let result = backend.fetch_one_params(&sql, params).await?;
            match result {
                Some(json) => Self::from_json(&json),
                None => Err(Error::QueryError("Failed to create record".to_string())),
            }
        } else {
            // For MySQL: execute insert, then fetch by primary key
            let sql = builder
                .insert_into(Self::table_name(), &columns)
                .values_params(&query_values)
                .build()?;

            let params = builder.params();
            backend.execute(&sql, params).await?;

            // If the model has a primary key value, fetch it back
            if let Some(pk_value) = values.primary_key_value() {
                Self::find(backend, pk_value).await?
                    .ok_or_else(|| Error::QueryError("Failed to fetch created record".to_string()))
            } else {
                // For auto-increment IDs, we'd need LAST_INSERT_ID() - not implemented yet
                Err(Error::QueryError("Auto-increment ID retrieval not yet implemented for MySQL".to_string()))
            }
        }
    }

    /// Update a record
    async fn update(&self, backend: &dyn Backend) -> Result<()> {
        let pk_value = self.primary_key_value().ok_or_else(|| {
            Error::QueryError("Cannot update record without primary key".to_string())
        })?;

        let mut builder = backend.query_builder();
        let data = self.to_values();

        builder.update(Self::table_name());
        
        for (col, val) in data.iter() {
            if col != Self::primary_key() {
                builder.set_param(col, val.to_query_value());
            }
        }

        builder.where_eq(Self::primary_key(), pk_value.to_query_value());
        let sql = builder.build()?;
        let params = builder.params();

        backend.execute(&sql, params).await?;
        Ok(())
    }

    /// Delete a record
    async fn delete(&self, backend: &dyn Backend) -> Result<()> {
        let pk_value = self.primary_key_value().ok_or_else(|| {
            Error::QueryError("Cannot delete record without primary key".to_string())
        })?;

        let mut builder = backend.query_builder();
        builder.delete_from(Self::table_name());
        builder.where_eq(Self::primary_key(), pk_value.to_query_value());
        let sql = builder.build()?;
        let params = builder.params();

        backend.execute(&sql, params).await?;
        Ok(())
    }

    /// Delete records by condition
    async fn delete_where(backend: &dyn Backend, condition: &str) -> Result<u64> {
        let mut builder = backend.query_builder();
        let sql = builder
            .delete_from(Self::table_name())
            .where_clause(condition)
            .build()?;

        backend.execute_raw(&sql).await
    }

    /// Count all records
    async fn count(backend: &dyn Backend) -> Result<i64> {
        let mut builder = backend.query_builder();
        let count_col = Column::new("COUNT(*) as count", ColumnType::BigInteger);
        
        let sql = builder
            .select(&[count_col])
            .from(Self::table_name())
            .build()?;

        let result = backend.fetch_one(&sql).await?;
        match result {
            Some(json) => {
                let count = json.get("count")
                    .and_then(|v| v.as_i64())
                    .ok_or_else(|| Error::QueryError("Failed to parse count result".to_string()))?;
                Ok(count)
            }
            None => Ok(0),
        }
    }
}
