use std::io::Write;
use std::time::Duration;

use rust_xlsxwriter::Workbook;
use tempfile::NamedTempFile;

use localsql_studio::db::types::SqlValue;
use localsql_studio::ui::worker::{start_worker, WorkerCommand, WorkerEvent};

#[test]
fn test_worker_thread_ingestion_and_query() {
    // 1. Create temporary Excel file
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name("Inventory").unwrap();

    sheet.write(0, 0, "item_id").unwrap();
    sheet.write(0, 1, "item_name").unwrap();
    sheet.write(0, 2, "quantity").unwrap();

    sheet.write(1, 0, 1i64).unwrap();
    sheet.write(1, 1, "Apples").unwrap();
    sheet.write(1, 2, 50i64).unwrap();

    sheet.write(2, 0, 2i64).unwrap();
    sheet.write(2, 1, "Bananas").unwrap();
    sheet.write(2, 2, 80i64).unwrap();

    let excel_temp = NamedTempFile::new().unwrap();
    let excel_path = excel_temp.path().with_extension("xlsx");
    workbook.save(&excel_path).unwrap();

    // 2. Create temporary CSV file with .csv extension
    let mut csv_temp = tempfile::Builder::new()
        .suffix(".csv")
        .tempfile()
        .unwrap();
    writeln!(csv_temp, "sku,price,in_stock").unwrap();
    writeln!(csv_temp, "1,1.99,true").unwrap();
    writeln!(csv_temp, "2,0.89,true").unwrap();
    let csv_path = csv_temp.path().to_path_buf();

    // 3. Start background worker
    let (tx, rx) = start_worker();

    // 4. Dispatch IngestFiles command
    tx.send(WorkerCommand::IngestFiles(vec![excel_path.clone(), csv_path]))
        .expect("Failed to send IngestFiles");

    // Wait for FilesIngested event (timeout 10s)
    let event = rx
        .recv_timeout(Duration::from_secs(10))
        .expect("Timed out waiting for FilesIngested event");

    let tables = match event {
        WorkerEvent::FilesIngested { tables, errors } => {
            assert!(errors.is_empty(), "Ingestion returned errors: {:?}", errors);
            assert_eq!(tables.len(), 2, "Expected 2 tables ingested (1 Excel sheet + 1 CSV)");
            tables
        }
        other => panic!("Unexpected event received: {:?}", other),
    };

    // Verify metadata for Excel table
    let excel_table = tables
        .iter()
        .find(|t| t.sheet_name.contains("Inventory"))
        .expect("Inventory table not found");
    assert_eq!(excel_table.row_count, 2);
    assert_eq!(excel_table.columns.len(), 3);
    assert_eq!(excel_table.columns[0].0, "item_id");
    assert_eq!(excel_table.columns[1].0, "item_name");
    assert_eq!(excel_table.columns[2].0, "quantity");

    // 5. Dispatch SQL Query over channel
    let sql = format!(
        "SELECT item_name, quantity FROM \"{}\" WHERE item_id = 1",
        excel_table.table_name
    );
    tx.send(WorkerCommand::ExecuteQuery {
        query_id: 42,
        sql,
    })
    .expect("Failed to send ExecuteQuery");

    let query_event = rx
        .recv_timeout(Duration::from_secs(10))
        .expect("Timed out waiting for QueryFinished event");

    match query_event {
        WorkerEvent::QueryFinished { query_id, result } => {
            assert_eq!(query_id, 42);
            let res = result.expect("Query failed");
            assert_eq!(res.row_count, 1);
            assert_eq!(res.rows[0][0], SqlValue::Text("Apples".to_string()));
            assert_eq!(res.rows[0][1], SqlValue::BigInt(50));
        }
        other => panic!("Unexpected query event: {:?}", other),
    }

    // Cleanup temporary files
    let _ = std::fs::remove_file(excel_path);
}
