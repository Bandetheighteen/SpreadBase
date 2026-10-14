# Codebase Structure

**Analysis Date:** 2026-10-14

## Directory Layout

```text
Single Server SQL Program/
├── app.py                     # Entry point for native PySide6 desktop application
├── server.py                  # Standalone HTTP + Tkinter background server
├── requirements.txt           # Python dependencies (PySide6, duckdb, pandas, etc.)
├── LocalSQLStudio.spec        # PyInstaller packaging configuration
├── Concept.txt                # Comprehensive product design & architecture specification
├── database/                  # DuckDB engine wrapper and worker thread management
│   ├── __init__.py
│   └── duckdb_manager.py      # DuckDB connection & async QueryWorker
├── ui/                        # PySide6 graphical user interface widgets
│   ├── __init__.py
│   ├── main_window.py         # Main IDE application window & splitter layout
│   ├── explorer.py            # Left sidebar tree explorer for sources & tables
│   ├── sql_editor.py          # Tabbed SQL editor container with actions
│   ├── line_number_area.py    # Custom code editor with gutter & line numbers
│   ├── syntax_highlighter.py  # SQL syntax colorization rules
│   └── results_grid.py        # TableView with virtual model, sorting & filtering
├── utils/                     # UI utilities, styling & icons
│   ├── __init__.py
│   ├── theme.py               # VS Code dark palette & global QSS stylesheet
│   └── icon_manager.py        # In-memory SVG to QIcon factory
├── sources/                   # Dataset inspection & file parsing
│   ├── __init__.py
│   ├── file_parser.py         # Excel workbook inspection & schema inference
│   └── LOGO.png               # Application icon & brand asset
├── export/                    # Data export formatting
│   ├── __init__.py
│   └── exporter.py            # CSV, Excel, JSON, and Parquet serializers
├── workspace/                 # Application state & session persistence
│   ├── __init__.py
│   └── workspace_manager.py   # JSON-backed configuration & history tracking
├── wiki/                      # Knowledge base & technical documentation
│   ├── index.md               # Knowledge base table of contents
│   ├── log.md                 # Archival log of research topics
│   └── database-tools/        # Deep-dive research notes on local SQL tools
│       ├── local-sql-on-csv-excel.md
│       └── go-language-cli-tools-for-local-sql.md
├── LocalSQL-Studio-Next/      # Next-generation React 19 + TypeScript + Electron app
│   ├── package.json           # Node.js dependencies & scripts
│   ├── main.cjs               # Electron main process (IPC & window management)
│   ├── preload.cjs            # Electron preload context isolation bridge
│   ├── index.html             # Vite entry HTML
│   ├── vite.config.ts         # Vite bundler configuration
│   ├── tailwind.config.cjs    # Tailwind CSS design system configuration
│   ├── tsconfig.json          # TypeScript project references
│   └── src/
│       ├── main.tsx           # React DOM root initialization
│       ├── App.tsx            # Main workbench with DuckDB-WASM & Monaco editor
│       ├── index.css          # Tailwind CSS global styles
│       └── App.css            # Component-specific styles
├── build/                     # PyInstaller temporary build artifacts
├── dist/                      # PyInstaller binary output (`LocalSQLStudio.exe`)
└── raw/                       # Staging directory for raw user datasets
```

## Directory Purposes

**`database/`:**
- Purpose: DuckDB database interaction and concurrency management.
- Contains: In-process database connector, table/view registration routines, and thread-safe worker execution.
- Key files: `database/duckdb_manager.py`.

**`ui/`:**
- Purpose: All visual desktop presentation components implemented with PySide6.
- Contains: Layout containers, custom widgets, tab managers, code editors, and table grids.
- Key files: `ui/main_window.py`, `ui/sql_editor.py`, `ui/results_grid.py`, `ui/explorer.py`.

**`sources/`:**
- Purpose: File discovery, validation, and schema inference.
- Contains: Parsers for inspecting Excel sheet names and mapping column types to SQL equivalents.
- Key files: `sources/file_parser.py`.

**`export/`:**
- Purpose: Exporting query results to standard file formats.
- Contains: DataFrame conversion logic for `.csv`, `.xlsx`, `.json`, and `.parquet`.
- Key files: `export/exporter.py`.

**`workspace/`:**
- Purpose: Session persistence and history catalogue.
- Contains: Logic for loading and saving `workspace.json` (registered files, logical tables, query history).
- Key files: `workspace/workspace_manager.py`.

**`utils/`:**
- Purpose: Reusable application-wide UI utilities.
- Contains: Dark theme styling, color tokens, and SVG icon generators.
- Key files: `utils/theme.py`, `utils/icon_manager.py`.

**`wiki/`:**
- Purpose: Project documentation and domain research notes.
- Contains: Markdown files analyzing competitive local database tools, CLI architectures, and research logs.
- Key files: `wiki/index.md`, `wiki/database-tools/local-sql-on-csv-excel.md`.

**`LocalSQL-Studio-Next/`:**
- Purpose: Modern rewrite utilizing React 19, TypeScript, DuckDB-WASM, and Electron.
- Contains: Full frontend SPA and Electron desktop distribution pipeline.
- Key files: `LocalSQL-Studio-Next/src/App.tsx`, `LocalSQL-Studio-Next/main.cjs`.

## Key File Locations

**Entry Points:**
- `app.py`: PySide6 Native Desktop App entry point.
- `server.py`: Python background HTTP and Tkinter web server.
- `LocalSQL-Studio-Next/main.cjs`: Electron desktop entry point.
- `LocalSQL-Studio-Next/src/main.tsx`: React web app entry point.

**Configuration:**
- `requirements.txt`: Python package requirements.
- `LocalSQLStudio.spec`: PyInstaller executable compilation configuration.
- `LocalSQL-Studio-Next/package.json`: Node dependencies, build scripts, electron-builder specs.
- `LocalSQL-Studio-Next/tsconfig.json`: TypeScript compiler options.
- `LocalSQL-Studio-Next/tailwind.config.cjs`: Tailwind CSS theme configuration.

**Core Logic:**
- `database/duckdb_manager.py`: Native query execution and view/table registration.
- `sources/file_parser.py`: Schema deduction from raw datasets.
- `workspace/workspace_manager.py`: Catalogue state management.
- `LocalSQL-Studio-Next/src/App.tsx`: DuckDB-WASM worker initialization and query running.

**Testing:**
- Currently no automated unit test files are located in root or `LocalSQL-Studio-Next/` (see `TESTING.md` and `CONCERNS.md`).

## Naming Conventions

**Files:**
- Python modules: `snake_case.py` (e.g., `main_window.py`, `duckdb_manager.py`, `file_parser.py`).
- TypeScript / React: `kebab-case` for config (`tsconfig.app.json`), `PascalCase` for React components (`App.tsx`), `camelCase` / `kebab-case` for CJS scripts (`main.cjs`, `preload.cjs`).
- Documentation: `UPPERCASE.md` for planning, `kebab-case.md` for wiki articles (`local-sql-on-csv-excel.md`).

**Directories:**
- Python packages: `lowercase` (`database/`, `ui/`, `sources/`, `export/`, `workspace/`, `utils/`).
- Web modules: `kebab-case` (`database-tools/`, `LocalSQL-Studio-Next/`).

**Code Identifiers:**
- Python Classes: `PascalCase` (`MainWindow`, `DuckDBManager`, `SqlCodeEditor`, `VirtualTableModel`).
- Python Functions/Methods: `snake_case` (`run_query`, `register_csv`, `get_excel_sheets`).
- Python Constants: `SCREAMING_SNAKE_CASE` (`BACKGROUND`, `PRIMARY_ACCENT`).
- TypeScript / React: `camelCase` for functions/hooks (`runQuery`, `initDB`, `loadFile`), `PascalCase` for React components (`App`).

## Where to Add New Code

**New Database / Query Feature (Python):**
- Primary code: Add methods to `database/duckdb_manager.py` (e.g., query cancellation, export via DuckDB `COPY`, transaction handling).
- UI Integration: Hook methods in `ui/main_window.py` or `ui/sql_editor.py`.

**New UI Panel or Tab (Python):**
- Implementation: Create new widget in `ui/` (e.g., `ui/schema_viewer.py`, `ui/query_history_panel.py`).
- Integration: Add widget to splitter or tab in `ui/main_window.py`.

**New Export Format:**
- Implementation: Add static serializer method in `export/exporter.py` (e.g., `Exporter.export_to_sql()`).
- UI Trigger: Add file filter option in `ui/main_window.py:export_results()`.

**New Feature (LocalSQL-Studio-Next):**
- React UI components: Create modular components in `LocalSQL-Studio-Next/src/components/` (e.g., `Editor.tsx`, `Sidebar.tsx`, `DataGrid.tsx`).
- Native capabilities: Add IPC handlers in `LocalSQL-Studio-Next/main.cjs` and expose via `LocalSQL-Studio-Next/preload.cjs`.

## Special Directories

**`dist/` & `build/`:**
- Purpose: Generated by PyInstaller when building `LocalSQLStudio.exe`.
- Generated: Yes.
- Committed: Recommended for `.gitignore` (contains 100MB+ binary `LocalSQLStudio.exe`).

**`LocalSQL-Studio-Next/release/`:**
- Purpose: Generated by `electron-builder` when packaging Windows installers.
- Generated: Yes.
- Committed: Recommended for `.gitignore` (contains ~118MB executable installers).

**`LocalSQL-Studio-Next/node_modules/` & `venv/`:**
- Purpose: Third-party package dependencies for Node and Python.
- Generated: Yes.
- Committed: Excluded via `.gitignore`.

---

*Structure analysis: 2026-10-14*
