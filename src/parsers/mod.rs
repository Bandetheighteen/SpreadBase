//! Data ingestion parsers — Excel workbook and CSV file reading.

pub mod csv;
pub mod excel;

pub use csv::read_csv_file;
pub use excel::{read_excel_file, sanitize_identifier, sanitize_table_name, ParseError, SheetTable};
