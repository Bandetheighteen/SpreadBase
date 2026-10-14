# Architecture Research

**Domain:** Local Desktop SQL Workbench (100% Rust)
**Researched:** 2026-10-14
**Confidence:** HIGH

## Standard Architecture

### System Overview

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       GUI Layer (eframe / egui)                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                              Toolbar                                  │  │
│  │   [Open Excel/CSV]  [Run Query (F5)]  [Export Data]  [Clear Workspace]│  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│  ┌───────────────────────┬───────────────────────────────────────────────┐  │
│  │   Sidebar Explorer    │             Center Main Workspace             │  │
│  │   (Databases & Tables)│  ┌─────────────────────────────────────────┐  │  │
│  │                       │  │      SQL Editor (TextEdit / Tabbed)     │  │  │
│  │   📁 sales.xlsx (DB)  │  └─────────────────────────────────────────┘  │  │
│  │     📊 sheet_orders   │  ┌─────────────────────────────────────────┐  │  │
│  │     📊 sheet_items    │  │ Virtual Data Grid (egui_extras::Table)  │  │  │
│  │   📄 customers.csv    │  │ Column Headers | Virtualized Row Scroll │  │  │
│  │                       │  └─────────────────────────────────────────┘  │  │
│  └───────────────────────┴───────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │           Status Bar: [🟢 Ready | 1,420 rows returned in 2.1ms]         │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                      Crossbeam Channel (Request / Response)
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Background Query & Ingestion Engine                      │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────┐  ┌──────────────────────────────────────────┐  │
│  │    Dataset Ingestion    │  │              DuckDB Manager              │  │
│  │   - `calamine` (Excel)  │  │   - In-memory OLAP Connection            │  │
│  │   - `csv` (Streaming)   │  │   - Table/View Dynamic Registration      │  │
│  │   - Schema & Type Infer │  │   - Async Query Executor & Profiler      │  │
│  └─────────────────────────┘  └──────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                         Exporter Subsystem                            │  │
│  │     - CSV Exporter  - Excel (`rust_xlsxwriter`)  - JSON Serializer     │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Implementation |
|-----------|----------------|----------------|
| **App Shell (`app.rs`)** | Coordinates `eframe::App` lifecycle, layout splitters, keyboard shortcuts, and channel events. | `struct LocalSqlApp` implementing `eframe::App`. |
| **Explorer Widget (`ui/explorer.rs`)** | Displays tree of loaded databases (files), tables (sheets/CSVs), and schema column types. | `egui::CollapsingHeader` with icon indicators. |
| **SQL Editor (`ui/editor.rs`)** | Multi-line SQL text buffer, cursor positioning, and execute dispatch. | `egui::TextEdit::multiline`. |
| **Results Grid (`ui/grid.rs`)** | High-performance virtualized table rendering visible rows only. | `egui_extras::TableBuilder`. |
| **DuckDB Engine (`db/manager.rs`)** | Wraps `duckdb::Connection`, registers parsed tables, runs queries, and catches SQL errors. | `duckdb` crate with in-memory `:memory:` instance. |
| **Excel Parser (`parsers/excel.rs`)** | Opens `.xlsx`/`.xls` workbooks, inspects sheet names, and extracts cell data with inferred types. | `calamine::open_workbook_auto` and `calamine::Reader`. |
| **CSV Parser (`parsers/csv.rs`)** | Ingests CSV files and maps columns into DuckDB tables or views. | `csv` crate and DuckDB `read_csv_auto`. |
| **Export Service (`export/mod.rs`)** | Formats in-memory query results into disk files (`.csv`, `.xlsx`, `.json`). | `rust_xlsxwriter`, `serde_json`, `csv`. |

## Recommended Project Structure

```text
Single Server SQL Program/
├── Cargo.toml                 # Cargo dependencies & workspace config
├── build.rs                   # Windows application icon embedding
├── src/
│   ├── main.rs                # Native entry point & eframe window launch
│   ├── app.rs                 # Main application state & update loop
│   ├── ui/                    # egui visual components
│   │   ├── mod.rs
│   │   ├── toolbar.rs         # Top action buttons & file picker triggers
│   │   ├── explorer.rs        # Sidebar database/table tree
│   │   ├── editor.rs          # SQL query text editor
│   │   ├── grid.rs            # Virtualized results table builder
│   │   └── theme.rs           # Dark theme palette & font definitions
│   ├── db/                    # DuckDB interaction
│   │   ├── mod.rs
│   │   ├── manager.rs         # Database connection & table registration
│   │   └── types.rs           # QueryResult, ColumnMeta, and DataType models
│   ├── parsers/               # File ingestion
│   │   ├── mod.rs
│   │   ├── excel.rs           # calamine workbook & sheet extraction
│   │   └── csv.rs             # CSV reader & column type inference
│   └── export/                # Result serialization
│       ├── mod.rs
│       ├── csv_export.rs
│       ├── excel_export.rs
│       └── json_export.rs
└── tests/                     # Integration tests
    ├── test_duckdb.rs
    ├── test_excel_parser.rs
    └── test_export.rs
```

## Architectural Patterns

### Pattern 1: Non-Blocking Query Channel
**What:** The GUI thread never executes SQL queries or parses files directly. It dispatches commands over a `crossbeam_channel::Sender` to a background worker thread and polls a `Receiver` on each UI frame.
**When to use:** Every query run and file ingestion operation.
**Trade-offs:** Avoids GUI lockups completely; requires thread-safe message passing (`Arc<Mutex<...>>` or owned channels).

**Example:**
```rust
pub enum WorkerCommand {
    LoadExcel { path: PathBuf },
    LoadCsv { path: PathBuf },
    ExecuteQuery { sql: String },
}

pub enum WorkerResponse {
    TableLoaded { db_name: String, tables: Vec<TableInfo> },
    QueryResult(Result<QueryResultData, String>),
}
```

### Pattern 2: Virtualized Table Rendering (`TableBuilder`)
**What:** Only creates UI widgets for rows and columns that are currently inside the screen viewport.
**When to use:** Rendering the SQL query results grid.
**Trade-offs:** Negligible CPU/GPU usage when scrolling 100,000 rows, compared to creating UI widgets for every cell which causes massive frame drops.

## Anti-Patterns

### Anti-Pattern 1: Synchronous Heavy Parsing on the UI Thread
**What people do:** Reading a 50MB Excel workbook with `calamine` inside `eframe::App::update()`.
**Why it's wrong:** Freezes the Windows window, causing Windows to mark the application as "Not Responding".
**Do this instead:** Spawn file parsing in a worker thread and show a spinner or progress message in the status bar until the worker sends a `TableLoaded` response.

---
*Architecture research for: Local Desktop SQL Workbench (100% Rust)*
*Researched: 2026-10-14*
