//! Embedded DuckDB database manager for LocalSQL Studio

use crate::db::types::{ColumnMeta, DbError, QueryResult, SqlValue};
use crate::parsers::{read_csv_file, read_excel_file, SheetTable};
use duckdb::types::ValueRef;
use duckdb::Connection;
use std::path::Path;
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

    /// Registers a `SheetTable` into DuckDB as an independent relational table.
    pub fn register_sheet_table(&self, sheet: &SheetTable) -> Result<(), DbError> {
        if sheet.columns.is_empty() {
            return Ok(());
        }

        // Determine column types based on row values
        let mut col_defs = Vec::with_capacity(sheet.columns.len());
        for (col_idx, col_name) in sheet.columns.iter().enumerate() {
            let col_type = infer_column_sql_type(&sheet.rows, col_idx);
            let sanitized_col = col_name.replace('"', "\"\"");
            col_defs.push(format!("\"{}\" {}", sanitized_col, col_type));
        }

        let table_ident = sheet.table_name.replace('"', "\"\"");
        let create_table_sql = format!(
            "CREATE OR REPLACE TABLE \"{}\" ({})",
            table_ident,
            col_defs.join(", ")
        );

        let mut conn = self.conn.lock().map_err(|e| DbError::ConnectionError(e.to_string()))?;
        conn.execute(&create_table_sql, [])
            .map_err(|e| DbError::QueryError(format!("Failed to create table {}: {}", sheet.table_name, e)))?;

        if !sheet.rows.is_empty() {
            let placeholders = vec!["?"; sheet.columns.len()].join(", ");
            let insert_sql = format!("INSERT INTO \"{}\" VALUES ({})", table_ident, placeholders);

            let tx = conn.transaction().map_err(|e| DbError::QueryError(e.to_string()))?;
            {
                let mut stmt = tx.prepare(&insert_sql).map_err(|e| DbError::QueryError(e.to_string()))?;
                for row in &sheet.rows {
                    stmt.execute(duckdb::params_from_iter(row.iter()))
                        .map_err(|e| DbError::QueryError(format!("Failed to insert row: {}", e)))?;
                }
            }
            tx.commit().map_err(|e| DbError::QueryError(e.to_string()))?;
        }

        Ok(())
    }

    /// Ingests an Excel file (.xlsx, .xls) and exposes all sheets as relational tables.
    pub fn register_excel_file(&self, path: &Path) -> Result<Vec<String>, DbError> {
        let tables = read_excel_file(path).map_err(|e| DbError::IoError(e.to_string()))?;
        let mut table_names = Vec::with_capacity(tables.len());

        for sheet in &tables {
            self.register_sheet_table(sheet)?;
            table_names.push(sheet.table_name.clone());
        }

        Ok(table_names)
    }

    /// Ingests a CSV file and registers it as a relational table.
    pub fn register_csv_file(&self, path: &Path, table_name: Option<&str>) -> Result<String, DbError> {
        let table = read_csv_file(path, table_name).map_err(|e| DbError::IoError(e.to_string()))?;
        let name = table.table_name.clone();
        self.register_sheet_table(&table)?;
        Ok(name)
    }

    /// Lists all user-created tables currently queryable in the in-memory database.
    pub fn list_tables(&self) -> Result<Vec<String>, DbError> {
        let query = "SELECT table_name FROM information_schema.tables WHERE table_schema = 'main' ORDER BY table_name";
        let res = self.execute_query(query)?;
        let mut tables = Vec::with_capacity(res.row_count);
        for row in res.rows {
            if let Some(SqlValue::Text(name)) = row.first() {
                tables.push(name.clone());
            }
        }
        Ok(tables)
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
            ValueRef::Decimal(d) => {
                let scale = d.scale() as i32;
                let val = d.value() as f64;
                SqlValue::Double(val / 10f64.powi(scale))
            }
            ValueRef::Text(bytes) => {
                let s = String::from_utf8_lossy(bytes).into_owned();
                SqlValue::Text(s)
            }
            ValueRef::Blob(bytes) => SqlValue::Blob(bytes.to_vec()),
            ValueRef::Date32(days) => {
                chrono::NaiveDate::from_num_days_from_ce_opt(days + 719163)
                    .map(SqlValue::Date)
                    .unwrap_or_else(|| SqlValue::Text(format!("Date32({})", days)))
            }
            ValueRef::Timestamp(unit, val) => {
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

/// Infers the best DuckDB SQL column type for a set of rows.
fn infer_column_sql_type(rows: &[Vec<SqlValue>], col_idx: usize) -> &'static str {
    let mut has_int = false;
    let mut has_float = false;
    let mut has_bool = false;
    let mut has_text = false;

    for row in rows {
        if let Some(val) = row.get(col_idx) {
            match val {
                SqlValue::Null => {}
                SqlValue::Boolean(_) => has_bool = true,
                SqlValue::TinyInt(_) | SqlValue::SmallInt(_) | SqlValue::Integer(_) | SqlValue::BigInt(_) | SqlValue::HugeInt(_) => {
                    has_int = true;
                }
                SqlValue::Float(_) | SqlValue::Double(_) => has_float = true,
                SqlValue::Text(_) | SqlValue::Date(_) | SqlValue::Timestamp(_) | SqlValue::Blob(_) => {
                    has_text = true;
                }
            }
        }
    }

    if has_text {
        "VARCHAR"
    } else if has_float || (has_int && has_float) {
        "DOUBLE"
    } else if has_int {
        "BIGINT"
    } else if has_bool {
        "BOOLEAN"
    } else {
        "VARCHAR"
    }
}
