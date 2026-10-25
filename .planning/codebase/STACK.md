---
last_mapped_commit: bd6c25b37e18be419e3d1a63a4fd11d4ec501580
last_mapped_at: 2026-10-25
---
<!-- refreshed: 2026-10-25 -->

# Technology Stack

**Analysis Date:** 2026-10-25

## Languages

**Primary:**
- Rust (2021 Edition) — Core production architecture for high-performance single-binary desktop executable (`Cargo.toml`, `src/lib.rs`, `src/main.rs`).
- Python (>=3.10) — Legacy desktop prototype backend, PySide6 GUI layer, file parser, and DuckDB orchestration (`app.py`, `server.py`, `database/duckdb_manager.py`, `ui/main_window.py`).
- TypeScript / JavaScript (ES2022+ / CJS) — Legacy experimental frontend and Electron shell (`LocalSQL-Studio-Next/src/App.tsx`, `LocalSQL-Studio-Next/main.cjs`, `LocalSQL-Studio-Next/preload.cjs`).

**Secondary:**
- SQL (DuckDB SQL Dialect) — In-process analytical queries executed against CSV, Parquet, and Excel sheets.
- CSS / QSS — Qt Style Sheets (`utils/theme.py`) for the native Python app and Tailwind CSS (`LocalSQL-Studio-Next/tailwind.config.cjs`, `LocalSQL-Studio-Next/src/index.css`) for the React frontend.

## Runtime

**Environment:**
- Rust Toolchain (`rustc` / `cargo` 1.99.0+) compiling native Windows 64-bit binaries.
- Python 3.10+ virtual environment (`venv/`) for running legacy Python prototype.
- Node.js runtime (v18+ / v20+) and Electron runtime (`electron` v32.0.0) for `LocalSQL-Studio-Next`.
- WebAssembly (`@duckdb/duckdb-wasm` in browser workers) for legacy web prototype.

**Package Managers & Build Systems:**
- Rust: Cargo (`Cargo.toml`, `Cargo.lock`) with release profile optimizations (`opt-level = 3`, `lto = true`, `strip = true`).
- Python: `pip` with `requirements.txt`.
- Node.js: `npm` with `package-lock.json` present in `LocalSQL-Studio-Next/`.

## Frameworks & Libraries

**Core Rust Architecture:**
- `eframe` (0.29.1) & `egui` (0.29.1) & `egui_extras` (0.29.1) — Immediate-mode GPU-accelerated desktop GUI framework (`wgpu` backend, default fonts).
- `duckdb` (1.1.1) — Embedded in-process OLAP database engine configured with `bundled`, `chrono`, and `serde_json` features.
- `calamine` (0.26.1) — Pure Rust Excel workbook parser for `.xlsx` and `.xls` sheet extraction.
- `csv` (1.3.0) — High-throughput streaming CSV parser and deserializer.
- `rfd` (0.15.0) — Native OS file dialog integration for file opening and save paths.
- `crossbeam-channel` (0.5.13) — High-performance multi-producer multi-consumer channels for lock-free worker thread coordination.
- `rust_xlsxwriter` (0.70.0) — Native Excel file generation for result export.
- `serde` (1.0) & `serde_json` (1.0) & `chrono` (0.4) — Serialization, JSON handling, and timestamp manipulation.

**Legacy Frameworks:**
- PySide6 (>=6.6.0) — Qt 6 framework for legacy Python desktop application (`ui/main_window.py`).
- React 19 (`react` 19.2.8, `react-dom` 19.2.8) & Vite (`vite` 8.3.0) & Electron (`electron` 32.0.0) — Legacy web/desktop prototype (`LocalSQL-Studio-Next/`).
- Monaco Editor (`@monaco-editor/react` ^4.7.0) — SQL code editor in `LocalSQL-Studio-Next/src/App.tsx`.
- Tailwind CSS (`tailwindcss` 3.4.19) & Lucide React (`lucide-react` 1.48.0) — UI styling in React app.

**Build & Packaging:**
- Cargo — Standalone single-binary Windows compilation (`target/release/localsql_studio.exe`).
- PyInstaller (`pyinstaller` >= 6.4.0, `LocalSQLStudio.spec`) — Compiles legacy Python app into `dist/LocalSQLStudio.exe`.
- Electron Builder (`electron-builder` ^26.15.3) — Packages legacy Electron installer.

## Key Dependencies

**Critical:**
- `duckdb` (crate 1.1.1 bundled) — Primary analytical engine executing zero-lag vectorized SQL over local datasets.
- `calamine` (0.26.1) & `csv` (1.3.0) — Core ingestion engine reading local spreadsheets and tabular files into DuckDB.
- `eframe` (0.29.1) — Desktop window shell and rendering pipeline for immediate-mode GUI.

**Development & Testing:**
- `tempfile` (3.10) — Dev-dependency for creating isolated temporary files during Rust integration tests.
- `cargo test` — Native test harness.

## Configuration

**Environment:**
- Local-first and offline by default. No remote credentials or external API connections required.

**Build:**
- `Cargo.toml` — Rust package manifest specifying dependencies, library/binary targets, and release profile.
- `LocalSQLStudio.spec` — Legacy PyInstaller configuration targeting `app.py`.
- `LocalSQL-Studio-Next/vite.config.ts` & `tsconfig.json` — Legacy TypeScript bundler configuration.

## Platform Requirements

**Development:**
- Windows 10/11 x64 (PowerShell environment)
- Rust toolchain 1.99.0+ (`rustc`, `cargo`)
- Python 3.10+ (for legacy tooling)
- Node.js 18+ and npm (for legacy tooling)

**Production:**
- Standalone Windows 64-bit native executable (`localsql_studio.exe`), requiring no runtime dependencies or database daemon.

---

*Stack analysis: 2026-10-25*
