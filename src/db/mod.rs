//! Database management module — embedded DuckDB engine, types, and query execution.

pub mod manager;
pub mod types;

pub use manager::DbManager;
pub use types::{ColumnMeta, DbError, QueryResult, SqlValue};
