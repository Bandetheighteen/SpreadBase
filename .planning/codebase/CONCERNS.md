# Codebase Concerns

**Analysis Date:** 2026-10-14

## Tech Debt

**Dual Codebase Split (PySide6 vs. React/Electron):**
- Issue: The repository contains two parallel implementations: a native Python/PySide6 desktop app (`app.py`, `ui/`, `database/`) and an unfinished TypeScript/React/Electron rewrite (`LocalSQL-Studio-Next/`). In addition, `server.py` attempts a third hybrid pattern (Python HTTP server + Tkinter window serving a React `dist` build).
- Files: `app.py`, `server.py`, `LocalSQLStudio.spec`, `LocalSQL-Studio-Next/src/App.tsx`, `LocalSQL-Studio-Next/main.cjs`.
- Impact: Architectural confusion, duplicated feature work, and unclear release targets.
- Fix approach: Explicitly designate one implementation as the primary production target (e.g. either commit to the native PySide6 app or complete the migration to `LocalSQL-Studio-Next`). Deprecate or isolate legacy code.

**Disconnected Workspace State:**
- Issue: `WorkspaceManager` (`workspace/workspace_manager.py`) is fully implemented with methods to persist sources, table mappings, and query history to `workspace.json`, but is never initialized or referenced by `MainWindow` (`ui/main_window.py`).
- Files: `ui/main_window.py:30`, `workspace/workspace_manager.py:4-53`.
- Impact: Registered tables, explorer state, and query histories are completely lost when the user closes the application.
- Fix approach: Integrate `WorkspaceManager` into `MainWindow.__init__`, restore registered tables upon startup, and invoke `log_query()` in `_on_query_success()`.

**Synchronous File Ingestion on GUI Thread:**
- Issue: When users add files via the "Add File" dialog (`ui/main_window.py:143-170`), `self.db.register_csv()` and `self.db.register_excel()` run synchronously on the Qt main thread.
- Files: `ui/main_window.py:157, 166`, `database/duckdb_manager.py:39-44`.
- Impact: Loading a multi-sheet Excel workbook or large CSV freezes the entire UI window until pandas finishes parsing.
- Fix approach: Move file registration and schema inspection into a background `QRunnable` / `QThread` with progress reporting.

## Known Bugs & Fragile Areas

**SQL Identifier and Keyword Collisions:**
- Symptoms: Loading an Excel sheet with spaces, hyphens, or SQL reserved names (e.g., `Order`, `Group`, `Table-1`) can cause query execution errors.
- Files: `database/duckdb_manager.py:35, 43`, `server.py:75, 79`, `LocalSQL-Studio-Next/main.cjs:67`.
- Trigger: A sheet named "Group By" or "2024-Sales" gets transformed to `sheet_group_by` or `sheet_2024_sales`. If an identifier starts with a digit or matches a keyword without quoting, DuckDB syntax parsing fails.
- Fix approach: Quote table identifiers using DuckDB standard double quotes: `CREATE TABLE "{sanitized_table_name}" AS SELECT * FROM df`.

**Single Quote Escaping in File Paths:**
- Symptoms: Failure when loading CSV files located in paths containing single quotes (e.g., `C:/Users/Bob's Files/data.csv`).
- Files: `database/duckdb_manager.py:34-36`.
- Trigger: `file_path.replace("'", "''")` provides basic escaping, but raw string formatting into SQL literals can fail on Windows backslash paths.
- Fix approach: Pass file paths via normalized forward slashes or use DuckDB prepared statements / parameter binding.

## Security Considerations

**Unrestricted SQL Execution on Local Machine:**
- Risk: DuckDB SQL has native capabilities to access the filesystem (`read_csv_auto`, `copy ... to ...`, `ATTACH`).
- Files: `database/duckdb_manager.py:59`, `server.py:49`, `LocalSQL-Studio-Next/src/App.tsx:56`.
- Current mitigation: The application runs locally in offline mode with no external network exposure.
- Recommendations: In `server.py:23-26`, the server binds to `127.0.0.1:8080` with `Access-Control-Allow-Origin: *`. Any website visited in the user's browser could theoretically send cross-origin POST requests to `http://127.0.0.1:8080/api/query` to execute SQL. Restrict CORS origins or enforce an authentication token header if `server.py` is maintained.

## Performance Bottlenecks

**Excel Sheet String Serialization in Electron:**
- Problem: High memory consumption and UI stutter when loading large Excel workbooks in `LocalSQL-Studio-Next`.
- Files: `LocalSQL-Studio-Next/main.cjs:66`, `LocalSQL-Studio-Next/src/App.tsx:87-93`.
- Cause: The Electron main process reads sheets using `xlsx`, converts the entire dataset into an in-memory CSV string (`xlsx.utils.sheet_to_csv`), copies the string across the IPC boundary to the renderer, registers it via `registerFileText`, and parses it into DuckDB-WASM.
- Improvement path: Read Excel directly into binary Arrow buffers or stream chunks through DuckDB-WASM's file system API.

**Pandas Intermediate Duplication in Python:**
- Problem: Large Excel files consume 2x–3x their memory footprint.
- Files: `database/duckdb_manager.py:41-43`.
- Cause: `pd.read_excel()` builds a full pandas DataFrame in RAM, then `CREATE TABLE {table_name} AS SELECT * FROM df` copies the data into DuckDB buffers.
- Improvement path: Use DuckDB's spatial / excel extension (`INSTALL spatial; LOAD spatial;` or native Parquet/CSV pipelines) or stream sheets in batches.

## Scaling Limits

**DuckDB-WASM Memory Ceiling:**
- Current capacity: Browser WebAssembly heap limit (typically 2GB–4GB).
- Limit: Datasets exceeding ~1.5GB of raw CSV/Excel data will crash the WASM worker with Out-Of-Memory (OOM) errors.
- Scaling path: For large datasets (>1GB), use the native Python engine (`database/duckdb_manager.py`) which can query directly from disk without WASM heap limits.

## Dependencies at Risk

**SheetJS (`xlsx` 0.18.5) Vulnerabilities & Licensing:**
- Risk: The version of `xlsx` on npm (`0.18.5`) has known vulnerabilities and is no longer actively maintained on the public npm registry (newer releases are hosted on SheetJS CDN).
- Files: `LocalSQL-Studio-Next/package.json:26`.
- Impact: Potential security advisories during `npm audit`.
- Migration plan: Consider using `exceljs`, native DuckDB Excel loading, or updated SheetJS distributions.

**DuckDB-WASM Pre-Release Version:**
- Risk: `@duckdb/duckdb-wasm` is pinned to dev version `^1.33.1-dev57.0`.
- Files: `LocalSQL-Studio-Next/package.json:18`.
- Impact: Dev builds can exhibit instability or breaking API changes across minor updates.
- Migration plan: Pin to a stable release of `@duckdb/duckdb-wasm`.

## Missing Critical Features

**Table & View Schema Introspection in UI:**
- Problem: `ExplorerPanel` in `ui/explorer.py` has tree placeholders for "Sources", "Tables", "Views", and "Queries", but does not dynamically populate tables when files are loaded in `ui/main_window.py`.
- Blocks: Users cannot click a table in the sidebar to view columns or generate `SELECT * FROM table LIMIT 100` templates.

**Query Cancellation:**
- Problem: Once a long-running SQL query is dispatched to DuckDB, the user has no way to cancel execution; the "Run" button is simply disabled until completion.
- Blocks: Users can hang the query worker if they execute heavy Cartesian joins or infinite loops.

## Test Coverage Gaps

**Untested Core Layers:**
- What's not tested: 100% of the codebase.
- Files: `database/duckdb_manager.py`, `sources/file_parser.py`, `export/exporter.py`, `workspace/workspace_manager.py`, `ui/*.py`, `LocalSQL-Studio-Next/src/App.tsx`.
- Risk: Regressions in schema inference, data type mapping, SQL generation, and export fidelity can occur unnoticed during refactoring.

---

*Concerns analysis: 2026-10-14*
