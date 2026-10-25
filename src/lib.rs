//! LocalSQL Studio — Core Library
//!
//! Exposes the four primary modules:
//! - `db`      — Embedded DuckDB database management and query execution
//! - `parsers` — Excel and CSV file ingestion
//! - `ui`      — Desktop GUI components (eframe/egui)
//! - `export`  — Multi-format result export (CSV, XLSX, JSON)

pub mod db;
pub mod export;
pub mod parsers;
pub mod ui;
