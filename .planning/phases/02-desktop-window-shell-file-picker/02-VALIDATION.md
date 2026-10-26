# Phase 2 Validation Strategy: Desktop Window Shell & File Picker

## Validation Criteria

Phase 2 introduces the native desktop graphical interface, async worker channel, file selection dialog, and database/table sidebar explorer.

| Requirement | Target Component | Validation Method |
|-------------|------------------|-------------------|
| **UI-01** | `src/main.rs`, `src/ui/app.rs` | Compiles with `eframe` and `egui`, initializes window configuration. |
| **UI-02** | `src/ui/app.rs` | Verifies toolbar, left sidebar panel, and central workspace layout. |
| **UI-03** | `src/ui/theme.rs` | Custom dark styling applied with high contrast and crisp typography. |
| **UI-04** | `src/ui/sidebar.rs` | File tree renders loaded files, tables (sheets), and columns with types. |
| **UI-05** | `src/ui/app.rs` | Reactive frame rendering maintains low CPU when idle. |
| **ENGINE-03** | `src/ui/worker.rs` | Async message channel ensures file reading and queries run off the main thread. |

## Automated Test Coverage

1. **`tests/test_ui_worker.rs`**:
   - Tests background worker channel message dispatching (`WorkerCommand::IngestFiles`, `WorkerCommand::ExecuteQuery`).
   - Verifies that file ingestion completes asynchronously and returns table metadata (sheet names, column counts, column names, and types).
   - Verifies error propagation when invalid or non-existent files are passed.
2. **`cargo check`**:
   - Ensures all UI modules, layout panels, and `eframe::App` implementations compile cleanly.
3. **`cargo test`**:
   - Ensures no regressions across Phase 1 DuckDB and ingestion test suites.
