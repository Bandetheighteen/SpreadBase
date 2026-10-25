//! Database and SQL types for LocalSQL Studio

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Strongly-typed SQL value representing cells in DuckDB query results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SqlValue {
    Null,
    Boolean(bool),
    TinyInt(i8),
    SmallInt(i16),
    Integer(i32),
    BigInt(i64),
    HugeInt(i128),
    Float(f32),
    Double(f64),
    Text(String),
    Date(NaiveDate),
    Timestamp(NaiveDateTime),
    Blob(Vec<u8>),
}

impl SqlValue {
    /// Returns true if the value is SQL NULL.
    pub fn is_null(&self) -> bool {
        matches!(self, SqlValue::Null)
    }

    /// Converts the value to a string representation for display or export.
    pub fn to_display_string(&self) -> String {
        match self {
            SqlValue::Null => "NULL".to_string(),
            SqlValue::Boolean(b) => b.to_string(),
            SqlValue::TinyInt(v) => v.to_string(),
            SqlValue::SmallInt(v) => v.to_string(),
            SqlValue::Integer(v) => v.to_string(),
            SqlValue::BigInt(v) => v.to_string(),
            SqlValue::HugeInt(v) => v.to_string(),
            SqlValue::Float(v) => format!("{:.4}", v),
            SqlValue::Double(v) => format!("{:.4}", v),
            SqlValue::Text(s) => s.clone(),
            SqlValue::Date(d) => d.format("%Y-%m-%d").to_string(),
            SqlValue::Timestamp(ts) => ts.format("%Y-%m-%d %H:%M:%S").to_string(),
            SqlValue::Blob(b) => format!("<BLOB {} bytes>", b.len()),
        }
    }
}

impl fmt::Display for SqlValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_display_string())
    }
}

impl duckdb::types::ToSql for SqlValue {
    fn to_sql(&self) -> duckdb::Result<duckdb::types::ToSqlOutput<'_>> {
        match self {
            SqlValue::Null => Ok(duckdb::types::ToSqlOutput::Borrowed(duckdb::types::ValueRef::Null)),
            SqlValue::Boolean(b) => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::Boolean(*b))),
            SqlValue::TinyInt(v) => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::TinyInt(*v))),
            SqlValue::SmallInt(v) => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::SmallInt(*v))),
            SqlValue::Integer(v) => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::Int(*v))),
            SqlValue::BigInt(v) => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::BigInt(*v))),
            SqlValue::HugeInt(v) => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::HugeInt(*v))),
            SqlValue::Float(v) => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::Float(*v))),
            SqlValue::Double(v) => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::Double(*v))),
            SqlValue::Text(s) => Ok(duckdb::types::ToSqlOutput::Borrowed(duckdb::types::ValueRef::Text(s.as_bytes()))),
            _ => Ok(duckdb::types::ToSqlOutput::Owned(duckdb::types::Value::Text(self.to_display_string()))),
        }
    }
}

/// Metadata describing a table column in query result sets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnMeta {
    pub name: String,
    pub data_type: String,
}

impl ColumnMeta {
    pub fn new(name: impl Into<String>, data_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data_type: data_type.into(),
        }
    }
}

/// Container for query execution results including column schemas, row records, and timing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<ColumnMeta>,
    pub rows: Vec<Vec<SqlValue>>,
    pub row_count: usize,
    pub execution_time_ms: f64,
}

impl QueryResult {
    pub fn empty(execution_time_ms: f64) -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            row_count: 0,
            execution_time_ms,
        }
    }

    pub fn new(
        columns: Vec<ColumnMeta>,
        rows: Vec<Vec<SqlValue>>,
        execution_time_ms: f64,
    ) -> Self {
        let row_count = rows.len();
        Self {
            columns,
            rows,
            row_count,
            execution_time_ms,
        }
    }
}

/// Errors originating from database operations or conversions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DbError {
    ConnectionError(String),
    QueryError(String),
    ConversionError(String),
    TableNotFound(String),
    IoError(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionError(msg) => write!(f, "Connection Error: {}", msg),
            DbError::QueryError(msg) => write!(f, "Query Error: {}", msg),
            DbError::ConversionError(msg) => write!(f, "Conversion Error: {}", msg),
            DbError::TableNotFound(table) => write!(f, "Table Not Found: {}", table),
            DbError::IoError(msg) => write!(f, "I/O Error: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

impl From<duckdb::Error> for DbError {
    fn from(err: duckdb::Error) -> Self {
        DbError::QueryError(err.to_string())
    }
}

impl From<std::io::Error> for DbError {
    fn from(err: std::io::Error) -> Self {
        DbError::IoError(err.to_string())
    }
}
