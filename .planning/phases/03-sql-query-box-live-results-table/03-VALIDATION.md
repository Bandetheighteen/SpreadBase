# Phase 3 Validation Strategy: SQL Query Box & Live Results Table

## Validation Criteria

Phase 3 introduces the multi-line SQL editor, shortcut key handlers (F5, Ctrl+Enter), error banner diagnostics, and virtualized high-performance results table grid.

| Requirement | Target Component | Validation Method |
|-------------|------------------|-------------------|
| **EDITOR-01** | `src/ui/editor.rs` | Monospace code editor with text selection and multi-line navigation. |
| **EDITOR-02** | `src/ui/app.rs` | F5 and Ctrl+Enter keyboard shortcuts dispatch query execution. |
| **EDITOR-03** | `src/ui/editor.rs`, `src/ui/app.rs` | If text is selected in the editor, only the selected fragment executes; else full buffer executes. |
| **EDITOR-04** | `src/ui/sidebar.rs`, `src/ui/app.rs` | Clicking table generates `SELECT * FROM "{table}" LIMIT 100;`. |
| **ENGINE-04** | `src/db/manager.rs`, `src/ui/app.rs` | Query runtime duration tracked and displayed in milliseconds. |
| **ENGINE-05** | `src/ui/app.rs` | Syntax and runtime errors rendered as clear red diagnostic messages without crashing. |
| **GRID-01** | `src/ui/grid.rs` | `egui_extras::TableBuilder` virtualized scrolling renders 50,000+ rows smoothly at 60 FPS. |
| **GRID-02** | `src/ui/grid.rs` | Headers display column names with proper alignment and horizontal scrolling for wide datasets. |
| **GRID-03** | `src/ui/grid.rs` | NULL values visually distinguished with muted placeholder styling. |
| **GRID-04** | `src/ui/grid.rs`, `src/ui/app.rs` | Status bar and table footer display total rows returned and execution timing. |

## Automated Test Coverage

1. **`tests/test_ui_grid.rs`**:
   - Executes queries returning large datasets (10,000+ rows) through DuckDB and verifies row count, column definitions, and microsecond timing.
   - Tests NULL value handling and heterogeneous column display formatting.
   - Verifies error reporting for malformed SQL queries.
2. **`cargo check`**:
   - Ensures `egui_extras::TableBuilder` and all UI editor and grid modules compile cleanly.
3. **`cargo test`**:
   - Regressions check across Phase 1, Phase 2, and Phase 3 suites.
