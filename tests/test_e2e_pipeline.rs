use std::io::Write;

use rust_xlsxwriter::Workbook;
use tempfile::NamedTempFile;

use localsql_studio::db::manager::DbManager;
use localsql_studio::db::types::SqlValue;
use localsql_studio::export::{export_to_csv, export_to_excel};

#[test]
fn test_end_to_end_circular_pipeline() {
    // =========================================================================
    // STEP 1: Create initial Excel file fixture with multiple sheets
    // =========================================================================
    let mut workbook = Workbook::new();

    // Sheet 1: Departments
    let sheet1 = workbook.add_worksheet();
    sheet1.set_name("Departments").unwrap();
    sheet1.write(0, 0, "dept_id").unwrap();
    sheet1.write(0, 1, "dept_name").unwrap();

    sheet1.write(1, 0, 1i64).unwrap();
    sheet1.write(1, 1, "Engineering").unwrap();

    sheet1.write(2, 0, 2i64).unwrap();
    sheet1.write(2, 1, "Marketing").unwrap();

    // Sheet 2: Employees
    let sheet2 = workbook.add_worksheet();
    sheet2.set_name("Employees").unwrap();
    sheet2.write(0, 0, "emp_id").unwrap();
    sheet2.write(0, 1, "emp_name").unwrap();
    sheet2.write(0, 2, "dept_id").unwrap();

    sheet2.write(1, 0, 101i64).unwrap();
    sheet2.write(1, 1, "Alice").unwrap();
    sheet2.write(1, 2, 1i64).unwrap();

    sheet2.write(2, 0, 102i64).unwrap();
    sheet2.write(2, 1, "Bob").unwrap();
    sheet2.write(2, 2, 1i64).unwrap();

    sheet2.write(3, 0, 103i64).unwrap();
    sheet2.write(3, 1, "Charlie").unwrap();
    sheet2.write(3, 2, 2i64).unwrap();

    let excel_temp = NamedTempFile::new().unwrap();
    let excel_path = excel_temp.path().with_extension("xlsx");
    workbook.save(&excel_path).unwrap();

    // =========================================================================
    // STEP 2: Create initial CSV file fixture
    // =========================================================================
    let mut csv_temp = tempfile::Builder::new()
        .suffix(".csv")
        .tempfile()
        .unwrap();
    writeln!(csv_temp, "emp_id,amount").unwrap();
    writeln!(csv_temp, "101,95000.50").unwrap();
    writeln!(csv_temp, "102,105000.20").unwrap();
    writeln!(csv_temp, "103,72000.80").unwrap();
    let csv_path = csv_temp.path().to_path_buf();

    // =========================================================================
    // STEP 3: Ingest both files into in-memory database engine
    // =========================================================================
    let db1 = DbManager::new_in_memory().expect("Failed to create DbManager");
    let excel_tables = db1.register_excel_file(&excel_path).expect("Failed to ingest Excel");
    assert_eq!(excel_tables.len(), 2);

    let csv_table = db1.register_csv_file(&csv_path, None).expect("Failed to ingest CSV");

    // =========================================================================
    // STEP 4: Execute multi-sheet cross-file analytical SQL JOIN query
    // =========================================================================
    let join_sql = format!(
        "SELECT 
            d.dept_name,
            COUNT(e.emp_id) AS total_employees,
            ROUND(AVG(s.amount), 2) AS avg_salary
         FROM \"{}\" d
         JOIN \"{}\" e ON d.dept_id = e.dept_id
         JOIN \"{}\" s ON e.emp_id = s.emp_id
         GROUP BY d.dept_name
         ORDER BY avg_salary DESC",
        excel_tables[0], excel_tables[1], csv_table
    );

    let query_result = db1.execute_query(&join_sql).expect("Complex JOIN query failed");
    assert_eq!(query_result.row_count, 2);
    assert_eq!(query_result.columns.len(), 3);

    // Row 1: Engineering, 2 employees, 100,000.35 avg salary
    assert_eq!(query_result.rows[0][0], SqlValue::Text("Engineering".to_string()));
    assert_eq!(query_result.rows[0][1], SqlValue::BigInt(2));
    assert_eq!(query_result.rows[0][2], SqlValue::Double(100000.35));

    // Row 2: Marketing, 1 employee, 72,000.80 avg salary
    assert_eq!(query_result.rows[1][0], SqlValue::Text("Marketing".to_string()));
    assert_eq!(query_result.rows[1][1], SqlValue::BigInt(1));
    assert_eq!(query_result.rows[1][2], SqlValue::Double(72000.80));

    // =========================================================================
    // STEP 5: Export results to new Excel and CSV files
    // =========================================================================
    let exported_excel_temp = NamedTempFile::new().unwrap();
    let exported_excel_path = exported_excel_temp.path().with_extension("xlsx");
    export_to_excel(&query_result, &exported_excel_path, Some("DeptSalaryReport"))
        .expect("Failed to export results to Excel");

    let exported_csv_temp = tempfile::Builder::new()
        .suffix(".csv")
        .tempfile()
        .unwrap();
    let exported_csv_path = exported_csv_temp.path().to_path_buf();
    export_to_csv(&query_result, &exported_csv_path)
        .expect("Failed to export results to CSV");

    // =========================================================================
    // STEP 6: Re-ingest the exported files into a fresh new DbManager instance
    // =========================================================================
    let db2 = DbManager::new_in_memory().expect("Failed to create fresh DbManager");
    let reimported_excel_tables = db2
        .register_excel_file(&exported_excel_path)
        .expect("Failed to re-import exported Excel file");
    let reimported_csv_table = db2
        .register_csv_file(&exported_csv_path, Some("reimported_csv_report"))
        .expect("Failed to re-import exported CSV file");

    // =========================================================================
    // STEP 7: Verify complete data fidelity on re-imported datasets
    // =========================================================================
    // 7a. Query re-imported Excel table
    let check_excel_sql = format!(
        "SELECT dept_name, total_employees, avg_salary FROM \"{}\" ORDER BY avg_salary DESC",
        reimported_excel_tables[0]
    );
    let recheck_excel = db2.execute_query(&check_excel_sql).expect("Query on re-imported Excel failed");
    assert_eq!(recheck_excel.row_count, 2);
    assert_eq!(recheck_excel.rows[0][0], SqlValue::Text("Engineering".to_string()));
    assert_eq!(recheck_excel.rows[0][1], SqlValue::BigInt(2));
    assert_eq!(recheck_excel.rows[0][2], SqlValue::Double(100000.35));
    assert_eq!(recheck_excel.rows[1][0], SqlValue::Text("Marketing".to_string()));
    assert_eq!(recheck_excel.rows[1][1], SqlValue::BigInt(1));
    assert_eq!(recheck_excel.rows[1][2], SqlValue::Double(72000.80));

    // 7b. Query re-imported CSV table
    let check_csv_sql = format!(
        "SELECT dept_name, total_employees, avg_salary FROM \"{}\" ORDER BY avg_salary DESC",
        reimported_csv_table
    );
    let recheck_csv = db2.execute_query(&check_csv_sql).expect("Query on re-imported CSV failed");
    assert_eq!(recheck_csv.row_count, 2);
    assert_eq!(recheck_csv.rows[0][0], SqlValue::Text("Engineering".to_string()));
    assert_eq!(recheck_csv.rows[0][1], SqlValue::BigInt(2));
    assert_eq!(recheck_csv.rows[0][2], SqlValue::Double(100000.35));
    assert_eq!(recheck_csv.rows[1][0], SqlValue::Text("Marketing".to_string()));
    assert_eq!(recheck_csv.rows[1][1], SqlValue::BigInt(1));
    assert_eq!(recheck_csv.rows[1][2], SqlValue::Double(72000.80));

    // Clean up temporary files
    let _ = std::fs::remove_file(excel_path);
    let _ = std::fs::remove_file(exported_excel_path);
}
