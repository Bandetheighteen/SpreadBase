//! Excel workbook ingestion parser using Calamine

use crate::db::SqlValue;
use calamine::{open_workbook_auto, Data, Reader};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

/// Parsing and schema deduction errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseError {
    IoError(String),
    FormatError(String),
    EmptyFile(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IoError(msg) => write!(f, "I/O Error: {}", msg),
            ParseError::FormatError(msg) => write!(f, "Format Error: {}", msg),
            ParseError::EmptyFile(msg) => write!(f, "Empty File: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err.to_string())
    }
}

/// Represents an individual worksheet parsed into an in-memory relational table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetTable {
    /// Original sheet title in the Excel workbook
    pub sheet_name: String,
    /// Sanitized relational table identifier in DuckDB
    pub table_name: String,
    /// Column headings treated as SQL variables/fields
    pub columns: Vec<String>,
    /// Cell records
    pub rows: Vec<Vec<SqlValue>>,
}

/// Sanitizes arbitrary sheet and file names into clean, safe SQL identifiers.
pub fn sanitize_identifier(raw: &str) -> String {
    let mut sanitized = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            sanitized.push(ch);
        } else if ch == ' ' || ch == '-' || ch == '.' || ch == '/' || ch == '\\' {
            sanitized.push('_');
        }
    }

    if sanitized.is_empty() {
        return "table_data".to_string();
    }

    // Ensure it starts with a letter or underscore
    if !sanitized.chars().next().unwrap().is_alphabetic() && !sanitized.starts_with('_') {
        sanitized = format!("t_{}", sanitized);
    }

    sanitized
}

/// Generates a composite table name from workbook and sheet (e.g. `workbook_sheet`)
pub fn sanitize_table_name(workbook_name: &str, sheet_name: &str) -> String {
    let clean_wb = sanitize_identifier(workbook_name);
    let clean_sh = sanitize_identifier(sheet_name);
    if clean_wb.is_empty() || clean_wb == clean_sh {
        clean_sh
    } else {
        format!("{}_{}", clean_wb, clean_sh)
    }
}

/// Reads an Excel workbook (.xlsx, .xls) and parses all worksheets into `SheetTable`s.
pub fn read_excel_file(path: &Path) -> Result<Vec<SheetTable>, ParseError> {
    let mut workbook = open_workbook_auto(path)
        .map_err(|e| ParseError::FormatError(format!("Failed to open Excel workbook: {}", e)))?;

    let workbook_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("workbook");

    let sheet_names = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err(ParseError::EmptyFile(format!(
            "Excel workbook has no sheets: {:?}",
            path
        )));
    }

    let mut tables = Vec::with_capacity(sheet_names.len());

    for sheet_name in sheet_names {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let (height, width) = range.get_size();
            if height == 0 || width == 0 {
                // Empty sheet
                tables.push(SheetTable {
                    sheet_name: sheet_name.clone(),
                    table_name: sanitize_table_name(workbook_stem, &sheet_name),
                    columns: Vec::new(),
                    rows: Vec::new(),
                });
                continue;
            }

            let mut rows_iter = range.rows();
            
            // First row serves as column headers (variables)
            let header_row = rows_iter.next().unwrap_or(&[]);
            let mut columns = Vec::with_capacity(width);
            let mut col_counts = std::collections::HashMap::new();

            for (idx, cell) in header_row.iter().enumerate() {
                let mut col_name = match cell {
                    Data::String(s) if !s.trim().is_empty() => s.trim().to_string(),
                    Data::Int(i) => i.to_string(),
                    Data::Float(f) => f.to_string(),
                    _ => format!("col_{}", idx + 1),
                };

                // Deduplicate column names if needed
                let count = col_counts.entry(col_name.clone()).or_insert(0);
                *count += 1;
                if *count > 1 {
                    col_name = format!("{}_{}", col_name, count);
                }

                columns.push(col_name);
            }

            // Remaining rows are parsed into SqlValue cells
            let mut data_rows = Vec::with_capacity(height.saturating_sub(1));
            for row in rows_iter {
                let mut row_values = Vec::with_capacity(columns.len());
                let mut all_empty = true;

                for (col_idx, cell) in row.iter().enumerate() {
                    if col_idx >= columns.len() {
                        break;
                    }
                    let val = calamine_cell_to_sql_value(cell);
                    if !val.is_null() {
                        all_empty = false;
                    }
                    row_values.push(val);
                }

                // Fill any missing trailing columns with NULL
                while row_values.len() < columns.len() {
                    row_values.push(SqlValue::Null);
                }

                if !all_empty {
                    data_rows.push(row_values);
                }
            }

            tables.push(SheetTable {
                sheet_name: sheet_name.clone(),
                table_name: sanitize_table_name(workbook_stem, &sheet_name),
                columns,
                rows: data_rows,
            });
        }
    }

    Ok(tables)
}

/// Converts a Calamine cell Data to SqlValue with type reconciliation.
fn calamine_cell_to_sql_value(cell: &Data) -> SqlValue {
    match cell {
        Data::Empty => SqlValue::Null,
        Data::String(s) => SqlValue::Text(s.clone()),
        Data::Float(f) => {
            if f.is_finite() && f.fract() == 0.0 && *f >= (i64::MIN as f64) && *f <= (i64::MAX as f64) {
                SqlValue::BigInt(*f as i64)
            } else {
                SqlValue::Double(*f)
            }
        },
        Data::Int(i) => SqlValue::BigInt(*i),
        Data::Bool(b) => SqlValue::Boolean(*b),
        Data::DateTime(dt) => {
            // Excel dates are numbers representing days since 1899-12-30
            SqlValue::Double(dt.as_f64())
        }
        Data::DateTimeIso(iso) => SqlValue::Text(iso.clone()),
        Data::Error(err) => SqlValue::Text(format!("#ERR:{:?}", err)),
        _ => SqlValue::Text(format!("{:?}", cell)),
    }
}
