# External Integrations

**Analysis Date:** 2026-10-14

## APIs & External Services

**Local-First Design:**
- In accordance with the core architecture defined in `Concept.txt`, the application operates completely offline by default.
- No third-party SaaS APIs, telemetry trackers, cloud services, or remote analytics endpoints are integrated.
- Source data strictly remains on the local filesystem and in local RAM.

## Data Storage

**Databases:**
- **DuckDB (In-Memory Python):**
  - Engine: DuckDB in-process OLAP engine (`database/duckdb_manager.py:27`).
  - Connection: `duckdb.connect(':memory:')` initialized on application start.
  - Driver/Client: Python native `duckdb` module.
  - Table Registration: Creates views via `read_csv_auto('{file_path}')` for CSVs; loads Excel sheets into DataFrames and executes `CREATE TABLE {table_name} AS SELECT * FROM df`.
- **DuckDB-WASM (In-Memory Browser/Electron):**
  - Engine: `@duckdb/duckdb-wasm` running inside a WebAssembly Web Worker (`LocalSQL-Studio-Next/src/App.tsx:25-34`).
  - Connection: `AsyncDuckDBConnection` via `duckdb.connect()`.
  - Data Ingestion: Ingests CSV files via `insertCSVFromPath` and `registerFileText`.

**File Storage:**
- **Local Filesystem Only:**
  - Ingestion: Reads `.csv`, `.xlsx`, and `.xls` files selected by the user via file dialogs (`ui/main_window.py:144-170`, `LocalSQL-Studio-Next/main.cjs:49-56`).
  - Metadata: Stores workspace configuration, registered file sources, table schemas, and query history in `workspace.json` via `workspace/workspace_manager.py`.
  - Export Target: Writes query output files to local disk in `.csv`, `.xlsx`, `.json`, or `.parquet` formats via `export/exporter.py`.

**Caching:**
- None. Query results and registered tables reside exclusively in DuckDB in-memory buffers during process lifetime.

## Authentication & Identity

**Auth Provider:**
- None. This is a single-user local desktop application. There are no user accounts, passwords, JWT tokens, or authentication providers.

## Monitoring & Observability

**Error Tracking:**
- **Python / PySide6 App:** Exception catching in `database/duckdb_manager.py` emits errors via Qt signals (`QueryWorkerSignals.error`), displayed in the "Messages" tab of `ui/results_grid.py:110-115` and status bar messages (`ui/main_window.py:138-142`).
- **React / Electron App:** React state (`error`) rendered via `AlertCircle` banner (`LocalSQL-Studio-Next/src/App.tsx:66-67, 137-142`).
- **Electron Main Process:** Web contents console messages forwarded to stdout (`LocalSQL-Studio-Next/main.cjs:32-34`).
- **External Error Tracking Service:** None (no Sentry, Datadog, or Bugsnag).

**Logs:**
- Standard console logging (`sys.stdout`, `console.log`).
- `LocalSQL-Studio-Next/out.log` used for local development logging.
- `wiki/log.md` contains historical research notes.

## CI/CD & Deployment

**Hosting & Distribution:**
- Distributed as standalone Windows desktop application binaries:
  - Native Python executable: `dist/LocalSQLStudio.exe` built via PyInstaller (`LocalSQLStudio.spec`).
  - Electron installer: `LocalSQL-Studio-Next/release/LocalSQL Studio Setup 0.0.0.exe` built via Electron Builder (`LocalSQL-Studio-Next/package.json`).
- Python background server (`server.py`) optionally serves static files from `dist/` on local port `8080` with a Tkinter tray window.

**CI Pipeline:**
- None detected. Builds and packaging are executed locally.

## Environment Configuration

**Required env vars:**
- None required for runtime.
- `NODE_ENV`: Set to `development` during Electron dev workflow (`LocalSQL-Studio-Next/package.json:9`).

**Secrets location:**
- No secrets, credentials, or API tokens are used or stored.

## Webhooks & Callbacks

**Incoming:**
- **Python HTTP API (`server.py:21-94`):**
  - `POST /api/query` - Receives JSON `{ "query": "..." }`, runs query via DuckDB, returns JSON array of rows.
  - `POST /api/load` - Receives JSON `{ "filePath": "..." }`, reads Excel sheets via pandas, creates DuckDB tables, returns loaded table names.
- **Electron IPC Handlers (`LocalSQL-Studio-Next/main.cjs:49-76`):**
  - `dialog:openFile` - Invokes OS file picker dialog for Excel and CSV files.
  - `db:loadExcel` - Reads workbook using SheetJS (`xlsx.readFile`), converts sheets to CSV strings, and returns sheet metadata to the renderer.

**Outgoing:**
- None. All network requests are strictly loopback (`127.0.0.1` / `localhost`).

---

*Integration audit: 2026-10-14*
