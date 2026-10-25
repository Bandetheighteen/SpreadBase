# Requirements: LocalSQL Studio (100% Rust)

**Defined:** 2026-10-14
**Core Value:** Instantly treat any local Excel workbook or CSV file as a relational database and execute SQL queries with zero lag, zero server setup, and zero data leaving the local machine.

## v1 Requirements

Requirements for initial 100% Rust release. Each maps to roadmap phases.

### Ingestion & Dataset Modeling (Excel as Database)

- [ ] **INGEST-01**: User can open an Excel workbook (`.xlsx`, `.xls`) via native file picker or drag-and-drop, registering the file as a logical database.
- [ ] **INGEST-02**: All sheets contained within an ingested Excel workbook are automatically discovered and registered as independent queryable SQL tables.
- [ ] **INGEST-03**: Sheet names and workbook names are sanitized into valid, escaped SQL identifiers (`"{workbook}_{sheet}"`) without colliding with SQL reserved keywords.
- [ ] **INGEST-04**: Column types across Excel sheets are inferred with graceful type reconciliation (coercing heterogeneous mixed columns to `VARCHAR` and blank cells to `NULL`).
- [ ] **INGEST-05**: User can open a `.csv` file, registering it as a standalone queryable table with automatic delimiter and schema inference.

### Embedded SQL Query Engine

- [ ] **ENGINE-01**: Query engine embeds DuckDB via bundled native Rust bindings with zero external database processes or network daemons.
- [ ] **ENGINE-02**: SQL queries execute in-memory with full support for analytical SQL: `SELECT`, `WHERE`, `JOIN` (across multiple sheets and files), `GROUP BY`, `ORDER BY`, `LIMIT`, window functions, and CTEs (`WITH ...`).
- [ ] **ENGINE-03**: Long-running queries and file ingestion run asynchronously on a background worker thread, ensuring the GUI thread never drops below 60 FPS or triggers OS "Not Responding" alerts.
- [ ] **ENGINE-04**: Query execution tracks runtime duration with microsecond precision and returns execution metrics to the user.
- [ ] **ENGINE-05**: SQL syntax errors, invalid column references, or missing table names are caught gracefully and formatted as clear diagnostic messages in the UI.

### Desktop GUI & Immediate-Mode Layout

- [ ] **UI-01**: Desktop interface is implemented 100% in pure Rust using `eframe` and `egui` with native GPU acceleration and sub-100ms startup.
- [ ] **UI-02**: Main window features a responsive three-region IDE layout: Top Toolbar, Left Sidebar Explorer, and Right Editor/Results Workspace.
- [ ] **UI-03**: Visual design adopts a clean, high-density dark IDE theme with consistent typography and visual hierarchy.
- [ ] **UI-04**: Sidebar Explorer dynamically displays all loaded databases (files) and tables (sheets/CSVs) in an expandable tree view with column names and inferred types.
- [ ] **UI-05**: UI operates in reactive mode to maintain near-zero CPU utilization when idle while responding instantaneously to user input.

### SQL Code Editor

- [ ] **EDITOR-01**: Multi-line SQL text editor supporting text selection, indentation, and standard navigation.
- [ ] **EDITOR-02**: User can execute the written SQL query using the toolbar "Run" button or standard keyboard shortcuts (F5 or Ctrl+Enter).
- [ ] **EDITOR-03**: If text is selected in the editor, only the selected SQL fragment is executed; otherwise, the full editor contents are executed.
- [ ] **EDITOR-04**: Double-clicking or selecting a table in the Sidebar Explorer can insert a quick query template (`SELECT * FROM "{table}" LIMIT 100;`).

### Virtual Data Grid & Results Inspection

- [ ] **GRID-01**: Query results are rendered using virtualized scrolling (`egui_extras::TableBuilder`), allowing smooth 60 FPS inspection of 50,000+ rows without memory or frame rate degradation.
- [ ] **GRID-02**: Column headers display column names with proper alignment and horizontal scroll capability for wide datasets.
- [ ] **GRID-03**: NULL values and empty cells are visually distinguished with muted placeholder styling.
- [ ] **GRID-04**: Results panel displays a status bar showing total rows returned, execution time (e.g. `1,250 rows in 3.4ms`), and execution status.

### Multi-Format Export

- [ ] **EXPORT-01**: User can export the current query result set to a local CSV file via native OS save dialog.
- [ ] **EXPORT-02**: User can export the current query result set to an Excel workbook (`.xlsx`) via native OS save dialog using pure-Rust `rust_xlsxwriter`.
- [ ] **EXPORT-03**: User can export the current query result set to formatted JSON.

### Verification & Quality Assurance

- [ ] **VERIFY-01**: Automated unit tests verify Excel parsing with `calamine`, covering multi-sheet extraction and type reconciliation.
- [ ] **VERIFY-02**: Automated unit tests verify DuckDB table registration and SQL execution across simulated Excel and CSV datasets.
- [ ] **VERIFY-03**: End-to-end integration verification tests confirm query execution, error handling, and file export succeed without panics or leaks.
- [ ] **VERIFY-04**: Binary builds cleanly as a single standalone Windows executable (`cargo build --release`).

## v2 Requirements

Deferred to future releases after v1 validation.

- **HIST-01**: Persistent query execution history and saved query bookmarks.
- **GRID-05**: Interactive column sorting and in-grid row filtering directly on the results table headers.
- **PARQ-01**: Native Parquet file ingestion and direct DuckDB Parquet querying.
- **DIAG-01**: Visual ERD relationship diagram between sheets sharing foreign key columns.

## Out of Scope

Explicitly excluded to prevent scope creep and maintain zero-lag focus.

| Feature | Reason |
|---------|--------|
| Remote Database Connections (Postgres/MySQL) | Local-first file analyzer only; remote connections add massive complexity and security overhead. |
| In-Place Excel File Mutation | Read-only analytical queries only; mutating original workbooks risks corrupting formulas, macros, or formatting. |
| Web Browser / Electron Packaging | Explicitly rejected by user in favor of 100% pure Rust compiled binary. |
| Cloud Sync / Telemetry | Strict local-first privacy; no data ever leaves the machine. |

## Traceability

Which phases cover which requirements. Updated to reflect the streamlined 4-phase roadmap.

| Requirement | Phase | Status |
|-------------|-------|--------|
| INGEST-01 | Phase 1 | Pending |
| INGEST-02 | Phase 1 | Pending |
| INGEST-03 | Phase 1 | Pending |
| INGEST-04 | Phase 1 | Pending |
| INGEST-05 | Phase 1 | Pending |
| ENGINE-01 | Phase 1 | Pending |
| ENGINE-02 | Phase 1 | Pending |
| VERIFY-01 | Phase 1 | Pending |
| VERIFY-02 | Phase 1 | Pending |
| UI-01 | Phase 2 | Pending |
| UI-02 | Phase 2 | Pending |
| UI-03 | Phase 2 | Pending |
| UI-04 | Phase 2 | Pending |
| UI-05 | Phase 2 | Pending |
| ENGINE-03 | Phase 2 | Pending |
| EDITOR-01 | Phase 3 | Pending |
| EDITOR-02 | Phase 3 | Pending |
| EDITOR-03 | Phase 3 | Pending |
| EDITOR-04 | Phase 3 | Pending |
| GRID-01 | Phase 3 | Pending |
| GRID-02 | Phase 3 | Pending |
| GRID-03 | Phase 3 | Pending |
| GRID-04 | Phase 3 | Pending |
| ENGINE-04 | Phase 3 | Pending |
| ENGINE-05 | Phase 3 | Pending |
| EXPORT-01 | Phase 4 | Pending |
| EXPORT-02 | Phase 4 | Pending |
| EXPORT-03 | Phase 4 | Pending |
| VERIFY-03 | Phase 4 | Pending |
| VERIFY-04 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 29 total
- Mapped to phases: 29
- Unmapped: 0 ✓

---
*Requirements defined: 2026-10-14*
*Last updated: 2026-10-26 (Streamlined to 4 phases)*
