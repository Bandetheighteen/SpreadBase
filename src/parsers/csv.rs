//! CSV file ingestion parser

use crate::db::SqlValue;
use crate::parsers::excel::{sanitize_identifier, ParseError, SheetTable};
use std::path::Path;

/// Reads a CSV file and parses it into a `SheetTable`.
pub fn read_csv_file(path: &Path, table_name_override: Option<&str>) -> Result<SheetTable, ParseError> {
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("csv_data");

    let table_name = match table_name_override {
        Some(name) => sanitize_identifier(name),
        None => sanitize_identifier(file_stem),
    };

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_path(path)
        .map_err(|e| ParseError::FormatError(format!("Failed to open CSV file: {}", e)))?;

    let headers = reader
        .headers()
        .map_err(|e| ParseError::FormatError(format!("Failed to read CSV headers: {}", e)))?;

    let mut columns = Vec::with_capacity(headers.len());
    let mut col_counts = std::collections::HashMap::new();

    for (idx, h) in headers.iter().enumerate() {
        let trimmed = h.trim();
        let mut col_name = if trimmed.is_empty() {
            format!("col_{}", idx + 1)
        } else {
            trimmed.to_string()
        };

        let count = col_counts.entry(col_name.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            col_name = format!("{}_{}", col_name, count);
        }

        columns.push(col_name);
    }

    let mut data_rows = Vec::new();
    for result in reader.records() {
        let record = result.map_err(|e| ParseError::FormatError(format!("CSV row parse error: {}", e)))?;
        let mut row_values = Vec::with_capacity(columns.len());

        for (i, field) in record.iter().enumerate() {
            if i >= columns.len() {
                break;
            }
            row_values.push(parse_csv_cell(field));
        }

        while row_values.len() < columns.len() {
            row_values.push(SqlValue::Null);
        }

        data_rows.push(row_values);
    }

    Ok(SheetTable {
        sheet_name: file_stem.to_string(),
        table_name,
        columns,
        rows: data_rows,
    })
}

/// Infers cell data type from raw string.
fn parse_csv_cell(field: &str) -> SqlValue {
    let trimmed = field.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("null") || trimmed.eq_ignore_ascii_case("na") {
        return SqlValue::Null;
    }

    if let Ok(b) = trimmed.parse::<bool>() {
        return SqlValue::Boolean(b);
    }

    if let Ok(i) = trimmed.parse::<i64>() {
        return SqlValue::BigInt(i);
    }

    if let Ok(f) = trimmed.parse::<f64>() {
        if !f.is_nan() {
            return SqlValue::Double(f);
        }
    }

    SqlValue::Text(trimmed.to_string())
}
