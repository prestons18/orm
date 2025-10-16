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
    
    /// Build a WHERE clause (DEPRECATED - vulnerable to SQL injection, use where_eq)
    #[deprecated(note = "Use where_eq() with parameters for SQL injection protection")]
    fn where_clause(&mut self, condition: &str) -> &mut Self;
    
    /// Add a WHERE clause with a parameter (safe from SQL injection)
    fn where_eq(&mut self, column: &str, value: QueryValue) -> &mut Self;
    
    /// Build an ORDER BY clause
    fn order_by(&mut self, column: &str, direction: OrderDirection) -> &mut Self;
    
    /// Build a LIMIT clause
    fn limit(&mut self, limit: u64) -> &mut Self;
    
    /// Build an OFFSET clause
    fn offset(&mut self, offset: u64) -> &mut Self;
    
    /// Build an INSERT query
    fn insert_into(&mut self, table: &str, columns: &[&str]) -> &mut Self;
    
    /// Add values for INSERT (DEPRECATED - vulnerable to SQL injection, use values_params)
    #[deprecated(note = "Use values_params() with parameters for SQL injection protection")]
    fn values(&mut self, values: &[&str]) -> &mut Self;
    
    /// Add parameterized values for INSERT (safe from SQL injection)
    fn values_params(&mut self, values: &[QueryValue]) -> &mut Self;
    
    /// Build an UPDATE query
    fn update(&mut self, table: &str) -> &mut Self;
    
    /// Add SET clause for UPDATE (DEPRECATED - vulnerable to SQL injection, use set_param)
    #[deprecated(note = "Use set_param() with parameters for SQL injection protection")]
    fn set(&mut self, column: &str, value: &str) -> &mut Self;
    
    /// Add SET clause with parameter (safe from SQL injection)
    fn set_param(&mut self, column: &str, value: QueryValue) -> &mut Self;
    
    /// Build a DELETE query
    fn delete_from(&mut self, table: &str) -> &mut Self;
    
    /// Add RETURNING clause (SQLite only)
    fn returning(&mut self, columns: &[&str]) -> &mut Self;
    
    /// Add JOIN clause
    fn join(&mut self, table: &str, on: &str, join_type: JoinType) -> &mut Self;
    
    /// Add INNER JOIN clause
    fn inner_join(&mut self, table: &str, on: &str) -> &mut Self {
        self.join(table, on, JoinType::Inner)
    }
    
    /// Add LEFT JOIN clause
    fn left_join(&mut self, table: &str, on: &str) -> &mut Self {
        self.join(table, on, JoinType::Left)
    }
    
    /// Add RIGHT JOIN clause
    fn right_join(&mut self, table: &str, on: &str) -> &mut Self {
        self.join(table, on, JoinType::Right)
    }
    
    /// Add GROUP BY clause
    fn group_by(&mut self, columns: &[&str]) -> &mut Self;
    
    /// Add HAVING clause
    fn having(&mut self, condition: &str) -> &mut Self;
    
    /// Add DISTINCT
    fn distinct(&mut self) -> &mut Self;
    
    /// Build the final SQL string
    fn build(&self) -> Result<String>;
    
    /// Get the query parameters
    fn params(&self) -> &[QueryValue];
    
    /// Reset the query builder
    fn reset(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
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