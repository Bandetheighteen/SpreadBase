# LocalSQL Studio (Rust)

## What This Is

A blazing-fast, 100% Rust-powered local SQL workbench for querying Excel workbooks (`.xlsx`, `.xls`) and CSV files. Treats each Excel file as a database and each worksheet within it as an individual relational table, executing analytical queries through an embedded DuckDB engine with an immediate-mode GPU-accelerated desktop GUI (`egui`/`eframe`).

## Core Value

Instantly treat any local Excel workbook or CSV file as a relational database and execute SQL queries with zero lag, zero server setup, and zero data leaving the local machine.

## Requirements

### Validated

(None yet — rewrite to 100% Rust to validate)

### Active

- [ ] **100% Pure Rust Architecture**: Standalone native desktop application with zero Python, Node.js, or Electron dependencies, compiled directly with Cargo.
- [ ] **GPU-Accelerated Native GUI**: Immediate-mode interface built with `egui` and `eframe`, providing 60+ FPS responsiveness, dark IDE aesthetics, and instant startup.
- [ ] **Excel as Database Paradigm**: Ingesting an Excel file (`.xlsx`, `.xls`) registers the file as a logical database, automatically exposing all contained sheets as queryable SQL tables.
- [ ] **CSV Table Ingestion**: Ingesting CSV files registers them as directly queryable relational tables.
- [ ] **Embedded DuckDB Engine**: Execute complex SQL (JOINs, aggregations, window functions, CTEs) in-process via Rust `duckdb` bindings.
- [ ] **High-Performance Parsers**: Utilize `calamine` for fast Excel reading and `csv` for multi-megabyte streaming file ingestion.
- [ ] **SQL Code Editor**: Multi-tab SQL editor with syntax coloring, execution triggers (F5/Ctrl+Enter), and execution timing metrics.
- [ ] **Virtual Data Grid**: High-performance results viewer with virtualized row scrolling, column sorting, search filtering, and null-value handling.
- [ ] **Multi-Format Export**: Export query result sets to CSV, Excel (`.xlsx`), JSON, and Parquet.
- [ ] **Comprehensive Test Suite**: Automated unit and integration tests covering parser correctness, DuckDB registration, and query execution.

### Out of Scope

- **Remote Database Connections**: No PostgreSQL, MySQL, or cloud warehouse connections — strictly focused on local CSV and Excel file analysis.
- **Web / Browser Deployment**: No WASM web app or cloud hosting — standalone native desktop application.
- **Source File Mutation / Write-Back**: Query operations are read-only; source Excel and CSV files are never modified in place.
- **Python / Electron Code Maintenance**: Legacy Python and Electron code will be replaced by the Rust implementation.

## Context

- The previous prototype was split between a Python/PySide6 implementation (`app.py`, `ui/`) and an Electron/React rewrite (`LocalSQL-Studio-Next/`), which suffered from UI lag, memory overhead, and dual-codebase fragmentation.
- The user requested a complete rebuild in 100% Rust to achieve maximum responsiveness, stability, and zero lag.
- Rust toolchain (`cargo` 1.99.0, `rustc` 1.99.0) is installed on Windows.

## Constraints

- **Tech Stack**: 100% Rust (`eframe`/`egui`, `duckdb` crate, `calamine`, `csv`). No Python or Node.js runtime dependencies.
- **Operating Model**: Local-first and offline by default. No network requests or telemetry.
- **Platform**: Windows 64-bit native executable.
- **Performance**: Instant UI responsiveness; sub-second query and ingestion times for standard analytical workloads.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rebuild 100% in Rust | Eliminates Python and Electron lag; delivers single-binary distribution and native speed | — Pending |
| GUI with `eframe` / `egui` | Immediate-mode GPU rendering ensures zero UI latency, compact executable size, and pure Rust stack | — Pending |
| Embedded DuckDB Engine | Best-in-class in-process OLAP engine supporting rich SQL features, window functions, and vectorized execution | — Pending |
| `calamine` + `csv` Crates | Pure Rust Excel and CSV parsers providing memory-efficient table ingestion | — Pending |
| Replace in-place | Clean repository root transition directly to Cargo workspace | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-10-14 after initialization*
