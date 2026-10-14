# Technology Stack

**Analysis Date:** 2026-10-14

## Languages

**Primary:**
- Python (>=3.10) - Native desktop application backend, PySide6 GUI layer, file parser, and DuckDB orchestration (`app.py`, `server.py`, `database/duckdb_manager.py`, `ui/main_window.py`).
- TypeScript / JavaScript (ES2022+ / CJS) - Next-generation React/Electron application (`LocalSQL-Studio-Next/src/App.tsx`, `LocalSQL-Studio-Next/main.cjs`, `LocalSQL-Studio-Next/preload.cjs`).

**Secondary:**
- SQL (DuckDB SQL Dialect) - In-process analytical queries executed against CSV, Parquet, and Excel sheets.
- CSS / QSS - Qt Style Sheets (`utils/theme.py`) for the native Python app and Tailwind CSS (`LocalSQL-Studio-Next/tailwind.config.cjs`, `LocalSQL-Studio-Next/src/index.css`) for the React frontend.

## Runtime

**Environment:**
- Python 3.10+ virtual environment (`venv/`)
- Node.js runtime (v18+ / v20+) for `LocalSQL-Studio-Next`
- Electron runtime (`electron` v32.0.0)
- WebAssembly (DuckDB-WASM in dedicated browser workers: `duckdb-browser-mvp.worker.js`, `duckdb-browser-eh.worker.js`)

**Package Manager:**
- Python: `pip` with `requirements.txt`
- Node.js: `npm` with `package-lock.json` present in `LocalSQL-Studio-Next/`

## Frameworks

**Core:**
- PySide6 (>=6.6.0) - Qt 6 framework for the native Python desktop application (`ui/main_window.py`).
- React 19 (`react` 19.2.8, `react-dom` 19.2.8) - UI component library for `LocalSQL-Studio-Next`.
- Vite (`vite` 8.3.0) - Build tool and development server for React frontend.
- Electron (`electron` 32.0.0) - Cross-platform desktop shell wrapping the React/Vite interface.

**Database & Analytical Engine:**
- DuckDB (`duckdb` >= 0.10.0 Python package) - In-memory and file-backed OLAP SQL engine in Python.
- DuckDB-WASM (`@duckdb/duckdb-wasm` ^1.33.1-dev57.0) - WebAssembly client-side SQL engine in browser/Electron.
- Apache Arrow (`apache-arrow` ^21.2.0) - In-memory columnar data interchange format for DuckDB-WASM query results.

**Code Editor:**
- Monaco Editor (`@monaco-editor/react` ^4.7.0) - Full-featured SQL code editor in `LocalSQL-Studio-Next/src/App.tsx`.
- Custom Qt Editor (`ui/line_number_area.py`, `ui/syntax_highlighter.py`) - Custom `QPlainTextEdit` with line gutter and syntax highlighting for native Qt app.

**Styling & UI Components:**
- Tailwind CSS (`tailwindcss` 3.4.19, `postcss` 8.5.28, `autoprefixer` 10.6.1) - Utility-first styling in `LocalSQL-Studio-Next`.
- Lucide React (`lucide-react` 1.48.0) - Modern iconography for the React interface.
- Custom QSS Theme (`utils/theme.py`, `utils/icon_manager.py`) - VS Code-inspired dark theme and SVG icon generator.

**Build & Packaging:**
- PyInstaller (`pyinstaller` >= 6.4.0, `LocalSQLStudio.spec`) - Compiles Python/PySide6 app into standalone executable `dist/LocalSQLStudio.exe`.
- Electron Builder (`electron-builder` ^26.15.3) - Packages Electron app into NSIS installer and portable executable (`LocalSQL-Studio-Next/release/`).

**Linting & Quality:**
- Oxlint (`oxlint` ^1.81.0, `LocalSQL-Studio-Next/.oxlintrc.json`) - High-performance JavaScript/TypeScript linter.

## Key Dependencies

**Critical:**
- `duckdb` (>=0.10.0) & `@duckdb/duckdb-wasm` (^1.33.1-dev57.0) - Core analytical query engine enabling sub-second SQL queries over CSV and Excel.
- `pandas` (>=2.2.0) - DataFrame abstraction used in Python for schema inference, type conversion, and intermediate DuckDB table registration.
- `openpyxl` (>=3.1.2) - Read-only Excel workbook parser extracting sheet names and schemas in Python (`sources/file_parser.py`).
- `xlsx` (SheetJS ^0.18.5) - In-memory workbook reading and conversion to CSV streams in Electron main process (`LocalSQL-Studio-Next/main.cjs`).

**Infrastructure:**
- `pyinstaller` (>=6.4.0) - Executable distribution tooling.
- `concurrently` (^10.0.5) & `wait-on` (^9.1.0) - Development script coordination for Vite and Electron (`LocalSQL-Studio-Next/package.json`).

## Configuration

**Environment:**
- Local-first and offline by default. No external API keys or remote service credentials required.
- Dynamic asset resolution via `sys._MEIPASS` when running from PyInstaller bundle (`server.py`).

**Build:**
- `LocalSQLStudio.spec` - PyInstaller build specification targeting `app.py` with icon `sources/LOGO.png`.
- `LocalSQL-Studio-Next/vite.config.ts` - Vite configuration with `@vitejs/plugin-react`.
- `LocalSQL-Studio-Next/tsconfig.json`, `tsconfig.app.json`, `tsconfig.node.json` - TypeScript compiler configurations.
- `LocalSQL-Studio-Next/tailwind.config.cjs` - Tailwind content path configuration.

## Platform Requirements

**Development:**
- Windows 10/11 x64 (PowerShell environment)
- Python 3.10+ with pip
- Node.js 18+ and npm

**Production:**
- Standalone Windows 64-bit desktop binaries (`dist/LocalSQLStudio.exe` or `LocalSQL-Studio-Next/release/LocalSQL Studio Setup 0.0.0.exe`).
- Fully self-contained; no network connection or database daemon required.

---

*Stack analysis: 2026-10-14*
