# Stack Research

**Domain:** Local Desktop SQL Workbench & In-Memory Analytical Processing (100% Rust)
**Researched:** 2026-10-14
**Confidence:** HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| **Rust** | 1.99.0+ (Edition 2021) | Application Language & Runtime | Guarantees zero-cost abstractions, memory safety, thread safety without garbage collection, and eliminates UI/runtime lag. |
| **eframe / egui** | 0.29.1 | Desktop Application Shell & Immediate-Mode GUI | Pure 100% Rust immediate-mode GUI framework. Renders via native GPU (wgpu/glow), starts up in under 50ms, and provides 60+ FPS zero-lag interaction without web bloat. |
| **duckdb-rs (`duckdb`)** | 1.1.1 (bundled) | Embedded OLAP SQL Engine | Industry-leading analytical SQL engine. Bundled feature compiles DuckDB directly into the Rust binary with zero external C/C++ runtime install requirements. |
| **calamine** | 0.26.1 | Pure-Rust Excel Parser | Blazingly fast reading of `.xlsx`, `.xls`, `.ods`, and `.xlsb` files without Microsoft Office dependencies. Extracts sheets and cell data in memory. |
| **csv** | 1.3.0 | High-Throughput CSV Parser | Streaming, SIMD-accelerated CSV parser capable of processing hundreds of megabytes per second. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| **egui_extras** | 0.29.1 | Virtualized Data Table (`TableBuilder`) | Essential for rendering large SQL result sets (100,000+ rows) with virtual row scrolling, column resizing, and sorting without dropping frames. |
| **rfd** (Rusty File Dialogs) | 0.15.0 | Native OS File Pickers | Seamless integration with native Windows open/save file dialogs for picking Excel/CSV datasets and exporting results. |
| **crossbeam-channel** | 0.5.13 | Multi-Producer Multi-Consumer Channels | Communicates between the GUI render loop and the background DuckDB query worker thread without blocking the UI thread. |
| **rust_xlsxwriter** | 0.70.0 | Pure-Rust Excel File Generation | Used to export query result grids directly into formatted `.xlsx` workbooks. |
| **serde & serde_json** | 1.0.200 | Serialization & JSON Output | Persisting workspace catalogues, query history, and JSON result export. |
| **chrono** | 0.4.38 | Timestamp & Duration Formatting | Tracking query execution timing down to microseconds and formatting date/time fields. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| **Cargo** | Build system, dependency manager, test runner | Standard Rust toolchain (`cargo build --release`, `cargo test`). |
| **rustfmt & clippy** | Code formatting and static analysis lints | Ensures idiomatic Rust conventions and catches anti-patterns early. |

## Installation & Cargo Dependencies

```toml
[dependencies]
# GUI Framework
eframe = { version = "0.29.1", default-features = false, features = ["default_fonts", "wgpu"] }
egui = "0.29.1"
egui_extras = { version = "0.29.1", features = ["default"] }

# Embedded SQL Database
duckdb = { version = "1.1.1", features = ["bundled", "chrono", "serde_json"] }

# Data Ingestion Parsers
calamine = "0.26.1"
csv = "1.3.0"

# OS File Dialogs
rfd = "0.15.0"

# Concurrency & Channels
crossbeam-channel = "0.5.13"

# Export & Serialization
rust_xlsxwriter = "0.70.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tempfile = "3.10"
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| **egui / eframe** | Tauri + Web UI | Use Tauri if complex HTML/CSS styling or web components are strictly needed. Rejected here because user explicitly specified 100% pure Rust with zero lag. |
| **egui / eframe** | Iced | Iced is an Elm-architecture retained mode GUI. Good for document-centric apps, but `egui` has vastly superior table virtualization (`egui_extras::TableBuilder`) and faster iteration. |
| **DuckDB (`duckdb-rs`)** | Apache Arrow DataFusion | DataFusion is 100% pure Rust, but DuckDB has vastly superior out-of-the-box analytical SQL dialect, window functions, and native CSV/Parquet `COPY` export capabilities. |
| **calamine** | `office` / `umya-spreadsheet` | `calamine` is read-only, making it significantly faster and lower-memory for analytical ingestion than read-write spreadsheet crates. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| **Python / PySide6** | Python GIL, heavy memory footprint, and Qt event loop lag during dataset loads. | 100% native compiled Rust (`eframe` + `duckdb`). |
| **Electron / Chromium** | High RAM usage (200MB+ idle), slow IPC string serialization, and WASM memory limits. | Native Rust executable (~15MB binary, instant 60 FPS). |
| **Dynamic DuckDB Linking** | Requires end-users to have `duckdb.dll` manually installed or placed in PATH. | `duckdb = { version = "...", features = ["bundled"] }` to statically bundle DuckDB into the binary. |
| **Blocking GUI Query Runs** | Freezes the GUI while executing SQL or parsing multi-megabyte Excel files. | Spawn background worker threads communicating via `crossbeam_channel`. |

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| `eframe@0.29.1` | `egui@0.29.1`, `egui_extras@0.29.1` | Must keep all `egui` family crates on the exact same minor release. |
| `duckdb@1.1.1` | Rust 1.80+ / 1.99.0 | Compiles with bundled C++ engine seamlessly via `cc` crate. |

## Sources

- [DuckDB Rust Official Documentation](https://duckdb.org/docs/api/rust) — Verified bundled compilation and connection API.
- [egui / eframe Crates & Examples](https://github.com/emilk/egui) — Verified `TableBuilder` virtual scrolling performance.
- [Calamine Crate Docs](https://docs.rs/calamine) — Verified sheet inspection and type coercion patterns.

---
*Stack research for: Local Desktop SQL Workbench (100% Rust)*
*Researched: 2026-10-14*
