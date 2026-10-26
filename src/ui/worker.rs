//! Background Worker Thread
//!
//! Handles file ingestion and SQL query execution off the main UI thread
//! using crossbeam channels, guaranteeing the UI never drops frames or freezes.

use std::path::PathBuf;
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::db::manager::DbManager;
use crate::db::types::{QueryResult, SqlValue};

/// Information about an ingested table for UI exploration.
#[derive(Debug, Clone, PartialEq)]
pub struct TableInfo {
    /// Full source file path
    pub file_path: String,
    /// Basename of the file (e.g. "sales.xlsx")
    pub file_name: String,
    /// Relational table name registered in DuckDB (e.g. "sales_q1")
    pub table_name: String,
    /// Original sheet or source name (e.g. "Q1 Sales")
    pub sheet_name: String,
    /// List of columns with (column_name, data_type)
    pub columns: Vec<(String, String)>,
    /// Row count in table
    pub row_count: usize,
}

/// Commands sent from the UI thread to the background worker.
#[derive(Debug, Clone)]
pub enum WorkerCommand {
    /// Ingest one or multiple Excel and/or CSV files
    IngestFiles(Vec<PathBuf>),
    /// Execute an analytical SQL query
    ExecuteQuery { query_id: u64, sql: String },
}

/// Events sent from the background worker back to the UI thread.
#[derive(Debug, Clone)]
pub enum WorkerEvent {
    /// Notification that file ingestion completed
    FilesIngested {
        tables: Vec<TableInfo>,
        errors: Vec<String>,
    },
    /// Notification that query execution completed
    QueryFinished {
        query_id: u64,
        result: Result<QueryResult, String>,
    },
}

/// Starts the background worker thread and returns UI communication channels.
pub fn start_worker() -> (Sender<WorkerCommand>, Receiver<WorkerEvent>) {
    let (cmd_tx, cmd_rx) = unbounded::<WorkerCommand>();
    let (event_tx, event_rx) = unbounded::<WorkerEvent>();

    thread::Builder::new()
        .name("spreadbase-worker".to_string())
        .spawn(move || {
            let db = match DbManager::new_in_memory() {
                Ok(manager) => manager,
                Err(e) => {
                    eprintln!("Failed to initialize in-memory DuckDB engine: {}", e);
                    return;
                }
            };

            while let Ok(cmd) = cmd_rx.recv() {
                match cmd {
                    WorkerCommand::IngestFiles(paths) => {
                        let mut new_tables = Vec::new();
                        let mut errors = Vec::new();

                        for path in paths {
                            let file_str = path.to_string_lossy().to_string();
                            let file_name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string();

                            let ext = path
                                .extension()
                                .and_then(|e| e.to_str())
                                .unwrap_or("")
                                .to_lowercase();

                            let registered_tables = if ext == "xlsx" || ext == "xls" {
                                db.register_excel_file(&path)
                            } else if ext == "csv" {
                                db.register_csv_file(&path, None).map(|t| vec![t])
                            } else {
                                errors.push(format!(
                                    "Unsupported file extension '.{}' for {:?}",
                                    ext, file_name
                                ));
                                continue;
                            };

                            match registered_tables {
                                Ok(table_names) => {
                                    for tbl in table_names {
                                        // Retrieve column metadata
                                        let col_query = format!("PRAGMA table_info('{}')", tbl);
                                        let mut columns = Vec::new();
                                        if let Ok(col_res) = db.execute_query(&col_query) {
                                            for row in col_res.rows {
                                                if row.len() >= 3 {
                                                    let col_name = match &row[1] {
                                                        SqlValue::Text(s) => s.clone(),
                                                        other => other.to_string(),
                                                    };
                                                    let col_type = match &row[2] {
                                                        SqlValue::Text(s) => s.clone(),
                                                        other => other.to_string(),
                                                    };
                                                    columns.push((col_name, col_type));
                                                }
                                            }
                                        }

                                        // Retrieve row count
                                        let count_query = format!("SELECT COUNT(*) FROM \"{}\"", tbl);
                                        let row_count = db
                                            .execute_query(&count_query)
                                            .ok()
                                            .and_then(|res| res.rows.first().cloned())
                                            .and_then(|row| row.first().cloned())
                                            .map(|val| match val {
                                                SqlValue::BigInt(c) => c as usize,
                                                SqlValue::HugeInt(c) => c as usize,
                                                SqlValue::Integer(c) => c as usize,
                                                _ => 0,
                                            })
                                            .unwrap_or(0);

                                        new_tables.push(TableInfo {
                                            file_path: file_str.clone(),
                                            file_name: file_name.clone(),
                                            table_name: tbl.clone(),
                                            sheet_name: tbl,
                                            columns,
                                            row_count,
                                        });
                                    }
                                }
                                Err(err) => {
                                    errors.push(format!("Failed to load {}: {}", file_name, err));
                                }
                            }
                        }

                        let _ = event_tx.send(WorkerEvent::FilesIngested {
                            tables: new_tables,
                            errors,
                        });
                    }

                    WorkerCommand::ExecuteQuery { query_id, sql } => {
                        let result = db.execute_query(&sql).map_err(|e| e.to_string());
                        let _ = event_tx.send(WorkerEvent::QueryFinished { query_id, result });
                    }
                }
            }
        })
        .expect("Failed to spawn worker thread");

    (cmd_tx, event_rx)
}
