<!-- refreshed: 2026-10-14 -->
# Architecture

**Analysis Date:** 2026-10-14

## System Overview

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                           Presentation Layer                            │
├────────────────────────────────────┬────────────────────────────────────┤
│    PySide6 Desktop Application     │    React 19 / Electron Frontend    │
│    `app.py` -> `ui/main_window.py` │    `LocalSQL-Studio-Next/src/`     │
│  ┌───────────────┬────────────────┐│  ┌───────────────┬────────────────┐│
│  │ ExplorerPanel │ SQLEditorPanel ││  │ Sidebar Table │ Monaco Editor  ││
│  │ `ui/explorer` │ `ui/sql_editor`││  │ `App.tsx`     │ `App.tsx`      ││
│  └───────────────┴────────────────┘│  └───────────────┴────────────────┘│
│  ┌────────────────────────────────┐│  ┌────────────────────────────────┐│
│  │ ResultsGridPanel (`VirtualTab`)││  │ Results Table Grid             ││
│  │ `ui/results_grid.py`           ││  │ `App.tsx`                      ││
│  └────────────────────────────────┘│  └────────────────────────────────┘│
└──────────────────┬─────────────────┴───────────────────┬────────────────┘
                   │                                     │
                   ▼ (Qt Signals / QThreadPool)          ▼ (IPC / Worker PostMessage)
┌────────────────────────────────────┐ ┌──────────────────────────────────┐
│         Python Engine Core         │ │     Electron Main & WASM Core    │
│   `database/duckdb_manager.py`     │ │ `LocalSQL-Studio-Next/main.cjs`  │
│   - In-memory DuckDB connection    │ │   - IPC handlers: openFile,      │
│   - `QueryWorker` async threads    │ │     loadExcel (via `xlsx`)       │
│   - View/Table registration        │ │ `duckdb-browser-*.worker.js`     │
│   `sources/file_parser.py`         │ │   - DuckDB-WASM engine           │
│   `export/exporter.py`             │ │   - Apache Arrow result buffers  │
│   `workspace/workspace_manager.py` │ │                                  │
└──────────────────┬─────────────────┘ └─────────────────┬────────────────┘
                   │                                     │
                   ▼                                     ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                    Local Filesystem / Datasets                          │
│               Local CSV, XLSX, and XLS Data Files                       │
└─────────────────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| Native App Entry | Initializes `QApplication` and launches main window | `app.py:1-15` |
| HTTP/Tray Server | Optional local HTTP server exposing DuckDB endpoints + Tkinter status window | `server.py:21-109` |
| Main Window UI | Coordinates toolbar, explorer splitter, editor tabs, and status bar | `ui/main_window.py:15-197` |
| SQL Editor Panel | Manages editor tabs, syntax highlighter, and query execution trigger | `ui/sql_editor.py:11-72` |
| Code Editor & Gutter | Custom `QPlainTextEdit` with line number area and VS Code dark theme styling | `ui/line_number_area.py:20-116` |
| Syntax Highlighter | Highlights SQL keywords, strings, numeric literals, and block comments | `ui/syntax_highlighter.py:4-77` |
| Explorer Panel | Displays tree hierarchy for sources, tables, views, and saved queries | `ui/explorer.py:7-46` |
| Results Grid Panel | High-performance virtual table model with sorting, column filtering, and messages | `ui/results_grid.py:45-118` |
| DuckDB Manager | Thread-safe DuckDB connection wrapper, asynchronous query dispatcher | `database/duckdb_manager.py:24-86` |
| File Parser | Inspects Excel sheets and infers SQL data types from sample chunks | `sources/file_parser.py:5-62` |
| Exporter | Converts query results to CSV, Excel, JSON, and Parquet | `export/exporter.py:3-27` |
| Workspace Manager | Persists workspace sources, table mappings, and query history to `workspace.json` | `workspace/workspace_manager.py:4-53` |
| Theme & Icons | Color palette, Qt Style Sheets (QSS), and dynamic SVG icon rendering | `utils/theme.py:1-220`, `utils/icon_manager.py:6-37` |
| Electron Main | Creates BrowserWindow, registers custom `app://` protocol, handles OS dialogs | `LocalSQL-Studio-Next/main.cjs:12-77` |
| Electron Preload | Securely exposes `electronAPI` (`openFile`, `loadExcel`) to renderer | `LocalSQL-Studio-Next/preload.cjs:1-9` |
| Next-Gen React UI | Single-page workbench using DuckDB-WASM, Monaco Editor, and Tailwind | `LocalSQL-Studio-Next/src/App.tsx:36-269` |

## Pattern Overview

**Overall:** Dual-architecture workspace transitioning from a Native Python (PySide6 + DuckDB) application to a Modern Web/Desktop hybrid (Electron + React 19 + DuckDB-WASM).

**Key Characteristics:**
- **Local-First & In-Process:** Zero network roundtrips for query processing; queries execute against local RAM using DuckDB engines.
- **Model-View-ViewModel (PySide6):** Qt's `QAbstractTableModel` (`VirtualTableModel`) combined with `QSortFilterProxyModel` decouples data storage from table view rendering.
- **Worker Thread Offloading:** Query execution is offloaded to background threads in Python (`QThreadPool`) and Web Workers in TypeScript (`@duckdb/duckdb-wasm`), preventing UI thread freeze.

## Layers

**Presentation Layer:**
- Purpose: Render IDE workspace, panels, tabs, code editors, and data tables.
- Location: `ui/` (Python), `LocalSQL-Studio-Next/src/` (React).
- Contains: Qt widgets, custom painters, syntax highlighters, React components, Monaco editor integration.
- Depends on: Application and Database layers.

**Application & Storage Layer:**
- Purpose: Workspace state persistence, file parsing, data export, and DuckDB orchestration.
- Location: `database/`, `sources/`, `export/`, `workspace/`, `LocalSQL-Studio-Next/main.cjs`.
- Contains: DuckDB connection management, `pd.read_excel` wrappers, `openpyxl` sheet introspection, SheetJS conversions.
- Depends on: Local disk filesystem, DuckDB native bindings / WASM bundles.

## Data Flow

### Primary Request Path (PySide6 Desktop App)

1. **User Action:** User inputs SQL in `SQLEditorPanel` and clicks "Run (F5)" (`ui/sql_editor.py:22`, `ui/main_window.py:40`).
2. **Dispatch:** `MainWindow.run_query()` captures the SQL text and calls `DuckDBManager.execute_query_async()` (`ui/main_window.py:113-123`).
3. **Background Execution:** `QueryWorker` runs inside `QThreadPool.globalInstance()`, calls `_execute_sync()`, fetches cursor descriptions and data tuples (`database/duckdb_manager.py:10-23, 54-72`).
4. **Result Emission:** `QueryWorkerSignals.finished` emits dictionary `{ "success": True, "columns": [...], "data": [...] }` back to the Qt main thread.
5. **Grid Update:** `MainWindow._on_query_success()` updates `VirtualTableModel` via `set_data()`, resetting model signals and repainting `QTableView` (`ui/main_window.py:124-135`, `ui/results_grid.py:14-19`).

### Secondary Flow: File Registration

1. **Selection:** User selects files via `QFileDialog.getOpenFileNames` (`ui/main_window.py:144-150`).
2. **Registration:**
   - For CSVs: `DuckDBManager.register_csv()` executes `CREATE OR REPLACE VIEW {table_name} AS SELECT * FROM read_csv_auto('{path}')` (`database/duckdb_manager.py:32-37`).
   - For Excel: `sources/file_parser.py` retrieves sheet names with `openpyxl`. `DuckDBManager.register_excel()` loads the sheet into pandas and creates a DuckDB table (`database/duckdb_manager.py:39-45`).
3. **Notification:** Success message logged to the "Messages" tab and status bar (`ui/main_window.py:158-167`).

### Next-Gen Data Flow (`LocalSQL-Studio-Next`)

1. User clicks "Open File" -> Renderer triggers `window.electronAPI.openFile()` (`LocalSQL-Studio-Next/src/App.tsx:76`).
2. Electron main process opens native file dialog (`LocalSQL-Studio-Next/main.cjs:49-56`).
3. Renderer calls `window.electronAPI.loadExcel(filePath)`, which reads sheets and converts each sheet into a CSV string (`LocalSQL-Studio-Next/main.cjs:58-76`).
4. Renderer registers CSV strings into DuckDB-WASM virtual filesystem using `registerFileText()` and `conn.insertCSVFromPath()` (`LocalSQL-Studio-Next/src/App.tsx:81-96`).
5. User enters SQL in Monaco Editor and clicks "Run Query".
6. DuckDB-WASM executes query in web worker, returns Apache Arrow table converted to JSON records, updating React state (`LocalSQL-Studio-Next/src/App.tsx:49-73`).

## Key Abstractions

**DuckDBManager:**
- Purpose: Encapsulates connection lifecycle, table registry, and query worker thread pools.
- Location: `database/duckdb_manager.py:24-86`
- Pattern: Facade / Manager pattern.

**VirtualTableModel:**
- Purpose: Bridges raw Python tuple results into Qt's item model system with lazy display formatting.
- Location: `ui/results_grid.py:8-44`
- Pattern: Model-View abstraction (`QAbstractTableModel`).

**WorkspaceManager:**
- Purpose: Manages persistent catalogue of user files, logical tables, and past query execution history.
- Location: `workspace/workspace_manager.py:4-53`
- Pattern: Active Record / Repository pattern over JSON (`workspace.json`).

## Entry Points

**`app.py`:**
- Location: `c:/Users/shoun/OneDrive/Desktop/Dokumen/Single Server SQL Program/app.py`
- Triggers: User running `python app.py` or double-clicking `dist/LocalSQLStudio.exe`.
- Responsibilities: Initializes `QApplication`, instantiates `MainWindow`, enters Qt event loop.

**`server.py`:**
- Location: `c:/Users/shoun/OneDrive/Desktop/Dokumen/Single Server SQL Program/server.py`
- Triggers: User running `python server.py`.
- Responsibilities: Launches background HTTP server on port 8080 with `/api/query` and `/api/load`, opens system web browser, presents Tkinter shutdown tray window.

**`LocalSQL-Studio-Next/main.cjs`:**
- Location: `c:/Users/shoun/OneDrive/Desktop/Dokumen/Single Server SQL Program/LocalSQL-Studio-Next/main.cjs`
- Triggers: User running `npm run dev` or executing compiled Electron binary.
- Responsibilities: Electron app lifecycle, window management, custom protocol registration, and native IPC handling.

## Architectural Constraints

- **Threading in Python:** The PySide6 UI runs strictly on the main thread. All database calls that could block on large datasets must run through `QThreadPool` / `QueryWorker` to avoid GUI freezes.
- **In-Memory Volatility:** DuckDB connects to `:memory:`. When the process exits, materialized tables and views are released. File registrations are preserved in `workspace.json` but must be re-registered on relaunch.
- **Path Sanitization:** Single quotes in local file paths must be escaped before interpolation into SQL statements (`database/duckdb_manager.py:34`).
- **WASM Memory Constraints:** DuckDB-WASM operates within browser WebAssembly 32-bit/64-bit memory limits (typically 2GB–4GB max heap).

## Anti-Patterns & Fragilities

### String Interpolation in Table and View Creation
**What happens:** Table names and file paths are directly formatted into SQL strings (`f"CREATE TABLE {table_name} AS SELECT * FROM df"` and `f"CREATE OR REPLACE VIEW {table_name} AS SELECT * FROM read_csv_auto('{file_path_clean}');"` in `database/duckdb_manager.py:35, 43`, `server.py:79`).
**Why it's wrong:** Sheet names or filenames with special symbols, spaces, or SQL keywords can cause syntax errors or unintended SQL execution.
**Do this instead:** Use parameterized table identifier sanitization or quote identifiers using DuckDB's double-quoting syntax `"{sanitized_identifier}"`.

### Disconnected Workspace State
**What happens:** `ui/main_window.py:150-167` registers files directly into `self.db`, but does not invoke `WorkspaceManager.add_source()` or `add_table()`. `WorkspaceManager` in `workspace/workspace_manager.py` is currently unused by `MainWindow`.
**Why it's wrong:** Registered tables and query history are not saved across application restarts.
**Do this instead:** Inject `WorkspaceManager` into `MainWindow` and synchronize file additions and query runs to `workspace.json`.

## Error Handling

**Strategy:** Graceful degradation with user-facing diagnostic messages and non-fatal recovery.

**Patterns:**
- Try/except blocks around DuckDB execution catch syntax and runtime errors without crashing the application (`database/duckdb_manager.py:18-22`, `server.py:57-61`).
- Error messages are piped into dedicated UI views: the "Messages" tab in `ui/results_grid.py:110-115` and inline banner in `LocalSQL-Studio-Next/src/App.tsx:66-67`.

## Cross-Cutting Concerns

**Logging:** Standard output and stderr; no structured logger module is currently implemented.
**Validation:** Basic file extension checking (`.csv`, `.xlsx`, `.xls`) in `ui/main_window.py:156, 159` and `LocalSQL-Studio-Next/main.cjs:52`.
**Authentication:** N/A (local single-user tool).

---

*Architecture analysis: 2026-10-14*
