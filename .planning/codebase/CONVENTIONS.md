# Coding Conventions

**Analysis Date:** 2026-10-14

## Naming Patterns

### Python

**Files:**
- Standard snake_case: `main_window.py`, `duckdb_manager.py`, `file_parser.py`, `results_grid.py`.

**Classes:**
- PascalCase: `DuckDBManager`, `MainWindow`, `SQLEditorPanel`, `VirtualTableModel`, `FileParser`, `Exporter`.
- Custom Qt widgets mirror standard Qt naming: `SqlCodeEditor` (subclasses `QPlainTextEdit`), `ResultsGridPanel` (subclasses `QWidget`).

**Functions & Methods:**
- snake_case for standard methods and helpers: `run_query()`, `register_csv()`, `get_excel_sheets()`, `save_workspace()`.
- camelCase or mixedCase when overriding Qt virtual methods or Qt handlers: `rowCount()`, `columnCount()`, `headerData()`, `paintEvent()`, `sizeHint()`.
- Leading underscore for internal/private methods: `_execute_sync()`, `_on_query_success()`, `_map_dtype_to_sql()`.

**Variables & Constants:**
- snake_case for local variables and attributes: `file_path`, `table_name`, `current_results`, `registered_tables`.
- SCREAMING_SNAKE_CASE for class-level configuration and theme constants: `BACKGROUND = "#1E1E1E"`, `PRIMARY_ACCENT = "#007ACC"`, `SVG_STRINGS = {...}`.

### TypeScript / React (`LocalSQL-Studio-Next`)

**Files:**
- PascalCase for React component files: `App.tsx`.
- camelCase or kebab-case for configuration and scripts: `main.cjs`, `preload.cjs`, `vite.config.ts`.

**Identifiers:**
- camelCase for functions, variables, and hooks: `runQuery()`, `loadFile()`, `initDB()`, `activeTab`.
- PascalCase for React components and types: `App`, `duckdb.DuckDBBundles`.

## Code Style

### Python

**Formatting:**
- 4-space indentation.
- No explicit automated linter/formatter config (`pyproject.toml`, `setup.cfg`, `flake8`, or `black`) in repository.
- Double quotes primarily used for SQL templates and UI strings, single quotes occasionally used in dictionaries.

**Design Idioms:**
- Static utility classes with `@staticmethod` and `@classmethod` (`FileParser` in `sources/file_parser.py`, `Exporter` in `export/exporter.py`, `Theme` in `utils/theme.py`, `IconManager` in `utils/icon_manager.py`).
- Qt Signal/Slot pattern for inter-thread communication (`QueryWorkerSignals` subclassing `QObject` with `Signal(dict)` and `Signal(str)` in `database/duckdb_manager.py:6-9`).
- Model-View separation: Custom `QAbstractTableModel` implementation (`VirtualTableModel` in `ui/results_grid.py:8-44`) coupled with `QSortFilterProxyModel` for sorting and client-side filtering.

### TypeScript / React

**Formatting & Linting:**
- 2-space indentation.
- Linter: Oxlint configured via `LocalSQL-Studio-Next/.oxlintrc.json`:
  ```json
  {
    "categories": {
      "correctness": "warn"
    }
  }
  ```
- Strict type checking enabled via `LocalSQL-Studio-Next/tsconfig.app.json` (`"strict": true`, `"noUnusedLocals": true`, `"noUnusedParameters": true`).

## Import Organization

### Python Order:
1. Standard library imports:
   ```python
   import os
   import sys
   import json
   import time
   import tempfile
   ```
2. Third-party library imports:
   ```python
   import duckdb
   import pandas as pd
   import openpyxl
   from PySide6.QtWidgets import QMainWindow, QWidget, ...
   from PySide6.QtCore import Qt, Signal, ...
   ```
3. Internal application module imports:
   ```python
   from utils.theme import Theme
   from ui.explorer import ExplorerPanel
   from database.duckdb_manager import DuckDBManager
   from export.exporter import Exporter
   ```

### TypeScript / React Order:
1. React core hooks: `import { useState, useEffect } from 'react';`
2. Third-party UI components & icons: `import Editor from '@monaco-editor/react';`, `import { Database, ... } from 'lucide-react';`
3. Engine libraries and worker asset imports: `import * as duckdb from '@duckdb/duckdb-wasm';`, `import duckdb_wasm from '...wasm?url';`
4. Stylesheets: `import './App.css';`

## Error Handling

**Patterns:**
- Try/Except in database execution: All synchronous and asynchronous query executions are wrapped in `try...except Exception as e:` blocks.
  ```python
  try:
      result = self.db_manager._execute_sync(self.query)
      self.signals.finished.emit(result)
  except Exception as e:
      self.signals.error.emit(str(e))
  ```
- User Notification: Caught exceptions are passed to UI panels and displayed as formatted messages with status indicators (`❌ Error: ...`) rather than crashing the interface (`ui/main_window.py:138-142`, `ui/results_grid.py:110-115`).
- IPC error responses: Electron IPC handlers return `{ success: false, error: err.message }` objects (`LocalSQL-Studio-Next/main.cjs:73-75`).

## Logging

**Framework:**
- Native console printing (`sys.stdout`, `console.log`) and Qt status bar messages.
- No third-party structured logging library (such as `loguru` or Python standard `logging`) is currently utilized.

**Patterns:**
- Ephemeral status updates emitted via `QStatusBar.showMessage("Executing query...")` and `showMessage("Ready | N rows | 0.012s")` in `ui/main_window.py:118, 134`.
- Diagnostic output logged to the "Messages" tab in `ResultsGridPanel`.

## Comments & Documentation

**When to Comment:**
- Docstrings describe purpose on key classes and public methods (`database/duckdb_manager.py:33, 39, 54, 74`, `sources/file_parser.py:7, 21, 50`).
- Inline comments explain non-obvious Qt quirks, such as font metrics calculation for line number gutter (`ui/line_number_area.py:54`) and preventing GC of highlighters (`ui/sql_editor.py:54`).

## Function Design

**Size:**
- Functions generally remain focused on a single responsibility (typically 10–35 lines).
- UI setup functions decompose large layouts into subroutines (`setup_toolbar()`, `setup_workspace()`, `setup_statusbar()` in `ui/main_window.py`).

**Return Values:**
- Database operations return structured dictionaries: `{"success": True, "columns": [...], "data": [...]}` or raise exceptions handled by caller.
- Export functions return a boolean `True` on success.

---

*Convention analysis: 2026-10-14*
