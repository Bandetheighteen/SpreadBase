# Roadmap: LocalSQL Studio (100% Rust)

## Overview

Rebuilding LocalSQL Studio as a standalone, 100% Rust desktop application. The application delivers an immediate-mode GPU-accelerated workbench (`egui`/`eframe`) powered by an embedded DuckDB analytical engine. It treats Excel workbooks as databases and their sheets as relational tables, enabling lag-free SQL analysis, multi-sheet JOINs, virtualized data inspection, and multi-format export.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3...): Planned milestone work
- Decimal phases (2.1, 2.2...): Urgent insertions (marked with INSERTED)

- [ ] **Phase 1: Rust Core Setup & Embedded DuckDB Engine** - Scaffolding Cargo workspace, bundling DuckDB, and establishing in-memory query execution.
- [ ] **Phase 2: Ingestion Engine (Excel as Database & CSV Tables)** - Implementing `calamine` Excel sheet extraction and CSV parsing with schema and type reconciliation.
- [ ] **Phase 3: Native Desktop GUI & Asynchronous Worker Architecture** - Building `eframe`/`egui` dark IDE shell, sidebar explorer, and non-blocking query channels.
- [ ] **Phase 4: SQL Code Editor & Virtualized Results Data Grid** - Adding multi-line SQL editor with F5 execution, quick table templates, and high-performance `TableBuilder` virtual grid.
- [ ] **Phase 5: Multi-Format Result Export & Native File Dialogs** - Exporting query results to CSV, Excel (`.xlsx`), and JSON via native OS save dialogs.
- [ ] **Phase 6: End-to-End Verification, Performance Testing & Legacy Cleanup** - Comprehensive automated test suite, complex multi-sheet JOIN validation, release packaging, and legacy cleanup.

## Phase Details

### Phase 1: Rust Core Setup & Embedded DuckDB Engine
**Goal**: Establish the Cargo project foundation and configure the embedded DuckDB database engine with in-memory connection and query capabilities.
**Depends on**: Nothing (first phase)
**Requirements**: ENGINE-01, ENGINE-02
**Success Criteria** (what must be TRUE):
  1. `cargo check` and `cargo test` compile cleanly on Windows with bundled DuckDB.
  2. In-memory DuckDB connection can be established, execute arbitrary SQL queries, and return structured columnar results.
  3. Basic unit tests verify SQL syntax execution, aggregations, and error capture.
**Plans**: 2 plans

Plans:
- [ ] 01-01: Scaffold `Cargo.toml` with dependencies (`duckdb`, `serde`, `tempfile`) and project module directory structure (`src/db/`, `src/parsers/`, `src/ui/`, `src/export/`).
- [ ] 01-02: Implement `DbManager` wrapping in-memory DuckDB connection with query execution, schema introspection (`DESCRIBE`), and unit tests.

### Phase 2: Ingestion Engine (Excel as Database & CSV Tables)
**Goal**: Implement fast Excel and CSV file ingestion where each Excel file acts as a database and all contained sheets are exposed as independent relational tables.
**Depends on**: Phase 1
**Requirements**: INGEST-01, INGEST-02, INGEST-03, INGEST-04, INGEST-05
**Success Criteria** (what must be TRUE):
  1. Opening an `.xlsx` file automatically discovers every sheet in the workbook using `calamine`.
  2. Each sheet is registered as an independent table in DuckDB with sanitized identifiers (`"{workbook}_{sheet}"`).
  3. Heterogeneous Excel columns (mixed text/numbers/blanks) are reconciled into safe SQL types without runtime panics.
  4. CSV files are parsed and registered as queryable tables.
  5. Multi-sheet SQL JOINs work seamlessly across registered tables.
**Plans**: 2 plans

Plans:
- [ ] 02-01: Implement `parsers/excel.rs` with `calamine`, sheet discovery, identifier sanitization, and type reconciliation.
- [ ] 02-02: Implement `parsers/csv.rs` and integrate table registration logic into `DbManager` with integration tests.

### Phase 3: Native Desktop GUI & Asynchronous Worker Architecture
**Goal**: Build the 100% Rust desktop interface shell using `eframe`/`egui` with a dark IDE theme, responsive layout, and non-blocking background query channels.
**Depends on**: Phase 2
**Requirements**: UI-01, UI-02, UI-03, UI-04, UI-05, ENGINE-03, ENGINE-04, ENGINE-05
**Success Criteria** (what must be TRUE):
  1. Desktop application launches instantaneously (<100ms) with zero UI lag or dropped frames.
  2. UI features Top Toolbar, Left Sidebar Explorer, and Center Workspace using native `egui` panels.
  3. Sidebar Explorer dynamically lists all loaded databases, tables, and column data types in an expandable tree view.
  4. File parsing and query execution are offloaded to background threads via `crossbeam_channel`, keeping the UI responsive at 60 FPS.
  5. Status bar reports execution time and row counts with microsecond precision.
**Plans**: 2 plans

Plans:
- [ ] 03-01: Implement `eframe::App` shell (`src/app.rs`, `src/ui/theme.rs`, `src/ui/toolbar.rs`) with dark IDE styling and file picker integration via `rfd`.
- [ ] 03-02: Implement background worker thread communication via `crossbeam_channel` and Sidebar Explorer tree widget (`src/ui/explorer.rs`).

### Phase 4: SQL Code Editor & Virtualized Results Data Grid
**Goal**: Implement the central SQL drafting interface and high-performance virtualized results viewer.
**Depends on**: Phase 3
**Requirements**: EDITOR-01, EDITOR-02, EDITOR-03, EDITOR-04, GRID-01, GRID-02, GRID-03, GRID-04
**Success Criteria** (what must be TRUE):
  1. User can type SQL queries into a multi-line editor and execute with F5, Ctrl+Enter, or Run button.
  2. Selecting specific SQL text executes only the selected fragment.
  3. Query results render in a virtualized data grid (`egui_extras::TableBuilder`) capable of scrolling 50,000+ rows at 60 FPS.
  4. NULL values and column headers are clearly styled and formatted.
  5. Clicking a table in the Sidebar inserts a quick template (`SELECT * FROM ... LIMIT 100`).
**Plans**: 2 plans

Plans:
- [ ] 04-01: Implement `ui/editor.rs` with shortcut handling, selection execution, and template insertion.
- [ ] 04-02: Implement `ui/grid.rs` using `egui_extras::TableBuilder` with virtualized row rendering and NULL cell presentation.

### Phase 5: Multi-Format Result Export & Native File Dialogs
**Goal**: Allow users to export any query result set into standard data formats.
**Depends on**: Phase 4
**Requirements**: EXPORT-01, EXPORT-02, EXPORT-03
**Success Criteria** (what must be TRUE):
  1. User can export query results to CSV files via native save dialogs.
  2. User can export query results to formatted `.xlsx` workbooks using pure-Rust `rust_xlsxwriter`.
  3. User can export query results to formatted JSON files.
**Plans**: 1 plan

Plans:
- [ ] 05-01: Implement `src/export/` supporting CSV, Excel (`.xlsx`), and JSON export formats integrated with `rfd` save dialogs.

### Phase 6: End-to-End Verification, Performance Testing & Legacy Cleanup
**Goal**: Thoroughly review, test, and verify the complete application across realistic multi-sheet Excel files, ensuring zero regressions and rock-solid stability before archiving legacy code.
**Depends on**: Phase 5
**Requirements**: VERIFY-01, VERIFY-02, VERIFY-03, VERIFY-04
**Success Criteria** (what must be TRUE):
  1. All automated tests pass (`cargo test`).
  2. End-to-end verification validates loading real multi-sheet Excel workbooks, executing JOIN queries, and exporting results.
  3. Binary compiles in release mode (`cargo build --release`) into a standalone Windows `.exe` with zero Python or Node dependencies.
  4. Outdated legacy files are safely archived and project documentation is updated.
**Plans**: 2 plans

Plans:
- [ ] 06-01: Write comprehensive integration test suite covering edge cases (empty sheets, numeric sheet names, special characters, large datasets).
- [ ] 06-02: Perform manual verification, produce release build, archive legacy Python/Electron code, and finalize documentation.

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Rust Core Setup & Embedded DuckDB Engine | 0/2 | Not started | - |
| 2. Ingestion Engine (Excel as DB & CSV) | 0/2 | Not started | - |
| 3. Native Desktop GUI & Async Worker | 0/2 | Not started | - |
| 4. SQL Code Editor & Virtual Results Grid | 0/2 | Not started | - |
| 5. Multi-Format Result Export | 0/1 | Not started | - |
| 6. Verification, Testing & Legacy Cleanup | 0/2 | Not started | - |
