//! Integration tests for Excel & CSV ingestion, table registration, and multi-sheet SQL queries.

use localsql_studio::db::{DbManager, SqlValue};
use rust_xlsxwriter::Workbook;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_multi_sheet_excel_ingestion_and_joins() {
    // 1. Create a multi-sheet Excel file fixture using rust_xlsxwriter
    let mut workbook = Workbook::new();

    // Sheet 1: Customers
    let sheet1 = workbook.add_worksheet();
    sheet1.set_name("Customers").unwrap();
    sheet1.write(0, 0, "id").unwrap();
    sheet1.write(0, 1, "name").unwrap();
    sheet1.write(0, 2, "city").unwrap();

    sheet1.write(1, 0, 1i64).unwrap();
    sheet1.write(1, 1, "Alice").unwrap();
    sheet1.write(1, 2, "London").unwrap();

    sheet1.write(2, 0, 2i64).unwrap();
    sheet1.write(2, 1, "Bob").unwrap();
    sheet1.write(2, 2, "Paris").unwrap();

    sheet1.write(3, 0, 3i64).unwrap();
    sheet1.write(3, 1, "Charlie").unwrap();
    sheet1.write(3, 2, "Berlin").unwrap();

    // Sheet 2: Orders
    let sheet2 = workbook.add_worksheet();
    sheet2.set_name("Orders").unwrap();
    sheet2.write(0, 0, "order_id").unwrap();
    sheet2.write(0, 1, "customer_id").unwrap();
    sheet2.write(0, 2, "amount").unwrap();

    sheet2.write(1, 0, 101i64).unwrap();
    sheet2.write(1, 1, 1i64).unwrap();
    sheet2.write(1, 2, 250.50f64).unwrap();

    sheet2.write(2, 0, 102i64).unwrap();
    sheet2.write(2, 1, 1i64).unwrap();
    sheet2.write(2, 2, 149.50f64).unwrap();

    sheet2.write(3, 0, 103i64).unwrap();
    sheet2.write(3, 1, 2i64).unwrap();
    sheet2.write(3, 2, 500.00f64).unwrap();

    let excel_temp = NamedTempFile::new().unwrap();
    let excel_path = excel_temp.path().with_extension("xlsx");
    workbook.save(&excel_path).unwrap();

    // 2. Create CSV file fixture
    let mut csv_temp = NamedTempFile::new().unwrap();
    writeln!(csv_temp, "product_id,product_name,category").unwrap();
    writeln!(csv_temp, "10,Widget,Hardware").unwrap();
    writeln!(csv_temp, "20,Gadget,Electronics").unwrap();

    // 3. Register Excel sheets and CSV into in-memory DuckDB
    let db = DbManager::new_in_memory().unwrap();
    let excel_tables = db.register_excel_file(&excel_path).expect("Failed to register Excel file");
    assert_eq!(excel_tables.len(), 2);

    let csv_table = db.register_csv_file(csv_temp.path(), Some("Products")).expect("Failed to register CSV file");
    assert_eq!(csv_table, "Products");

    let tables = db.list_tables().unwrap();
    assert!(tables.len() >= 3);

    // 4. Test Multi-Sheet SQL JOIN query
    // Querying Customers JOIN Orders across sheets
    let join_query = format!(
        "SELECT c.name, SUM(o.amount) AS total_spent
         FROM \"{}\" c
         JOIN \"{}\" o ON c.id = o.customer_id
         GROUP BY c.name
         ORDER BY total_spent DESC",
        excel_tables[0], excel_tables[1]
    );

    let result = db.execute_query(&join_query).expect("Multi-sheet JOIN query failed");
    assert_eq!(result.row_count, 2);

    // First row: Bob (500.00)
    assert_eq!(result.rows[0][0], SqlValue::Text("Bob".to_string()));
    assert_eq!(result.rows[0][1], SqlValue::Double(500.00));

    // Second row: Alice (400.00)
    assert_eq!(result.rows[1][0], SqlValue::Text("Alice".to_string()));
    assert_eq!(result.rows[1][1], SqlValue::Double(400.00));

    // 5. Test Cross-File JOIN (Excel sheet JOIN CSV table)
    let cross_file_sql = format!(
        "SELECT o.order_id, p.product_name, o.amount
         FROM \"{}\" o
         CROSS JOIN \"{}\" p
         WHERE p.product_id = 10 AND o.order_id = 101",
        excel_tables[1], csv_table
    );
    let cross_res = db.execute_query(&cross_file_sql).expect("Cross-file JOIN failed");
    assert_eq!(cross_res.row_count, 1);
    assert_eq!(cross_res.rows[0][0], SqlValue::BigInt(101));
    assert_eq!(cross_res.rows[0][1], SqlValue::Text("Widget".to_string()));
    assert_eq!(cross_res.rows[0][2], SqlValue::Double(250.50));

    // Clean up temporary files
    let _ = std::fs::remove_file(excel_path);
}

#[test]
fn test_heterogeneous_column_type_reconciliation() {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name("MixedData").unwrap();

    sheet.write(0, 0, "code").unwrap();
    sheet.write(0, 1, "description").unwrap();

    // Mixed types in the 'code' column: numbers, text, and blanks
    sheet.write(1, 0, 100i64).unwrap();
    sheet.write(1, 1, "Numeric code").unwrap();

    sheet.write(2, 0, "ABC-999").unwrap();
    sheet.write(2, 1, "Alphanumeric code").unwrap();

    // Blank row cell
    sheet.write(3, 1, "Blank code").unwrap();

    let temp = NamedTempFile::new().unwrap();
    let excel_path = temp.path().with_extension("xlsx");
    workbook.save(&excel_path).unwrap();

    let db = DbManager::new_in_memory().unwrap();
    let tables = db.register_excel_file(&excel_path).unwrap();

    let query = format!("SELECT code, description FROM \"{}\" ORDER BY description", tables[0]);
    let result = db.execute_query(&query).expect("Query on mixed column table failed");

    assert_eq!(result.row_count, 3);
    // Blank code -> NULL
    assert_eq!(result.rows[1][0], SqlValue::Null);
    assert_eq!(result.rows[1][1], SqlValue::Text("Blank code".to_string()));

    let _ = std::fs::remove_file(excel_path);
}
