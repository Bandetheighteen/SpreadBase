---
last_mapped_commit: bd6c25b37e18be419e3d1a63a4fd11d4ec501580
last_mapped_at: 2026-10-25
---
<!-- refreshed: 2026-10-25 -->

# Coding Conventions

**Analysis Date:** 2026-10-25

## Naming Patterns

### Rust (Active Core Architecture)

**Files & Modules:**
- `snake_case.rs` for all source files: `lib.rs`, `main.rs`, `mod.rs`.
- `snake_case` for module paths: `crate::db`, `crate::parsers`, `crate::ui`, `crate::export`.

**Types & Traits:**
- `PascalCase` for Structs, Enums, and Traits: `DuckEngine`, `QueryResult`, `QueryCommand`, `FileParser`, `DataCell`.
- Enum variants must be `PascalCase`: `QueryStatus::Running`, `QueryStatus::Success`, `QueryStatus::Error(String)`.

**Functions & Methods:**
- `snake_case` for all functions, methods, and macros: `execute_query()`, `register_sheet()`, `render_grid()`.
- Constructor functions conventionally named `new()` or `with_capacity()`.
- Conversion methods follow standard Rust idioms: `as_str()`, `to_string()`, `into_inner()`.

**Variables & Constants:**
- `snake_case` for variables, parameters, and struct fields: `file_path`, `row_count`, `table_name`.
- `SCREAMING_SNAKE_CASE` for compile-time constants and statics: `DEFAULT_FONT_SIZE`, `MAX_PREVIEW_ROWS`.

### Legacy Python & TypeScript

**Python:**
- Files: `snake_case.py` (`main_window.py`, `duckdb_manager.py`).
- Classes: `PascalCase` (`MainWindow`, `VirtualTableModel`).
- Methods: `snake_case` (`run_query()`).

**TypeScript / React:**
- Components: `PascalCase.tsx` (`App.tsx`).
- Variables & functions: `camelCase` (`runQuery()`, `loadFile()`).

## Code Style

### Rust

**Formatting & Linting:**
- Standard `rustfmt` formatting: 4-space indentation, 100-character line width target.
- Standard `cargo clippy` compliance: zero compiler warnings or clippy errors in production builds.

**Idioms & Best Practices:**
- Immutability by default: use `let` unless mutation is explicitly required (`let mut`).
- Ownership & Borrowing: prefer borrowed references (`&str`, `&Path`, `&[T]`) in function signatures over owned clones (`String`, `PathBuf`, `Vec<T>`), passing ownership only when data is retained.
- Channel communication: prefer `crossbeam_channel::unbounded()` or `bounded(n)` for asynchronous message passing without mutex contention.
- Pattern Matching: utilize exhaustive `match` statements and `if let` blocks over nested conditionals.

### Python & TypeScript

- Python: 4-space indentation, PEP 8 guidelines.
- TypeScript: 2-space indentation, Oxlint configured via `LocalSQL-Studio-Next/.oxlintrc.json`.

## Import Organization

### Rust Import Order:

Group imports into three distinct blocks separated by an empty line:

1. **Standard Library (`std`):**
   ```rust
   use std::path::{Path, PathBuf};
   use std::sync::Arc;
   use std::time::Instant;
   ```
2. **External Crates:**
   ```rust
   use crossbeam_channel::{Receiver, Sender};
   use duckdb::{Connection, Result as DuckResult};
   use eframe::egui;
   ```
3. **Crate-Local Modules:**
   ```rust
   use crate::db::DuckEngine;
   use crate::parsers::ExcelParser;
   ```

### Legacy Python Import Order:

1. Standard library (`os`, `sys`, `json`).
2. Third-party packages (`duckdb`, `pandas`, `PySide6`).
3. Local application imports (`ui.*`, `database.*`).

## Error Handling

### Rust

**Strategy:**
- Idiomatic `Result<T, E>` pattern throughout all modules.
- **No Uncontrolled Panics:** Avoid `unwrap()` and `expect()` in runtime and GUI rendering code. Reserve `.expect()` strictly for test harnesses or initialisation assertions that represent fatal programmer errors.
- Propagate recoverable errors with the `?` operator up to the caller or channel boundary.
- Worker threads capture errors and wrap them in strongly-typed messages (e.g. `QueryResponse::Error(String)`) so the UI can display formatted error details without terminating the application.

```rust
pub fn execute_query(&self, sql: &str) -> Result<QueryResult, DuckError> {
    let mut stmt = self.conn.prepare(sql)?;
    let rows = stmt.query([])?;
    // Process and collect rows
    Ok(result)
}
```

### Legacy Python

- `try...except Exception as e:` blocks wrapping query execution and emitting Qt error signals.

## Logging & Feedback

**Patterns:**
- Console diagnostics: `eprintln!` or standard logging for low-level worker status.
- User feedback: Elapsed execution time (`Instant::now().elapsed()`), affected row counts, and status indicators rendered in the `egui` status bar and output grid.

## Comments & Documentation

**Guidelines:**
- `//!` inner doc comments at the top of every module (`lib.rs`, `mod.rs`) documenting the module's role and architecture.
- `///` outer doc comments above all public structs, enums, traits, and functions detailing parameters and return semantics.
- Document any platform-specific quirks or unsafe blocks (if any are introduced).

## Function Design

**Guidelines:**
- Functions should adhere to the single-responsibility principle and remain compact (typically under 40 lines).
- Prefer functional iterator chains (`map`, `filter`, `fold`, `collect`) over manual index loops where readability is preserved.
- Immediate-mode UI functions decompose into modular panel rendering methods (`ui.render_sidebar()`, `ui.render_editor()`, `ui.render_results()`).

---

*Convention analysis: 2026-10-25*
