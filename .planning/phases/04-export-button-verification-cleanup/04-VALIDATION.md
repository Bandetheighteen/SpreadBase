# Phase 4 Validation Strategy: Export Button, Verification & Cleanup

## Validation Criteria

Phase 4 delivers the export capabilities (saving query results to Excel `.xlsx`, CSV, and JSON), end-to-end automated pipeline testing, production standalone release compilation, and legacy prototype cleanup.

| Requirement | Target Component | Validation Method |
|-------------|------------------|-------------------|
| **EXPORT-01** | `src/export/mod.rs`, `src/ui/app.rs` | Saves `QueryResult` to a local CSV file via native OS save dialog. |
| **EXPORT-02** | `src/export/mod.rs`, `src/ui/app.rs` | Saves `QueryResult` to an Excel workbook (`.xlsx`) via pure-Rust `rust_xlsxwriter`. |
| **EXPORT-03** | `src/export/mod.rs`, `src/ui/app.rs` | Saves `QueryResult` to formatted JSON file. |
| **VERIFY-03** | `tests/test_e2e_pipeline.rs` | Full automated integration test: ingest Excel/CSV -> execute SQL JOIN -> export results -> re-ingest and verify data fidelity. |
| **VERIFY-04** | `cargo build --release` | Produces a single standalone Windows executable `target/release/localsql_studio.exe` with zero runtime dependencies. |

## Automated Test Coverage

1. **`tests/test_export.rs`**:
   - Tests `export_to_csv`, `export_to_excel`, and `export_to_json` with null values, numbers, dates, booleans, and unicode strings.
   - Verifies exported files exist, are valid, and can be read back cleanly.
2. **`tests/test_e2e_pipeline.rs`**:
   - Complete circular verification test proving zero data loss across the entire application lifecycle.
3. **`cargo build --release`**:
   - Validates release profile build with LTO, size optimization, and strip flags.
