//! Multi-format result export — CSV, Excel (.xlsx), and JSON output.

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use rust_xlsxwriter::{Format, Workbook};
use serde_json::{json, Map, Value};

use crate::db::types::{QueryResult, SqlValue};

/// Errors that can occur during file export.
#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Excel(String),
    Csv(String),
    Json(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::Io(e) => write!(f, "I/O error: {}", e),
            ExportError::Excel(e) => write!(f, "Excel export error: {}", e),
            ExportError::Csv(e) => write!(f, "CSV export error: {}", e),
            ExportError::Json(e) => write!(f, "JSON export error: {}", e),
        }
    }
}

impl std::error::Error for ExportError {}

impl From<std::io::Error> for ExportError {
    fn from(e: std::io::Error) -> Self {
        ExportError::Io(e)
    }
}

/// Exports query results to a CSV file.
pub fn export_to_csv(result: &QueryResult, path: &Path) -> Result<(), ExportError> {
    let mut wtr = csv::WriterBuilder::new()
        .from_path(path)
        .map_err(|e| ExportError::Csv(e.to_string()))?;

    // Header row
    let headers: Vec<&str> = result.columns.iter().map(|c| c.name.as_str()).collect();
    wtr.write_record(&headers)
        .map_err(|e| ExportError::Csv(e.to_string()))?;

    // Data rows
    for row in &result.rows {
        let row_values: Vec<String> = row
            .iter()
            .map(|cell| match cell {
                SqlValue::Null => String::new(),
                _ => cell.to_display_string(),
            })
            .collect();
        wtr.write_record(&row_values)
            .map_err(|e| ExportError::Csv(e.to_string()))?;
    }

    wtr.flush().map_err(|e| ExportError::Csv(e.to_string()))?;
    Ok(())
}

/// Exports query results to a Microsoft Excel (.xlsx) workbook.
pub fn export_to_excel(
    result: &QueryResult,
    path: &Path,
    sheet_name: Option<&str>,
) -> Result<(), ExportError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let name = sheet_name.unwrap_or("Query Results");
    worksheet
        .set_name(name)
        .map_err(|e| ExportError::Excel(e.to_string()))?;

    // Header formatting
    let header_format = Format::new().set_bold();

    // Write column headers
    for (col_idx, col) in result.columns.iter().enumerate() {
        worksheet
            .write_string_with_format(0, col_idx as u16, &col.name, &header_format)
            .map_err(|e| ExportError::Excel(e.to_string()))?;
    }

    // Write rows
    for (row_idx, row) in result.rows.iter().enumerate() {
        let r = (row_idx + 1) as u32;
        for (col_idx, cell) in row.iter().enumerate() {
            let c = col_idx as u16;
            match cell {
                SqlValue::Null => {
                    // Leave blank cell
                }
                SqlValue::Boolean(b) => {
                    worksheet
                        .write_boolean(r, c, *b)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::TinyInt(n) => {
                    worksheet
                        .write_number(r, c, *n as f64)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::SmallInt(n) => {
                    worksheet
                        .write_number(r, c, *n as f64)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::Integer(n) => {
                    worksheet
                        .write_number(r, c, *n as f64)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::BigInt(n) => {
                    worksheet
                        .write_number(r, c, *n as f64)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::HugeInt(n) => {
                    worksheet
                        .write_number(r, c, *n as f64)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::Float(f) => {
                    worksheet
                        .write_number(r, c, *f as f64)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::Double(d) => {
                    worksheet
                        .write_number(r, c, *d)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::Text(s) => {
                    worksheet
                        .write_string(r, c, s)
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::Date(d) => {
                    worksheet
                        .write_string(r, c, d.format("%Y-%m-%d").to_string())
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::Timestamp(ts) => {
                    worksheet
                        .write_string(r, c, ts.format("%Y-%m-%d %H:%M:%S").to_string())
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
                SqlValue::Blob(bytes) => {
                    worksheet
                        .write_string(r, c, format!("<Blob {} bytes>", bytes.len()))
                        .map_err(|e| ExportError::Excel(e.to_string()))?;
                }
            }
        }
    }

    workbook
        .save(path)
        .map_err(|e| ExportError::Excel(e.to_string()))?;
    Ok(())
}

/// Exports query results to a formatted JSON file.
pub fn export_to_json(result: &QueryResult, path: &Path) -> Result<(), ExportError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    let mut json_rows = Vec::with_capacity(result.row_count);
    for row in &result.rows {
        let mut obj = Map::new();
        for (col_idx, col) in result.columns.iter().enumerate() {
            let val = match row.get(col_idx) {
                Some(SqlValue::Null) | None => Value::Null,
                Some(SqlValue::Boolean(b)) => Value::Bool(*b),
                Some(SqlValue::TinyInt(n)) => json!(*n),
                Some(SqlValue::SmallInt(n)) => json!(*n),
                Some(SqlValue::Integer(n)) => json!(*n),
                Some(SqlValue::BigInt(n)) => json!(*n),
                Some(SqlValue::HugeInt(n)) => json!(*n),
                Some(SqlValue::Float(f)) => json!(*f),
                Some(SqlValue::Double(d)) => json!(*d),
                Some(SqlValue::Text(s)) => Value::String(s.clone()),
                Some(SqlValue::Date(d)) => Value::String(d.format("%Y-%m-%d").to_string()),
                Some(SqlValue::Timestamp(ts)) => {
                    Value::String(ts.format("%Y-%m-%d %H:%M:%S").to_string())
                }
                Some(SqlValue::Blob(bytes)) => Value::String(format!("<Blob {} bytes>", bytes.len())),
            };
            obj.insert(col.name.clone(), val);
        }
        json_rows.push(Value::Object(obj));
    }

    serde_json::to_writer_pretty(writer, &json_rows)
        .map_err(|e| ExportError::Json(e.to_string()))?;
    Ok(())
}
