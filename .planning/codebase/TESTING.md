---
last_mapped_commit: bd6c25b37e18be419e3d1a63a4fd11d4ec501580
last_mapped_at: 2026-10-25
---
<!-- refreshed: 2026-10-25 -->

# Testing Patterns

**Analysis Date:** 2026-10-25

## Test Framework

**Current Status:**
- **Active Rust Architecture:** Configured with Rust's native test harness (`cargo test`). Integration test support is enabled via `[lib]` in `Cargo.toml`, and `tempfile = "3.10"` is configured under `[dev-dependencies]`.
- **Legacy Codebases:** No automated tests were ever implemented for the legacy Python or Electron applications (`requirements.txt` lacks `pytest`, and `LocalSQL-Studio-Next/package.json` lacks test runners).

**Test Runners & Tooling:**
- **Primary:** `cargo test` (built-in Rust test framework).
- **Fixtures:** `tempfile` (3.10) for disk-isolated CSV and Excel test file generation.
- **Coverage (Planned):** `cargo-tarpaulin` or `cargo-llvm-cov` for branch and line coverage reports.

**Run Commands:**

```bash

# Run all unit and integration tests

cargo test

# Run only library unit tests

cargo test --lib

# Run a specific test by name

cargo test test_duckdb_query_execution

# Run tests showing stdout output

cargo test -- --nocapture

# Run integration tests in tests/

cargo test --test '*'
```

## Test Organization

**Recommended & Active Structure:**

```text
Single Server SQL Program/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── db/
│   │   └── mod.rs             # Inlined unit tests: #[cfg(test)] mod tests
│   ├── parsers/
│   │   └── mod.rs             # Parser unit tests with mock CSV/sheet data
│   ├── ui/
│   │   └── mod.rs             # State management and event handler unit tests
│   └── export/
│       └── mod.rs             # Serialization and file export unit tests
└── tests/                     # Integration tests (black-box against public lib.rs API)
    ├── common/
    │   └── mod.rs             # Shared test helpers and fixture builders
    ├── test_duckdb_core.rs    # In-memory database initialization, query and types
    ├── test_csv_ingestion.rs  # Streaming CSV file registration and querying
    ├── test_excel_ingest.rs   # Calamine .xlsx workbook ingestion and table schemas
    └── test_export.rs         # Verification of exported .xlsx, .csv, and .json files
```

## Unit Test Conventions

**Inlined Unit Tests:**
Place unit tests at the bottom of the module they test inside a conditional compilation module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_connection() {
        let engine = DuckEngine::new().expect("Failed to initialize engine");
        let result = engine.execute_query("SELECT 42 AS answer").expect("Query failed");
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], DataCell::Integer(42));
    }
}
```

## Integration Tests & Fixtures

**Temporary Test Fixtures:**
Use `tempfile` to generate isolated spreadsheet and CSV samples during integration test execution to prevent filesystem pollution:

```rust
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_csv_file_registration() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "id,name,amount\n1,Alice,100.5\n2,Bob,200.0").unwrap();
    
    let engine = DuckEngine::new().unwrap();
    engine.register_csv_table("users", file.path()).unwrap();
    
    let result = engine.execute_query("SELECT SUM(amount) FROM users").unwrap();
    // Validate aggregate result
}
```

## Quality & Coverage Goals

- **Analytical Core:** 100% test coverage for SQL execution, type conversion (integers, floats, dates, strings, nulls), and error propagation.
- **Parsers:** Test edge cases such as empty worksheets, special characters in column headers, missing delimiters, and mismatched row counts.
- **Export:** Byte-level validation of output files ensuring generated Excel and CSV files can be reopened cleanly.

---

*Testing analysis: 2026-10-25*
