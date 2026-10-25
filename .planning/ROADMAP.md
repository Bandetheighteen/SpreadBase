# Roadmap: LocalSQL Studio (100% Rust)

## Overview

A standalone, 100% Rust desktop application that lets you query local Excel workbooks (`.xlsx`, `.xls`) and CSV files like databases. It treats each file as a database, each sheet inside as a table, and column headers as variables. You type SQL queries to fetch data, and click a button to export the results back into an Excel or CSV file.

## Phases

- [x] **Phase 1: Ingestion & Embedded SQL Engine** - Build the behind-the-scenes engine that reads Excel and CSV files, turns every sheet into a table (with column headers as variables), and runs SQL queries in memory.
- [ ] **Phase 2: Desktop Window Shell & File Picker** - Create the desktop window with dark styling, a button to open one or multiple files, and a left sidebar listing all loaded files, sheets, and columns.
- [ ] **Phase 3: SQL Query Box & Live Results Table** - Add the query editor box to type SQL, a "Run" button (and F5 shortcut), and a fast table that displays the fetched rows and columns.
- [ ] **Phase 4: Export Button, End-to-End Testing & Cleanup** - Add the export button to save query results into an Excel or CSV file, verify the full workflow end-to-end, and remove outdated prototype files.

## Phase Details

### Phase 1: Ingestion & Embedded SQL Engine
**Goal**: Build the core data engine that runs in memory, reads Excel/CSV files, exposes sheets as relational tables with column headers as fields, and executes SQL queries.
**Depends on**: Nothing (first phase)
**Requirements**: ENGINE-01, ENGINE-02, INGEST-01, INGEST-02, INGEST-03, INGEST-04, INGEST-05, VERIFY-01, VERIFY-02
**Success Criteria** (what must be TRUE):
  1. In-memory database engine starts and executes SQL queries (`SELECT`, `WHERE`, `JOIN`, `GROUP BY`).
  2. Opening an Excel file (`.xlsx`, `.xls`) reads all sheets and registers each sheet as an independent table.
  3. Column headers in sheets become table columns/variables; blank cells and mixed data types are handled safely.
  4. CSV files are read and registered as queryable tables.
  5. Multi-sheet SQL JOINs work smoothly across sheets.
  6. Automated tests pass (`cargo test`).
**Plans**: 2 plans

Plans:
- [x] 01-01: Implement in-memory database manager and SQL data types (`src/db/`).
- [x] 01-02: Implement Excel (`calamine`) and CSV ingestion parsers (`src/parsers/`) with table registration.

### Phase 2: Desktop Window Shell & File Picker
**Goal**: Build the responsive desktop application window, the file selection dialog, and the sidebar tree showing loaded files and tables.
**Depends on**: Phase 1
**Requirements**: UI-01, UI-02, UI-03, UI-04, UI-05, ENGINE-03
**Success Criteria** (what must be TRUE):
  1. Desktop app window launches instantly with clean dark theme layout.
  2. Clicking "Open File" allows picking one or multiple Excel and CSV files.
  3. Left sidebar displays loaded files, their tables (sheets), and column names.
  4. File reading and queries run in the background so the window never freezes or lags.
**Plans**: 2 plans

Plans:
- [ ] 02-01: Build the desktop application window shell and dark styling (`src/ui/`).
- [ ] 02-02: Wire native file picker (`rfd`), background thread channel, and sidebar table list.

### Phase 3: SQL Query Box & Live Results Table
**Goal**: Provide the interactive workspace where users can type SQL queries, click "Run" (or press F5), and view results in a fast table.
**Depends on**: Phase 2
**Requirements**: EDITOR-01, EDITOR-02, EDITOR-03, EDITOR-04, GRID-01, GRID-02, GRID-03, GRID-04, ENGINE-04, ENGINE-05
**Success Criteria** (what must be TRUE):
  1. User can type or edit SQL queries in a multi-line query box.
  2. Clicking "Run" (or pressing F5 / Ctrl+Enter) executes the query.
  3. Clicking a table in the sidebar inserts a template (`SELECT * FROM "{table}" LIMIT 100`).
  4. Results render smoothly in a table with row counts and execution timing.
  5. Any SQL typos or errors are shown clearly without crashing the app.
**Plans**: 2 plans

Plans:
- [ ] 03-01: Implement the SQL query text box with execution triggers and table templates.
- [ ] 03-02: Implement the virtualized results table grid and execution status bar.

### Phase 4: Export Button, End-to-End Testing & Cleanup
**Goal**: Add the export button to save query results to Excel or CSV, verify the whole app from end to end, and remove old prototype code.
**Depends on**: Phase 3
**Requirements**: EXPORT-01, EXPORT-02, EXPORT-03, VERIFY-03, VERIFY-04
**Success Criteria** (what must be TRUE):
  1. User can click an Export button to save query results as an Excel (`.xlsx`) or CSV file.
  2. End-to-end test validates the complete flow: open file -> run SQL -> see results -> export file.
  3. Standalone Windows `.exe` compiles with zero extra runtime requirements.
  4. Outdated Python and Electron files are archived and cleaned up.
**Plans**: 2 plans

Plans:
- [ ] 04-01: Implement result export button for Excel (`rust_xlsxwriter`) and CSV via save dialog.
- [ ] 04-02: End-to-end integration testing, release executable build, and cleanup of legacy prototypes.

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Ingestion & Embedded SQL Engine | 0/2 | Not started | - |
| 2. Desktop Window Shell & File Picker | 0/2 | Not started | - |
| 3. SQL Query Box & Live Results Table | 0/2 | Not started | - |
| 4. Export Button, Verification & Cleanup | 0/2 | Not started | - |
