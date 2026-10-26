# Local SQL Queries on CSV and Excel Files

> Archived: 2026-10-26

## Overview

A synthesis of tools and approaches for querying local CSV and Excel files using SQL without uploading data to external servers, treating local files as database tables.

## Existing Tools

There are several tools available that operate locally (processing via the machine's own CPU/RAM) and are privacy-first:

### Graphical User Interface (GUI) Tools
* **SpreadBase**: A standalone, 100% offline desktop application in pure Rust embedding DuckDB. Automatically registers sheets in multi-sheet Excel (.xlsx, .xls) and CSV files as relational tables, executes analytical SQL joins with zero lag, displays results in a 60 FPS virtualized table grid, and exports directly to Excel, CSV, and JSON.
* **DBeaver**: A universal database desktop application. Connecting via a "CSV" or "Flat File" driver allows a local directory to act as a database where every CSV is a queryable table.
* **Tad**: A fast, cross-platform desktop application embedding DuckDB. Designed for tabular data (CSV, Parquet, Excel), it allows instant viewing, filtering, pivoting, and SQL execution natively on local files.
* **DB Browser for SQLite**: A lightweight tool for SQLite. You can import CSVs into a local `.db` file and run SQL queries against it.

### Command-Line (CLI) Tools
* **DuckDB**: An in-process SQL database optimized for data analytics. It natively reads CSV, JSON, Parquet, and Excel files, providing extremely fast local execution (e.g., `SELECT * FROM read_csv_auto('data.csv')`).
* **q (harelba/q)**: A Python-based command-line tool that temporarily loads CSV/TSV files into an in-memory SQLite database, allowing SQL execution against the file.
* **TextQL**: Similar to `q`, written in Go, it imports data into an in-memory SQLite database for querying and drops it immediately after.
* **CSVQ**: A fast CLI tool in Go with a native lightweight query engine specifically optimized for CSV operations.
* **dsq**: A smart wrapper that pipes data (CSV, Excel, JSON, Parquet) into SQLite or DuckDB for highly performant queries.
* **OctoSQL**: A data analysis CLI tool for joining and querying data from multiple different file formats (like combining a local CSV with an Excel file) using standard SQL.

## Architecture for a Custom Local Tool

If building a custom UI tool for this purpose:

### Interface Layout
* **Top Left (File Manager)**: Area to browse/select local CSV/Excel files. Loaded files act as tables.
* **Right Side (Editor)**: SQL query editor with syntax highlighting.
* **Bottom (Results)**: Data grid showing the query results.

### Core Mechanics
* **Mounting**: Selected files are registered as tables in an embedded database engine (like DuckDB or SQLite) in-memory.
* **Executing**: The SQL text is sent to the local engine for processing against the mounted tables.
* **Rendering**: The raw row data returned by the engine is displayed in the data grid.

### Technology Stack Options
* **Pure Rust Desktop App (Implemented in SpreadBase v1.0.0)**: Built with `eframe`/`egui` (immediate-mode GPU rendering) and in-process DuckDB. Extremely fast startup, 40.8 MB standalone binary, zero runtime dependencies, crossbeam worker threads for non-blocking UI.
* **Web App (Most Portable)**: HTML/JS + **DuckDB-WASM** (WebAssembly). Runs completely locally within a web browser without requiring a server backend.
* **Python Desktop App**: CustomTkinter or PyQt for UI, using Python's built-in `sqlite3` or `duckdb` module.
* **Electron App (Professional)**: Node.js, React/Vue, and DuckDB packaged as a standalone application.

## See Also

- [SpreadBase: Standalone Rust Desktop SQL Studio for Spreadsheets](spreadbase-rust-desktop-sql-studio.md)
- [Go Language CLI Tools for Local SQL](go-language-cli-tools-for-local-sql.md)
