---
last_mapped_commit: bd6c25b37e18be419e3d1a63a4fd11d4ec501580
last_mapped_at: 2026-10-25
---
<!-- refreshed: 2026-10-25 -->

# External Integrations

**Analysis Date:** 2026-10-25

## APIs & External Services

**Local-First Design:**
- In accordance with the core design specification (`Concept.txt`) and project requirements (`.planning/PROJECT.md`), the application operates completely offline.
- No third-party cloud services, remote telemetry endpoints, or SaaS analytics APIs are integrated.
- All query operations, file ingestion, and analytical transformations remain strictly within the local process space and memory.

## Data Storage

**Databases:**
- **DuckDB (Embedded In-Process Engine — Rust):**
  - Engine: Embedded DuckDB v1.1.1 (`duckdb` crate with `bundled` feature).
  - Connection: `duckdb::Connection::open_in_memory()` running in background worker thread.
  - Driver/Client: Native Rust C bindings compiled directly into the binary.
  - Table Registration: Ingests sheets and files as in-memory DuckDB tables and views with zero network latency.
- **DuckDB (Legacy Python Prototype):**
  - Engine: DuckDB Python module (`database/duckdb_manager.py:27`).
  - Connection: `duckdb.connect(':memory:')`.
  - Table Registration: `read_csv_auto('{file_path}')` for CSVs; loads Excel sheets into pandas DataFrames and executes `CREATE TABLE {table_name} AS SELECT * FROM df`.
- **DuckDB-WASM (Legacy Web/Electron Prototype):**
  - Engine: `@duckdb/duckdb-wasm` running inside a WebAssembly Web Worker (`LocalSQL-Studio-Next/src/App.tsx`).

**File Storage:**
- **Local Filesystem Only:**
  - Ingestion: Reads local `.xlsx`, `.xls`, and `.csv` files selected by the user via native OS dialogs (`rfd::FileDialog` in Rust, `QFileDialog` in Python, Electron IPC dialog).
  - Storage Paradigm: Treats each Excel workbook as a logical database and each individual sheet as a queryable relational table.
  - Export Target: Writes query output datasets to local disk in `.csv`, `.xlsx` (via `rust_xlsxwriter`), and `.json` formats.

**Caching & Persistence:**
- Pure in-memory query execution. Materialized views and tables exist within DuckDB memory buffers during process lifetime.
- Legacy workspace session persistence existed via `workspace/workspace_manager.py` targeting `workspace.json`.

## Authentication & Identity

**Auth Provider:**
- None. This is a local desktop analytical workbench with single-user execution and no identity providers, tokens, or network logins.

## Monitoring & Observability

**Error Tracking:**
- **Rust Core:** `Result<T, E>` pattern propagating errors to GUI state, surfaced directly in the UI messages panel and status bar.
- **Python / PySide6 App:** Exception handling in `database/duckdb_manager.py` emits errors via Qt signals (`QueryWorkerSignals.error`), displayed in `ui/results_grid.py:110-115`.
- **React / Electron App:** React state error banner (`LocalSQL-Studio-Next/src/App.tsx:66-67`).
- **External Error Reporting:** None (no Sentry, Datadog, or external telemetry).

**Logs:**
- Standard console streams (`stdout`, `stderr`).
- Diagnostic messages in UI status bar and output panels.

## CI/CD & Deployment

**Hosting & Distribution:**
- Distributed as standalone Windows desktop application binaries:
  - Native Rust Executable: `target/release/localsql_studio.exe` built via `cargo build --release`.
  - Legacy PyInstaller executable: `dist/LocalSQLStudio.exe`.
  - Legacy Electron installer: `LocalSQL-Studio-Next/release/LocalSQL Studio Setup 0.0.0.exe`.

**CI Pipeline:**
- None configured in repo. All builds and test suites are executed locally via Cargo.

## Environment Configuration

**Required env vars:**
- None required for runtime execution.

**Secrets location:**
- No secrets, API keys, or credentials are used or stored.

## Webhooks & IPC Callbacks

**Inter-Thread Communication (Rust):**
- Lock-free message channels via `crossbeam_channel::unbounded()` connecting the immediate-mode GUI (`ui`) to the background DuckDB execution thread (`db`).

**Legacy Webhooks & IPC:**
- **Python HTTP API (`server.py:21-94`):**
  - `POST /api/query` — Receives JSON `{ "query": "..." }`, executes against in-memory DuckDB, returns JSON array of records.
  - `POST /api/load` — Receives JSON `{ "filePath": "..." }`, loads Excel sheets into DuckDB tables.
- **Electron IPC Handlers (`LocalSQL-Studio-Next/main.cjs:49-76`):**
  - `dialog:openFile` — Invokes native file dialog.
  - `db:loadExcel` — Reads workbook via SheetJS and streams CSV strings to renderer.

**Outgoing:**
- None. Loopback or in-process only.

---

*Integration audit: 2026-10-25*
