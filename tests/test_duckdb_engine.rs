//! Integration tests for embedded DuckDB engine and DbManager

use localsql_studio::db::{DbManager, SqlValue};

#[test]
fn test_in_memory_connection_initialization() {
    let db = DbManager::new_in_memory().expect("Failed to initialize in-memory DuckDB");
    let result = db.execute_query("SELECT 42 AS answer").expect("Query execution failed");
    
    assert_eq!(result.row_count, 1);
    assert_eq!(result.columns.len(), 1);
    assert_eq!(result.columns[0].name, "answer");
    assert_eq!(result.rows[0][0], SqlValue::Integer(42));
    assert!(result.execution_time_ms >= 0.0);
}

#[test]
fn test_basic_data_types() {
    let db = DbManager::new_in_memory().unwrap();
    let query = "SELECT 100::BIGINT AS big, 3.1415::DOUBLE AS flt, 'LocalSQL' AS str, true AS bool_val, NULL AS null_val";
    let result = db.execute_query(query).unwrap();

    assert_eq!(result.row_count, 1);
    assert_eq!(result.columns.len(), 5);
    
    assert_eq!(result.rows[0][0], SqlValue::BigInt(100));
    assert_eq!(result.rows[0][1], SqlValue::Double(3.1415));
    assert_eq!(result.rows[0][2], SqlValue::Text("LocalSQL".to_string()));
    assert_eq!(result.rows[0][3], SqlValue::Boolean(true));
    assert_eq!(result.rows[0][4], SqlValue::Null);
}

#[test]
fn test_table_creation_and_aggregations() {
    let db = DbManager::new_in_memory().unwrap();

    db.execute_statement("CREATE TABLE sales (id INTEGER, region VARCHAR, amount DOUBLE)").unwrap();
    db.execute_statement("INSERT INTO sales VALUES (1, 'North', 150.0), (2, 'North', 250.0), (3, 'South', 300.0)").unwrap();

    let query = "SELECT region, SUM(amount) AS total, COUNT(*) AS orders FROM sales GROUP BY region ORDER BY region";
    let result = db.execute_query(query).unwrap();

    assert_eq!(result.row_count, 2);
    // North
    assert_eq!(result.rows[0][0], SqlValue::Text("North".to_string()));
    assert_eq!(result.rows[0][1], SqlValue::Double(400.0));
    assert_eq!(result.rows[0][2], SqlValue::BigInt(2));
    // South
    assert_eq!(result.rows[1][0], SqlValue::Text("South".to_string()));
    assert_eq!(result.rows[1][1], SqlValue::Double(300.0));
    assert_eq!(result.rows[1][2], SqlValue::BigInt(1));
}

#[test]
fn test_advanced_analytical_sql() {
    let db = DbManager::new_in_memory().unwrap();

    db.execute_statement("CREATE TABLE employees (id INTEGER, name VARCHAR, salary DOUBLE, dept VARCHAR)").unwrap();
    db.execute_statement("INSERT INTO employees VALUES 
        (1, 'Alice', 90000.0, 'Engineering'),
        (2, 'Bob', 80000.0, 'Engineering'),
        (3, 'Charlie', 70000.0, 'Marketing')
    ").unwrap();

    // Test CTE and Window Function
    let query = "
        WITH ranked AS (
            SELECT name, dept, salary,
                   ROW_NUMBER() OVER (PARTITION BY dept ORDER BY salary DESC) as rank
            FROM employees
        )
        SELECT name, dept, salary FROM ranked WHERE rank = 1 ORDER BY dept
    ";
    let result = db.execute_query(query).unwrap();

    assert_eq!(result.row_count, 2);
    assert_eq!(result.rows[0][0], SqlValue::Text("Alice".to_string()));
    assert_eq!(result.rows[1][0], SqlValue::Text("Charlie".to_string()));
}

#[test]
fn test_error_handling_graceful_recovery() {
    let db = DbManager::new_in_memory().unwrap();

    // Syntax error must return Err without panic
    let bad_query = "SELECT FROM WHERE UNKNOWN";
    let err = db.execute_query(bad_query).unwrap_err();
    assert!(err.to_string().contains("Query Error"));

    // Query on non-existent table
    let missing_table = "SELECT * FROM non_existent_table";
    let err2 = db.execute_query(missing_table).unwrap_err();
    assert!(err2.to_string().contains("Query Error"));
}
