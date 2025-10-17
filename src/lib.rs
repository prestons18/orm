pub mod backend;
pub mod connection;
pub mod error;
pub mod migration;
pub mod model;
pub mod query;
pub mod schema;
pub mod transaction;
pub mod utils;

pub use error::{Error, Result};

pub mod prelude {
    pub use crate::backend::{Backend, DatabaseBackend};
    pub use crate::connection::{Connection, Database};
    pub use crate::error::{Error, Result};
    pub use crate::model::{FromRow, Model, ModelCrud, ModelQuery, Value};
    pub use crate::query::{JoinType, OrderDirection, QueryBuilder};
    pub use crate::schema::{Column, Table};
    pub use crate::transaction::Transaction;
}