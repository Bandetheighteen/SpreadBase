//! Embedded DuckDB database manager for LocalSQL Studio

use crate::db::types::{ColumnMeta, DbError, QueryResult, SqlValue};
use duckdb::types::ValueRef;
use duckdb::Connection;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Central database manager encapsulating the in-process DuckDB engine.
#[derive(Clone)]
pub struct DbManager {
    conn: Arc<Mutex<Connection>>,
}

impl DbManager {
    /// Creates a new in-memory DuckDB instance.
    pub fn new_in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory().map_err(|e| DbError::ConnectionError(e.to_string()))?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Executes an arbitrary analytical SQL query and returns structured results with timing.
    pub fn execute_query(&self, sql: &str) -> Result<QueryResult, DbError> {
        let start = Instant::now();
        let conn = self.conn.lock().map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        let mut stmt = conn.prepare(sql).map_err(|e| DbError::QueryError(e.to_string()))?;
        let mut rows_iter = stmt.query([]).map_err(|e| DbError::QueryError(e.to_string()))?;

        let (col_count, columns) = if let Some(stmt_ref) = rows_iter.as_ref() {
            let count = stmt_ref.column_count();
            let mut cols = Vec::with_capacity(count);
            for i in 0..count {
                let name = match stmt_ref.column_name(i) {
                    Ok(n) => n.clone(),
                    Err(_) => format!("col_{}", i),
                };
                cols.push(ColumnMeta::new(name, "UNKNOWN"));
            }
            (count, cols)
        } else {
            (0, Vec::new())
        };

        let mut rows = Vec::new();
        while let Some(row) = rows_iter.next().map_err(|e| DbError::QueryError(e.to_string()))? {
            let mut record = Vec::with_capacity(col_count);
            for i in 0..col_count {
                let val_ref = row.get_ref(i).map_err(|e| DbError::ConversionError(e.to_string()))?;
                record.push(Self::value_ref_to_sql_value(val_ref));
            }
            rows.push(record);
        }

        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        Ok(QueryResult::new(columns, rows, elapsed_ms))
    }

    /// Executes a DDL or DML statement (e.g. `CREATE TABLE`, `INSERT`) and returns rows affected.
    pub fn execute_statement(&self, sql: &str) -> Result<usize, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError::ConnectionError(e.to_string()))?;
        let rows_affected = conn.execute(sql, []).map_err(|e| DbError::QueryError(e.to_string()))?;
        Ok(rows_affected)
    }

    /// Converts a DuckDB ValueRef into our strongly-typed SqlValue.
    fn value_ref_to_sql_value(val_ref: ValueRef) -> SqlValue {
        match val_ref {
            ValueRef::Null => SqlValue::Null,
            ValueRef::Boolean(b) => SqlValue::Boolean(b),
            ValueRef::TinyInt(v) => SqlValue::TinyInt(v),
            ValueRef::SmallInt(v) => SqlValue::SmallInt(v),
            ValueRef::Int(v) => SqlValue::Integer(v),
            ValueRef::BigInt(v) => SqlValue::BigInt(v),
            ValueRef::HugeInt(v) => SqlValue::HugeInt(v),
            ValueRef::Float(v) => SqlValue::Float(v),
            ValueRef::Double(v) => SqlValue::Double(v),
            ValueRef::Text(bytes) => {
                let s = String::from_utf8_lossy(bytes).into_owned();
                SqlValue::Text(s)
            }
            ValueRef::Blob(bytes) => SqlValue::Blob(bytes.to_vec()),
            ValueRef::Date32(days) => {
                // days since Unix epoch (1970-01-01)
                chrono::NaiveDate::from_num_days_from_ce_opt(days + 719163)
                    .map(SqlValue::Date)
                    .unwrap_or_else(|| SqlValue::Text(format!("Date32({})", days)))
            }
            ValueRef::Timestamp(unit, val) => {
                // Microsecond/millisecond timestamps
                let secs = match unit {
                    duckdb::types::TimeUnit::Second => val,
                    duckdb::types::TimeUnit::Millisecond => val / 1_000,
                    duckdb::types::TimeUnit::Microsecond => val / 1_000_000,
                    duckdb::types::TimeUnit::Nanosecond => val / 1_000_000_000,
                };
                chrono::DateTime::from_timestamp(secs, 0)
                    .map(|dt| SqlValue::Timestamp(dt.naive_utc()))
                    .unwrap_or_else(|| SqlValue::Text(format!("Timestamp({}, {:?})", val, unit)))
            }
            _ => SqlValue::Text(format!("{:?}", val_ref)),
        }
    }
}
