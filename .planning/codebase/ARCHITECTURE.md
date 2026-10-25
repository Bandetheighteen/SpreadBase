---
last_mapped_commit: bd6c25b37e18be419e3d1a63a4fd11d4ec501580
last_mapped_at: 2026-10-25
---
<!-- refreshed: 2026-10-25 -->

# Architecture

**Analysis Date:** 2026-10-25

## System Overview

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                      Primary Rust Desktop Target                        │
│                   `src/main.rs` -> `src/lib.rs`                         │
├─────────────────────────────────────────────────────────────────────────┤
│    Presentation Layer (egui / eframe) - `src/ui/mod.rs`                 │
│  ┌───────────────────────┬───────────────────────────────────────────┐  │
│  │ Source Explorer Panel │ SQL Editor Tabs (Syntax Highlighting)     │  │
│  ├───────────────────────┴───────────────────────────────────────────┤  │
│  │ Virtual Data Grid (Virtual Scroll, Column Sort, Filter)           │  │
│  └───────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │
                                     ▼ (crossbeam-channel message passing)
┌─────────────────────────────────────────────────────────────────────────┐
│                    Analytical & Ingestion Core (Rust)                   │
├────────────────────────────────────┬────────────────────────────────────┤
│   Embedded DuckDB (`src/db/mod.rs`)│   Parsers (`src/parsers/mod.rs`)   │
│   - In-process DuckDB v1.1.1       │   - `calamine` (.xlsx, .xls)       │
│   - Background query worker thread │   - `csv` streaming parser         │
├────────────────────────────────────┴────────────────────────────────────┤
│   Export Layer (`src/export/mod.rs`) - `rust_xlsxwriter`, CSV, JSON     │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │
                                     ▼ (Native File I/O via `rfd`)
┌─────────────────────────────────────────────────────────────────────────┐
│                       Local Datasets (.xlsx, .csv)                      │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                   Legacy Prototype Architectures (Deprecated)           │
├────────────────────────────────────┬────────────────────────────────────┤
│    PySide6 Desktop Application     │    React 19 / Electron Frontend    │
│    `app.py` -> `ui/main_window.py` │    `LocalSQL-Studio-Next/src/`     │
│    `database/duckdb_manager.py`    │    `LocalSQL-Studio-Next/main.cjs` │
└────────────────────────────────────┴────────────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility | Location |
|-----------|----------------|----------|
| Rust Application Binary | Entry point initializing GUI framework and worker channels | `src/main.rs:1-8` |
| Rust Core Library | Defines public modules: `db`, `parsers`, `ui`, `export` | `src/lib.rs:1-12` |
| Rust DB Engine | Embedded DuckDB connector, table registration, async query worker | `src/db/mod.rs` |
| Rust Ingestion Parsers | Fast Excel (`calamine`) and CSV (`csv`) ingestion | `src/parsers/mod.rs` |
| Rust Desktop UI | Immediate-mode desktop shell, sidebar tree, editor, virtual grid | `src/ui/mod.rs` |
| Rust Exporter | Serializes result sets to Excel (`rust_xlsxwriter`), CSV, JSON | `src/export/mod.rs` |
| Legacy Python App | Native PySide6 Qt desktop prototype | `app.py:1-14`, `ui/main_window.py` |
| Legacy HTTP Server | Local HTTP server exposing DuckDB endpoints + Tkinter status window | `server.py:21-108` |
| Legacy Electron App | Experimental React 19 + DuckDB-WASM workbench | `LocalSQL-Studio-Next/` |
| Legacy Workspace Manager | Session state persistence to `workspace.json` | `workspace/workspace_manager.py` |

## Pattern Overview

**Current State:** Transitioning from fragmented multi-runtime prototypes (Python/PySide6 and React/Electron) to a unified **100% Pure Rust Architecture**.

**Key Characteristics:**
- **Zero-Lag Immediate-Mode GUI:** Uses `egui` and `eframe` for GPU-accelerated rendering at 60+ FPS with minimal RAM footprint (<50MB vs >300MB in Electron).
- **Asynchronous Worker Threading:** Complete decoupling of the UI render loop from DuckDB execution via `crossbeam-channel` channels (`Sender`/`Receiver`), guaranteeing that long queries never freeze the interface.
- **Embedded In-Process Execution:** Vectorized columnar analytical processing with zero network serialization overhead.
- **Workbooks as Databases:** Logical mapping where each ingested spreadsheet file acts as a database and each sheet acts as a relational table.

## Layers

**1. Presentation Layer (`src/ui/`):**
- Purpose: Render desktop shell, source tree explorer, multi-tab SQL editor, query execution controls, and virtualized results table.
- Implements: `eframe::App` trait.
- Communication: Non-blocking channel polling via `crossbeam_channel::Receiver::try_recv()` inside `update()`.

**2. Database Engine Layer (`src/db/`):**
- Purpose: Connection lifecycle management, schema registration, SQL execution, and result set mapping.
- Engine: Embedded `duckdb` (bundled).
- Threading: Dedicated query execution thread executing queries dispatched over channels.

**3. Ingestion & Parsing Layer (`src/parsers/`):**
- Purpose: High-speed ingestion of tabular files without intermediate Python/Node processes.
- Parsers: `calamine` for `.xlsx`/`.xls` binary sheets, `csv` for delimiter-delimited files.

**4. Export Layer (`src/export/`):**
- Purpose: Converting query result buffers into user-specified destination formats (`.xlsx` via `rust_xlsxwriter`, `.csv`, `.json`).

## Data Flow

### Primary Rust Query Execution Flow

1. **User Input:** User enters SQL in the editor and presses `F5` or `Ctrl+Enter`.
2. **Channel Dispatch:** The `ui` module packages the SQL query into an execution command and sends it across a `crossbeam_channel::Sender`.
3. **Worker Processing:** The background `db` thread receives the query, runs `conn.prepare()` and `query()`, streaming tuples into structured record batches.
4. **Result Delivery:** The worker transmits results (columns, types, row vectors, execution timing) across a response channel.
5. **Frame Rendering:** The UI loop checks the response channel on each frame tick, populates the virtual grid model, and renders the dataset at 60 FPS.

### File Ingestion Flow

1. **File Selection:** User picks files via `rfd::FileDialog`.
2. **Workbook Parsing:** `calamine` inspects the workbook structure and extracts sheet names.
3. **Table Registration:** Sheets are registered into DuckDB as relational tables using batch inserts or view bindings.
4. **Catalog Refresh:** Source explorer tree updates to display available tables and column schemas.

## Entry Points

**`src/main.rs`:**
- Primary Rust desktop application binary entry point.
- Triggers: `cargo run --bin localsql_studio` or executing compiled `localsql_studio.exe`.

**`src/lib.rs`:**
- Primary Rust library exposing modules (`db`, `parsers`, `ui`, `export`) for binary consumption and automated integration testing (`tests/`).

**`app.py` & `server.py`:**
- Legacy Python entry points for historical reference.

**`LocalSQL-Studio-Next/main.cjs`:**
- Legacy Electron entry point for historical reference.

## Architectural Constraints & Invariants

- **Zero External Runtimes:** The production application must compile to a single self-contained binary with zero external runtime requirements (no Python, Node, or Electron).
- **Read-Only Ingestion:** Source Excel and CSV files are strictly read-only; no query modifies the original files on disk.
- **Local Isolation:** Process must operate entirely disconnected from any remote network.
- **Thread Boundary:** Database handles and heavy file parsers must never execute on the GUI rendering thread.

## Anti-Patterns & Fragilities

### Legacy Codebase Split

- **Problem:** Three differing codebases exist at repository root: Rust (`src/`), Python (`app.py`, `ui/`, `database/`), and Electron (`LocalSQL-Studio-Next/`).
- **Resolution:** The project roadmap commits to building out the Rust modules (Phase 1–5), after which legacy Python and Electron files will be cleaned up (Phase 6).

### Unquoted Identifiers in Dynamic SQL

- **Problem:** Sheets with whitespace, dashes, or reserved SQL words can break SQL query execution.
- **Resolution:** All table names and schema identifiers must be quoted using standard DuckDB double-quotes `"{table_name}"`.

---

*Architecture analysis: 2026-10-25*
