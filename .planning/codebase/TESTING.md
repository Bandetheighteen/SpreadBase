# Testing Patterns

**Analysis Date:** 2026-10-14

## Test Framework

**Current Status:**
- **No Automated Test Framework Detected:** Neither the Python application nor the TypeScript/React (`LocalSQL-Studio-Next`) application has automated unit or integration tests configured.
- `requirements.txt` does not include `pytest`, `pytest-qt`, or `coverage`.
- `LocalSQL-Studio-Next/package.json` does not include `vitest`, `jest`, or `@testing-library/react`.

**Recommended Runners:**
- **Python Backend & UI:** `pytest` (>= 8.0) and `pytest-qt` (>= 4.4.0) for Qt signal and widget testing.
- **Frontend & WebAssembly:** `vitest` (^1.0+) with `@testing-library/react`.

**Run Commands (Planned / Recommended):**
```bash
# Python test execution
pytest tests/ -v                       # Run all Python unit tests
pytest tests/ --cov=database --cov=ui  # Run with coverage report

# React / Electron test execution
cd LocalSQL-Studio-Next
npm run test                           # Run Vitest test suite
```

## Test File Organization

**Location (Recommended Structure):**
```text
Single Server SQL Program/
├── tests/
│   ├── conftest.py                    # Shared pytest fixtures (in-memory DB, temp CSVs)
│   ├── test_duckdb_manager.py         # Tests for query execution and view creation
│   ├── test_file_parser.py            # Tests for Excel/CSV schema detection
│   ├── test_exporter.py               # Tests for CSV, Excel, Parquet file generation
│   ├── test_workspace_manager.py      # Tests for workspace.json serialization
│   └── ui/
│       ├── test_results_grid.py       # VirtualTableModel item model verification
│       └── test_sql_highlighter.py    # Syntax highlighting regex checks
└── LocalSQL-Studio-Next/
    └── src/
        └── __tests__/
            └── App.test.tsx           # React component and WASM init tests
```

## Test Structure & Patterns (Recommended)

### Unit Testing `DuckDBManager`
```python
import pytest
import os
import tempfile
from database.duckdb_manager import DuckDBManager

@pytest.fixture
def db_manager():
    return DuckDBManager()

@pytest.fixture
def sample_csv():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.csv', delete=False) as f:
        f.write("id,name,value\n1,Alice,100.5\n2,Bob,200.0\n")
        path = f.name
    yield path
    if os.path.exists(path):
        os.remove(path)

def test_register_and_query_csv(db_manager, sample_csv):
    db_manager.register_csv(sample_csv, "test_table")
    assert "test_table" in db_manager.registered_tables
    
    result = db_manager._execute_sync("SELECT * FROM test_table WHERE id = 1")
    assert result["success"] is True
    assert result["columns"] == ["id", "name", "value"]
    assert len(result["data"]) == 1
    assert result["data"][0][1] == "Alice"
```

### Testing `FileParser` Type Mapping
```python
from sources.file_parser import FileParser
import numpy as np
import pandas as pd

def test_map_dtype_to_sql():
    assert FileParser._map_dtype_to_sql(np.dtype("int64")) == "INTEGER"
    assert FileParser._map_dtype_to_sql(np.dtype("float64")) == "DOUBLE"
    assert FileParser._map_dtype_to_sql(np.dtype("bool")) == "BOOLEAN"
    assert FileParser._map_dtype_to_sql(np.dtype("datetime64[ns]")) == "DATE"
    assert FileParser._map_dtype_to_sql(np.dtype("object")) == "VARCHAR"
```

### UI Testing with `pytest-qt`
```python
from ui.results_grid import VirtualTableModel
from PySide6.QtCore import Qt

def test_virtual_table_model(qtbot):
    headers = ["ID", "Name"]
    data = [(1, "Alpha"), (2, "Beta")]
    model = VirtualTableModel(data, headers)
    
    assert model.rowCount() == 2
    assert model.columnCount() == 2
    assert model.data(model.index(0, 1), Qt.DisplayRole) == "Alpha"
```

## Mocking

**Guidelines:**
- Mock filesystem dialogs (`QFileDialog.getOpenFileNames`, `QFileDialog.getSaveFileName`) during GUI integration testing.
- For DuckDB-WASM in TypeScript, mock `@duckdb/duckdb-wasm` web workers when testing pure React UI states.
- Avoid mocking DuckDB in Python backend tests: DuckDB connects directly to `:memory:` in microseconds, making real queries faster and more reliable than mock objects.

## Coverage

**Requirements:**
- Current: 0% (no automated tests present).
- Target: Recommend reaching >=80% branch coverage on core data logic (`database/duckdb_manager.py`, `sources/file_parser.py`, `export/exporter.py`).

## Common Testing Gaps

1. **Query Execution Thread Safety:** No test verifies concurrent queries or background worker behavior under slow queries.
2. **Path Injection & Escaping:** No validation tests for file paths containing apostrophes, backslashes, or spaces.
3. **Large File Handling:** No stress tests verifying behavior on 100MB+ Excel or CSV datasets.
4. **Export Integrity:** No round-trip tests verifying that data exported via `export/exporter.py` can be read back identically.

---

*Testing analysis: 2026-10-14*
