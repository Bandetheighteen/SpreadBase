# Feature Research

**Domain:** Local Desktop SQL Workbench (100% Rust)
**Researched:** 2026-10-14
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist in any modern database workbench. Missing these makes the product feel broken or incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Excel as Database Ingestion** | Core user requirement: Opening an `.xlsx` file treats the file as a database and registers every sheet inside it as an independent table. | MEDIUM | Parse sheets with `calamine`, sanitize sheet names into valid SQL identifiers, and register as DuckDB tables. |
| **CSV File Ingestion** | Primary data format alongside Excel: Direct registration of `.csv` files as queryable tables. | LOW | Register via DuckDB's native `read_csv_auto` or stream via `csv` crate for sub-second load times. |
| **Sidebar Schema Explorer** | Users must see what databases (files) and tables (sheets/CSVs) are currently registered and inspect their columns. | MEDIUM | Hierarchical tree widget in `egui` displaying Database -> Tables -> Column names + Inferred SQL types. |
| **SQL Query Editor** | Multi-line text input for drafting and executing SQL queries with syntax highlighting. | MEDIUM | Text editor widget supporting selection execution, keyboard shortcuts (F5, Ctrl+Enter), and indentation. |
| **Virtual Data Grid** | Renders query results with zero lag, even for 50,000+ rows. | MEDIUM | `egui_extras::TableBuilder` virtualized rendering; only draws rows currently visible in the viewport. |
| **Query Execution Metrics** | Instant feedback on row count returned and query runtime in milliseconds. | LOW | Track execution duration using `std::time::Instant` and display in status footer: `Returned 1,420 rows in 3.4ms`. |
| **Diagnostic Error Messages** | Clear explanation of SQL syntax errors, missing columns, or invalid table names. | LOW | Catch DuckDB errors cleanly and display in an error notification banner without crashing the application. |
| **Multi-Format Export** | Ability to save query results to CSV, Excel (`.xlsx`), JSON, and Parquet. | MEDIUM | Built-in exporters via `rust_xlsxwriter`, `serde_json`, `csv`, and DuckDB native `COPY`. |

### Differentiators (Competitive Advantage)

Features that set this 100% Rust LocalSQL Studio apart from laggy Python and Electron tools.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Zero-Lag Immediate GUI** | 60+ FPS native GPU rendering, instant launch (<100ms), and near-zero memory footprint (<30MB RAM vs 300MB+ for Electron). | MEDIUM | Powered by `eframe`/`egui`. No web workers, no DOM overhead, no IPC string serialization. |
| **Cross-Sheet & Cross-File SQL Joins** | Users can seamlessly JOIN data across sheets within an Excel workbook or between an Excel sheet and a standalone CSV file. | LOW | Inherited naturally from DuckDB's unified in-memory catalog. |
| **Advanced Analytical SQL** | Support for CTEs (`WITH ...`), window functions (`RANK()`, `LEAD()`, `LAG()`), pivot queries, and regex functions. | LOW | Full DuckDB SQL engine capabilities available out of the box. |
| **Single Standalone Executable** | No Python environment, no venv, no Node runtime, no installer required. Double-click to run. | LOW | Single statically linked Windows `.exe` produced by Cargo. |
| **Non-Blocking Query Dispatch** | Long-running queries run in dedicated background threads; UI stays completely interactive with cancel capability. | MEDIUM | Thread pool architecture using `crossbeam_channel` for asynchronous result streaming. |

### Anti-Features (Avoid Building)

Features that seem appealing but create severe complexity or compromise product safety.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| **In-Place Excel File Mutation** | Users might want to `UPDATE` rows directly in the original `.xlsx` file. | High risk of destroying complex formulas, macros, or formatting in the user's original spreadsheet. | Keep all operations strictly read-only analytical SQL. Provide "Export to Excel" for writing modified datasets. |
| **Embedded Web Browser / Monaco in WebView** | Desiring Monaco editor web features. | Reintroduces the exact Chromium/IPC lag and multi-megabyte runtime bloat the user specifically asked to eliminate. | Native `egui` code editor with custom Rust syntax tokenizer. |
| **Client-Server Architecture** | Wanting to connect to remote databases (PostgreSQL, MySQL). | Massive scope creep, complex connection management, and security credentials storage. | Pure local-first workbench focused 100% on local CSV/Excel analytics. |

## Feature Dependencies

```
[File Picker (rfd)]
    └──requires──> [Excel/CSV Parsers (calamine / csv)]
                       └──requires──> [DuckDB Catalog Registration]
                                          └──requires──> [Sidebar Explorer]
                                          └──requires──> [SQL Query Engine]
                                                             └──requires──> [Virtual Results Grid]
                                                             └──requires──> [Exporter (CSV/XLSX/JSON)]
```

## MVP Definition

### Launch With (v1)

- [ ] Native `eframe`/`egui` desktop application window with clean dark IDE theme.
- [ ] File ingestion: Open Excel file (`.xlsx`, `.xls`) -> Auto-detect sheets -> Register as `{workbook}_{sheet}` tables in DuckDB.
- [ ] File ingestion: Open CSV file -> Auto-detect schema -> Register as `{filename}` table in DuckDB.
- [ ] Left sidebar Explorer listing active databases, tables, and column metadata.
- [ ] Center SQL Editor with execution hotkeys (F5, Ctrl+Enter).
- [ ] Asynchronous query execution with row count and duration metrics.
- [ ] Virtualized results grid with column headers and NULL formatting.
- [ ] Export results to CSV, Excel (`.xlsx`), and JSON via native save dialogs.
- [ ] Automated unit test suite verifying parser and DuckDB registration.

### Add After Validation (v1.x)

- [ ] Query history list and saved favorite queries.
- [ ] Table filtering and sorting directly from grid headers.
- [ ] Quick template generation (e.g. right-click table -> "SELECT * FROM ... LIMIT 100").

### Future Consideration (v2+)

- [ ] Parquet file ingestion and direct DuckDB file queries.
- [ ] Visual schema diagram generator for multi-sheet relationships.

---
*Feature research for: Local Desktop SQL Workbench (100% Rust)*
*Researched: 2026-10-14*
