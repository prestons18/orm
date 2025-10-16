pub mod builder;
pub mod executor;

use crate::error::Result;
use crate::schema::Column;

pub use executor::{QueryExecutor, QueryValue};

/// Trait for building SQL queries
pub trait QueryBuilder: Send + Sync {
    /// Build a SELECT query
    fn select(&mut self, columns: &[Column]) -> &mut Self;
    
    /// Build a FROM clause
    fn from(&mut self, table: &str) -> &mut Self;
    
    /// Build a WHERE clause
    fn where_clause(&mut self, condition: &str) -> &mut Self;
    
    /// Build an ORDER BY clause
    fn order_by(&mut self, column: &str, direction: OrderDirection) -> &mut Self;
    
    /// Build a LIMIT clause
    fn limit(&mut self, limit: u64) -> &mut Self;
    
    /// Build an OFFSET clause
    fn offset(&mut self, offset: u64) -> &mut Self;
    
    /// Build an INSERT query
    fn insert_into(&mut self, table: &str, columns: &[&str]) -> &mut Self;
    
    /// Add values for INSERT
    fn values(&mut self, values: &[&str]) -> &mut Self;
    
    /// Build an UPDATE query
    fn update(&mut self, table: &str) -> &mut Self;
    
    /// Add SET clause for UPDATE
    fn set(&mut self, column: &str, value: &str) -> &mut Self;
    
    /// Build a DELETE query
    fn delete_from(&mut self, table: &str) -> &mut Self;
    
    /// Add RETURNING clause (SQLite only)
    fn returning(&mut self, columns: &[&str]) -> &mut Self;
    
    /// Build the final SQL string
    fn build(&self) -> Result<String>;
    
    /// Reset the query builder
    fn reset(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl std::fmt::Display for OrderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderDirection::Asc => write!(f, "ASC"),
            OrderDirection::Desc => write!(f, "DESC"),
        }
    }
}