# Pitfalls Research

**Domain:** Local Desktop SQL Workbench (100% Rust)
**Researched:** 2026-10-14
**Confidence:** HIGH

## Critical Pitfalls

### Pitfall 1: C++ Compiler Toolchain for `duckdb-sys` on Windows

**What goes wrong:**
Compiling `duckdb` with the `bundled` feature compiles DuckDB's C++ source code during `cargo build`. If the MSVC C++ toolchain (`cl.exe`, Visual Studio C++ Build Tools) is missing or misconfigured, build fails with `error: failed to run custom build command for duckdb-sys`.

**Why it happens:**
DuckDB is written in high-performance C++11. The Rust crate provides bindings that compile the engine into a static library.

**How to avoid:**
Ensure Visual Studio C++ Build Tools (MSVC) or MinGW is present in the build environment. If native C++ compilation is blocked, fallback to dynamically loading `duckdb.dll` or building with standard MSVC environment.

**Phase to address:**
Phase 1 (Scaffolding & Engine Foundation).

---

### Pitfall 2: Heterogeneous Cell Types in Excel Sheets

**What goes wrong:**
Unlike relational databases, Excel sheets allow completely mixed data types in the same column (e.g., cell A2 is integer `42`, cell A3 is string `"N/A"`, cell A4 is an empty cell). Attempting to enforce strict SQL types like `INTEGER` causes runtime panic or ingestion failure when inserting row A3.

**Why it happens:**
Spreadsheets do not enforce strict schema constraints.

**How to avoid:**
Implement intelligent type reconciliation in `parsers/excel.rs`:
- Scan the first 100 rows to determine the dominant data type.
- If a column contains mixed strings and numbers, coerce the column type to `VARCHAR`.
- Convert empty cells, blank strings, or Excel errors (`#DIV/0!`, `#N/A`) into SQL `NULL`.

**Phase to address:**
Phase 2 (Dataset Ingestion & Excel-as-Database Architecture).

---

### Pitfall 3: Unescaped SQL Identifiers in Sheet and File Names

**What goes wrong:**
Excel sheet names frequently contain spaces, dashes, parentheses, or SQL reserved keywords (e.g. `Sheet 1`, `Order-Details`, `2024_Sales`, `Group`). Executing `CREATE TABLE Sheet 1 AS ...` causes instant SQL syntax errors.

**Why it happens:**
Direct string interpolation of sheet names into SQL statements without escaping.

**How to avoid:**
1. Sanitize the identifier: convert non-alphanumeric characters to underscores (e.g., `sheet_1`, `order_details`).
2. Prefix identifiers starting with numbers with an underscore.
3. Always wrap identifiers in standard SQL double quotes: `CREATE TABLE "{sanitized_name}" (...)`.

**Phase to address:**
Phase 2 (Dataset Ingestion).

---

### Pitfall 4: Immediate-Mode GUI High CPU Utilization on Idle

**What goes wrong:**
Immediate-mode GUIs like `egui` re-evaluate the entire UI tree every frame. If configured to continuously repaint, the application will consume 100% of a CPU core rendering identical static frames when idle.

**Why it happens:**
Calling `ctx.request_repaint()` unconditionally in the update loop.

**How to avoid:**
Rely on `egui`'s default reactive mode: `egui` only repaints when user input events occur (mouse hover, click, typing) or when a background query worker sends a message on the channel (triggering a single targeted `ctx.request_repaint()`).

**Phase to address:**
Phase 3 (GUI Implementation & Immediate-Mode Layout).

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Parsing Excel synchronously on UI thread | Saves writing channel communication logic | Causes window to freeze and trigger "Not Responding" on large files | Never. Always parse in background threads. |
| Ingesting all Excel cells as `VARCHAR` | Eliminates type deduction complexity | Prevents numeric SQL queries (`SUM`, `AVG`, `>`) without explicit casting | Never. Perform basic type inference (Int, Float, Bool, Text). |
| Rendering all rows in a simple loop | Simpler code than `TableBuilder` | Sluggish rendering and UI stutter above 1,000 rows | Never. Always use virtualized `TableBuilder`. |

## "Looks Done But Isn't" Checklist

- [ ] **Excel Ingestion:** Often missing multi-sheet support — verify that ALL sheets in a workbook are registered as independent tables.
- [ ] **SQL Error Handling:** Often crashes on bad SQL syntax — verify that typing invalid SQL displays an error message gracefully without closing the window.
- [ ] **Large Datasets:** Often crashes or stutters on 10,000+ rows — verify virtual scrolling maintains 60 FPS.
- [ ] **Column Resizing:** Often crops long strings — verify that columns can be resized or scrolled horizontally.

---
*Pitfalls research for: Local Desktop SQL Workbench (100% Rust)*
*Researched: 2026-10-14*
