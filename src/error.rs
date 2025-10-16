use thiserror::Error;

/// Core error type for the ORM
#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;