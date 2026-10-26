use localsql_studio::db::manager::DbManager;
use localsql_studio::db::types::{DbError, SqlValue};

#[test]
fn test_large_dataset_and_null_grid_formatting() {
    let db = DbManager::new_in_memory().expect("Failed to initialize DuckDB engine");

    // 1. Generate 10,000 rows with mixed types, nulls, and booleans
    let query = "
        SELECT 
            i AS id, 
            'Item #' || i AS name, 
            CASE WHEN i % 2 = 0 THEN i * 2.5 ELSE NULL END AS score, 
            (i % 3 = 0) AS active 
        FROM range(1, 10001) t(i)
    ";

    let result = db.execute_query(query).expect("Large query execution failed");

    // 2. Assert structural metrics
    assert_eq!(result.row_count, 10_000);
    assert_eq!(result.columns.len(), 4);
    assert_eq!(result.columns[0].name, "id");
    assert_eq!(result.columns[1].name, "name");
    assert_eq!(result.columns[2].name, "score");
    assert_eq!(result.columns[3].name, "active");

    // 3. Assert NULL values and boolean data
    // Row 1 (i = 1): score is NULL, active is false (1 % 3 != 0)
    assert_eq!(result.rows[0][0], SqlValue::BigInt(1));
    assert_eq!(result.rows[0][1], SqlValue::Text("Item #1".to_string()));
    assert_eq!(result.rows[0][2], SqlValue::Null);
    assert_eq!(result.rows[0][3], SqlValue::Boolean(false));

    // Row 2 (i = 2): score is 5.0 (2 * 2.5), active is false
    assert_eq!(result.rows[1][0], SqlValue::BigInt(2));
    assert_eq!(result.rows[1][1], SqlValue::Text("Item #2".to_string()));
    assert_eq!(result.rows[1][2], SqlValue::Double(5.0));
    assert_eq!(result.rows[1][3], SqlValue::Boolean(false));

    // Row 3 (i = 3): score is NULL, active is true (3 % 3 == 0)
    assert_eq!(result.rows[2][0], SqlValue::BigInt(3));
    assert_eq!(result.rows[2][1], SqlValue::Text("Item #3".to_string()));
    assert_eq!(result.rows[2][2], SqlValue::Null);
    assert_eq!(result.rows[2][3], SqlValue::Boolean(true));

    // 4. Assert timing metrics tracked
    assert!(result.execution_time_ms >= 0.0);
}

#[test]
fn test_query_syntax_error_diagnostic() {
    let db = DbManager::new_in_memory().expect("Failed to initialize DuckDB engine");

    // Query a non-existent table to verify graceful error return
    let invalid_query = "SELECT * FROM definitely_not_a_real_table_12345;";
    let result = db.execute_query(invalid_query);

    match result {
        Err(DbError::QueryError(msg)) => {
            assert!(
                msg.contains("definitely_not_a_real_table_12345") || msg.contains("Table with name"),
                "Expected descriptive error message, got: {}",
                msg
            );
        }
        other => panic!("Expected DbError::QueryError, got: {:?}", other),
    }
}
