# Project Research Summary

**Project:** LocalSQL Studio (100% Rust)
**Domain:** Local-First Desktop SQL Workbench & Embedded Analytical Engine
**Researched:** 2026-10-14
**Confidence:** HIGH

## Executive Summary

This research confirms the viability and optimal technical architecture for rebuilding LocalSQL Studio 100% in Rust. By discarding the multi-language hybrid approach (Python/PySide6 and Electron/React), the application will eliminate all UI lag, memory bloat, and runtime fragility.

The recommended stack pairs an immediate-mode GPU-accelerated GUI (`eframe`/`egui`) with an embedded analytical database engine (`duckdb-rs`) and specialized high-throughput file parsers (`calamine` for Excel, `csv` for CSV). A background worker thread with `crossbeam_channel` isolates database operations from the 60+ FPS UI loop.

Key risks—namely heterogeneous cell types in Excel workbooks, SQL identifier collisions, and MSVC toolchain requirements—are mitigated through robust type reconciliation, sanitized identifier quoting, and static bundling.

## Key Findings

### Recommended Stack
- **Application & GUI:** Rust 1.99.0+ with `eframe` and `egui` (v0.29.1) providing native GPU-rendered desktop interface with zero web dependencies.
- **Query Engine:** `duckdb` (v1.1.1) with the `bundled` feature for in-process vectorized analytical SQL execution without requiring external database servers.
- **Parsers:** `calamine` (v0.26.1) for zero-dependency Excel workbook and sheet extraction; `csv` (v1.3.0) for high-speed streaming CSV ingestion.
- **Table Virtualization & Export:** `egui_extras` (`TableBuilder`) for smooth scrolling of 50,000+ rows; `rust_xlsxwriter` and `serde_json` for export.

### Expected Features
- **Table Stakes:** Ingest Excel files as databases with sheets as tables; ingest CSV files as tables; sidebar schema explorer; multi-line SQL editor with F5 execution; virtualized data grid; error banners; multi-format export.
- **Differentiators:** Zero-lag native performance (<30MB RAM, <100ms startup); cross-sheet and cross-file SQL JOINs; advanced window functions and CTEs.
- **Anti-Features:** In-place Excel file mutation (strictly read-only analytical SQL); embedded browser WebViews (strictly 100% pure Rust).

### Architecture Approach
A clean separation of concerns:
1. `ui/`: `eframe::App` immediate-mode rendering with reactive frame updates.
2. `db/`: Thread-safe DuckDB manager running queries asynchronously.
3. `parsers/`: Pure-Rust Excel and CSV readers mapping files to DuckDB tables.
4. `export/`: Multi-format serializers for writing query results to disk.

### Critical Pitfalls
1. **Heterogeneous Excel Column Types:** Coerce mixed columns to `VARCHAR` and nullify empty cells to avoid ingestion errors.
2. **Unescaped SQL Identifiers:** Sanitize sheet names and wrap table names in double quotes (`"{table}"`).
3. **Synchronous UI Freezes:** Always parse files and execute queries on a background worker thread.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Rust Workspace Scaffolding & DuckDB Engine Foundation
- **Rationale:** Verify C++ toolchain and compile bundled DuckDB with Rust bindings before UI work.
- **Delivers:** `Cargo.toml`, core project structure, in-memory DuckDB manager, and unit test suite verifying SQL execution.
- **Avoids:** Toolchain and compilation pitfalls early.

### Phase 2: Ingestion Engine (Excel as Database & CSV Tables)
- **Rationale:** Implement the core business model: Excel workbook = Database, Sheets = Tables.
- **Delivers:** `parsers/excel.rs` (`calamine`) and `parsers/csv.rs`, table name sanitization, type reconciliation, and DuckDB registration.
- **Avoids:** Type mismatches and identifier syntax errors.

### Phase 3: Native Desktop GUI & Layout (eframe / egui)
- **Rationale:** Build the visual shell and reactive UI structure.
- **Delivers:** Main window, toolbar, left sidebar Explorer (listing databases, tables, columns), and SQL editor text area.
- **Avoids:** Idle CPU spin through egui reactive mode.

### Phase 4: Query Dispatch, Virtual Data Grid & Error Handling
- **Rationale:** Connect the SQL editor to DuckDB asynchronously and render results without lag.
- **Delivers:** `crossbeam_channel` worker thread, `egui_extras::TableBuilder` virtualized results grid, query runtime profiler, and diagnostic error banners.
- **Avoids:** UI thread blocking during heavy queries.

### Phase 5: Export Subsystem & Polish
- **Rationale:** Allow saving query results in standard analytical formats.
- **Delivers:** Export to CSV, Excel (`.xlsx`), JSON, and Parquet via native Windows file dialogs (`rfd`).

### Phase 6: Verification, Testing & Legacy Cleanup
- **Rationale:** Deliver a rock-solid, fully tested application to the user.
- **Delivers:** Automated test suite, manual end-to-end verification with sample files, and archiving/removal of laggy legacy code.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Cargo, eframe, duckdb, and calamine are standard in Rust ecosystem |
| Features | HIGH | Directly maps to user's audio request and proven workbench patterns |
| Architecture | HIGH | Established worker-channel pattern in immediate-mode GUIs |
| Pitfalls | HIGH | Specific gotchas identified with concrete mitigations |

**Overall confidence:** HIGH

---
*Research completed: 2026-10-14*
*Ready for roadmap: yes*
