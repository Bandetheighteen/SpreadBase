---
last_mapped_commit: bd6c25b37e18be419e3d1a63a4fd11d4ec501580
last_mapped_at: 2026-10-25
---
<!-- refreshed: 2026-10-25 -->

# Codebase Structure

**Analysis Date:** 2026-10-25

## Directory Layout

```text
Single Server SQL Program/
├── Cargo.toml                 # Rust package manifest (dependencies, targets, profiles)
├── Cargo.lock                 # Locked dependency graph for reproducible builds
├── src/                       # Core 100% Rust architecture
│   ├── main.rs                # Desktop application binary entry point
│   ├── lib.rs                 # Library target exposing core modules for app and integration tests
│   ├── db/                    # Embedded DuckDB engine, connections & worker threads
│   │   └── mod.rs             # Database management module definition
│   ├── parsers/               # Tabular data ingestion parsers
│   │   └── mod.rs             # Excel (calamine) and CSV parsing module
│   ├── ui/                    # Immediate-mode desktop GUI (egui / eframe)
│   │   └── mod.rs             # Desktop application shell, editor tabs & virtual table
│   └── export/                # Multi-format result serialisation
│       └── mod.rs             # Excel (rust_xlsxwriter), CSV, JSON export module
├── target/                    # Cargo build cache and binary output directory
├── Concept.txt                # Comprehensive product design & architecture specification
│
│── [Legacy Python Architecture - Deprecated]
├── app.py                     # Legacy PySide6 desktop entry point
├── server.py                  # Standalone HTTP + Tkinter background server
├── requirements.txt           # Legacy Python dependencies (PySide6, duckdb, pandas)
├── LocalSQLStudio.spec        # Legacy PyInstaller packaging configuration
├── database/                  # Legacy Python DuckDB wrapper
│   ├── __init__.py
│   └── duckdb_manager.py      # DuckDB connection & async QueryWorker
├── ui/                        # Legacy PySide6 GUI widgets
│   ├── __init__.py
│   ├── main_window.py         # Main IDE application window & splitter layout
│   ├── explorer.py            # Left sidebar tree explorer for sources & tables
│   ├── sql_editor.py          # Tabbed SQL editor container with actions
│   ├── line_number_area.py    # Custom code editor with gutter & line numbers
│   ├── syntax_highlighter.py  # SQL syntax colorization rules
│   └── results_grid.py        # TableView with virtual model, sorting & filtering
├── utils/                     # Legacy UI utilities, styling & icons
│   ├── __init__.py
│   ├── theme.py               # VS Code dark palette & global QSS stylesheet
│   └── icon_manager.py        # In-memory SVG to QIcon factory
├── sources/                   # Legacy dataset inspection & file parsing
│   ├── __init__.py
│   ├── file_parser.py         # Excel workbook inspection & schema inference
│   └── LOGO.png               # Application icon asset
├── export/                    # Legacy data export formatting
│   ├── __init__.py
│   └── exporter.py            # CSV, Excel, JSON, and Parquet serializers
├── workspace/                 # Legacy application state persistence
│   ├── __init__.py
│   └── workspace_manager.py   # JSON-backed configuration & history tracking
│
│── [Legacy Web/Electron Architecture - Deprecated]
├── LocalSQL-Studio-Next/      # Legacy React 19 + TypeScript + Electron app
│   ├── package.json           # Node.js dependencies & scripts
│   ├── main.cjs               # Electron main process
│   ├── preload.cjs            # Electron preload context isolation bridge
│   ├── index.html             # Vite entry HTML
│   ├── vite.config.ts         # Vite bundler configuration
│   └── src/
│       ├── main.tsx           # React DOM root
│       └── App.tsx            # Main workbench with DuckDB-WASM & Monaco editor
│
│── [Project Planning & Research]
├── .planning/                 # GSD planning directory (ROADMAP, STATE, phases, codebase)
├── wiki/                      # Knowledge base & technical research notes
├── build/                     # Legacy PyInstaller temporary build artifacts
├── dist/                      # Legacy PyInstaller binary output
└── raw/                       # Staging directory for test datasets
```

## Directory Purposes

**`src/`:**
- Purpose: Active core codebase for the 100% Rust implementation.
- Contains: Application binary entry (`main.rs`), public library interface (`lib.rs`), and modular domain subsystems.
- Key subdirectories:
  - `src/db/`: In-process DuckDB engine management, connection pooling, and background worker threads.
  - `src/parsers/`: Calamine Excel reader and CSV streaming ingestion.
  - `src/ui/`: Immediate-mode desktop interface built with `eframe` and `egui`.
  - `src/export/`: Output serializers generating `.xlsx`, `.csv`, and `.json`.

**`target/`:**
- Purpose: Cargo compilation cache, intermediate object files, and compiled binaries.
- Key output: `target/release/localsql_studio.exe`.

**`database/`, `ui/`, `sources/`, `export/`, `utils/`, `workspace/` (Legacy):**
- Purpose: Legacy Python 3 / PySide6 implementation and prototypes. Scheduled for cleanup in Phase 6.

**`LocalSQL-Studio-Next/` (Legacy):**
- Purpose: Legacy Electron / React 19 / DuckDB-WASM prototype. Scheduled for cleanup in Phase 6.

**`.planning/`:**
- Purpose: Getting Shit Done (GSD) project lifecycle tracking, milestone requirements, phase plans, and codebase intel.

## Key File Locations

**Entry Points:**
- `src/main.rs`: Rust native binary entry point.
- `src/lib.rs`: Rust library root exposing `db`, `parsers`, `ui`, `export`.
- `app.py`: Legacy PySide6 desktop entry point.
- `server.py`: Legacy Python HTTP / Tkinter server.
- `LocalSQL-Studio-Next/main.cjs`: Legacy Electron entry point.

**Configuration:**
- `Cargo.toml`: Active Cargo build configuration, dependencies, and compiler profiles.
- `requirements.txt`: Legacy Python requirements.
- `LocalSQLStudio.spec`: Legacy PyInstaller build spec.
- `LocalSQL-Studio-Next/package.json`: Legacy Node.js package definition.

**Core Logic:**
- `src/db/mod.rs`: Rust DuckDB engine wrapper and worker dispatch.
- `src/parsers/mod.rs`: Rust Excel and CSV file parsers.
- `src/ui/mod.rs`: Rust `egui`/`eframe` GUI application shell.
- `src/export/mod.rs`: Rust multi-format data export.

## Naming Conventions

**Rust:**
- File names: `snake_case.rs` (e.g., `lib.rs`, `main.rs`, `mod.rs`).
- Module names: `snake_case` (`db`, `parsers`, `ui`, `export`).
- Structs, Enums, Traits: `PascalCase` (e.g., `DuckEngine`, `QueryCommand`, `QueryResult`).
- Functions & Methods: `snake_case` (e.g., `execute_query`, `register_excel_sheet`).
- Constants & Statics: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_WINDOW_WIDTH`).
- Macros: `snake_case!` (e.g., `vec!`, `println!`).

**Legacy Python:**
- Modules: `snake_case.py`.
- Classes: `PascalCase`.
- Methods: `snake_case`.

**Legacy TypeScript:**
- Components: `PascalCase.tsx`.
- Utilities: `camelCase.ts` or `kebab-case.cjs`.

## Where to Add New Code

**New Database / Query Feature (Rust):**
- Primary location: `src/db/mod.rs` (or submodules like `src/db/connection.rs`, `src/db/worker.rs`).
- Public interface: Re-export in `src/db/mod.rs` and `src/lib.rs`.

**New File Ingestion Parser (Rust):**
- Primary location: `src/parsers/mod.rs` (or `src/parsers/excel.rs`, `src/parsers/csv.rs`).
- Integration: Hook parser results into `src/db/mod.rs` table registration.

**New UI Panel or Control (Rust):**
- Primary location: `src/ui/mod.rs` (or `src/ui/editor.rs`, `src/ui/table.rs`, `src/ui/sidebar.rs`).
- Integration: Add rendering subroutines into `eframe::App::update()`.

**New Export Format (Rust):**
- Primary location: `src/export/mod.rs`.
- Integration: Add format serializer and wire to UI export menu.

**Integration Tests:**
- Primary location: `tests/` directory at repository root (e.g., `tests/test_duckdb.rs`, `tests/test_parsers.rs`).

## Special Directories

**`target/`:**
- Generated by Cargo during build and test runs.
- Excluded via `.gitignore`.

**`dist/` & `build/`:**
- Generated by PyInstaller for legacy Python binaries.
- Excluded via `.gitignore`.

**`LocalSQL-Studio-Next/node_modules/` & `venv/`:**
- Third-party packages for legacy runtimes.
- Excluded via `.gitignore`.

---

*Structure analysis: 2026-10-25*
