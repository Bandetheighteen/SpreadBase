use std::fs::File;
use std::io::BufReader;

use calamine::{open_workbook_auto, DataType, Reader};
use chrono::NaiveDate;
use tempfile::NamedTempFile;

use localsql_studio::db::types::{ColumnMeta, QueryResult, SqlValue};
use localsql_studio::export::{export_to_csv, export_to_excel, export_to_json};

fn create_sample_query_result() -> QueryResult {
    let columns = vec![
        ColumnMeta {
            name: "id".to_string(),
            data_type: "BIGINT".to_string(),
        },
        ColumnMeta {
            name: "product".to_string(),
            data_type: "VARCHAR".to_string(),
        },
        ColumnMeta {
            name: "price".to_string(),
            data_type: "DOUBLE".to_string(),
        },
        ColumnMeta {
            name: "in_stock".to_string(),
            data_type: "BOOLEAN".to_string(),
        },
        ColumnMeta {
            name: "release_date".to_string(),
            data_type: "DATE".to_string(),
        },
    ];

    let rows = vec![
        vec![
            SqlValue::BigInt(1001),
            SqlValue::Text("Gaming Laptop".to_string()),
            SqlValue::Double(1299.99),
            SqlValue::Boolean(true),
            SqlValue::Date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
        ],
        vec![
            SqlValue::BigInt(1002),
            SqlValue::Null,
            SqlValue::Double(49.50),
            SqlValue::Boolean(false),
            SqlValue::Null,
        ],
        vec![
            SqlValue::BigInt(1003),
            SqlValue::Text("Wireless Mouse".to_string()),
            SqlValue::Null,
            SqlValue::Boolean(true),
            SqlValue::Date(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()),
        ],
    ];

    QueryResult {
        columns,
        rows,
        row_count: 3,
        execution_time_ms: 1.25,
    }
}

#[test]
fn test_export_to_csv_roundtrip() {
    let result = create_sample_query_result();

    let temp = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();
    let path = temp.path().to_path_buf();

    export_to_csv(&result, &path).expect("Failed to export to CSV");

    // Read back with csv crate
    let mut rdr = csv::Reader::from_path(&path).unwrap();
    let headers = rdr.headers().unwrap();
    assert_eq!(headers.len(), 5);
    assert_eq!(&headers[0], "id");
    assert_eq!(&headers[1], "product");

    let records: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();
    assert_eq!(records.len(), 3);
    assert_eq!(&records[0][0], "1001");
    assert_eq!(&records[0][1], "Gaming Laptop");
    assert_eq!(&records[1][1], ""); // NULL product exported as empty string
    assert_eq!(&records[1][3], "false");
}

#[test]
fn test_export_to_excel_roundtrip() {
    let result = create_sample_query_result();

    let temp = NamedTempFile::new().unwrap();
    let path = temp.path().with_extension("xlsx");

    export_to_excel(&result, &path, Some("Inventory")).expect("Failed to export to Excel");

    // Read back with calamine
    let mut workbook = open_workbook_auto(&path).unwrap();
    assert_eq!(workbook.sheet_names(), &["Inventory"]);

    let range = workbook.worksheet_range("Inventory").unwrap();
    assert_eq!(range.get_size(), (4, 5)); // 1 header row + 3 data rows, 5 columns

    // Verify row 0 (headers)
    assert_eq!(range.get_value((0, 0)).unwrap().as_string().unwrap(), "id");
    assert_eq!(range.get_value((0, 1)).unwrap().as_string().unwrap(), "product");

    // Verify row 1 (first data row)
    assert_eq!(range.get_value((1, 0)).unwrap().as_f64().unwrap() as i64, 1001);
    assert_eq!(range.get_value((1, 1)).unwrap().as_string().unwrap(), "Gaming Laptop");
    assert_eq!(range.get_value((1, 2)).unwrap().as_f64().unwrap(), 1299.99);
    assert_eq!(range.get_value((1, 3)).unwrap(), &calamine::Data::Bool(true));

    // Verify row 2 (row with NULL product)
    assert_eq!(range.get_value((2, 1)).unwrap(), &calamine::Data::Empty);

    let _ = std::fs::remove_file(path);
}

#[test]
fn test_export_to_json_roundtrip() {
    let result = create_sample_query_result();

    let temp = tempfile::Builder::new().suffix(".json").tempfile().unwrap();
    let path = temp.path().to_path_buf();

    export_to_json(&result, &path).expect("Failed to export to JSON");

    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let json_data: serde_json::Value = serde_json::from_reader(reader).unwrap();

    let arr = json_data.as_array().expect("Expected JSON array");
    assert_eq!(arr.len(), 3);

    // Row 1
    assert_eq!(arr[0]["id"], 1001);
    assert_eq!(arr[0]["product"], "Gaming Laptop");
    assert_eq!(arr[0]["price"], 1299.99);
    assert_eq!(arr[0]["in_stock"], true);

    // Row 2 (NULLs)
    assert!(arr[1]["product"].is_null());
    assert!(arr[1]["release_date"].is_null());
    assert_eq!(arr[1]["in_stock"], false);
}
