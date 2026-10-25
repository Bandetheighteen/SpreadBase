---
last_mapped_commit: bd6c25b37e18be419e3d1a63a4fd11d4ec501580
last_mapped_at: 2026-10-25
---
<!-- refreshed: 2026-10-25 -->

# Codebase Concerns

**Analysis Date:** 2026-10-25

## Tech Debt

**Active Architecture Transition (Rust Migration):**
- **Issue:** The repository is undergoing a complete architectural pivot to a 100% Pure Rust desktop application to resolve the UI lag, high memory consumption, and multi-language fragmentation of previous prototypes.
- **Current State:** The root Cargo package (`Cargo.toml`, `Cargo.lock`) and core module structure (`src/lib.rs`, `src/main.rs`, `src/db/mod.rs`, `src/parsers/mod.rs`, `src/ui/mod.rs`, `src/export/mod.rs`) are defined, but the module implementations are stubs awaiting Phase 1 through Phase 5 execution.
- **Impact:** The application is not yet functional in Rust; meanwhile, legacy Python and Electron implementations coexist in the same repository.
- **Fix approach:** Execute the planned GSD roadmap phases systematically, finishing the DuckDB engine wrapper (Phase 1), ingestion parsers (Phase 2), desktop GUI (Phase 3), editor/grid (Phase 4), and export (Phase 5).

**Legacy Multi-Codebase Footprint:**
- **Issue:** The root directory contains three concurrent code trees:
  1. Rust Architecture: `Cargo.toml`, `src/` (Active production target).
  2. Python Prototype: `app.py`, `ui/`, `database/`, `sources/`, `export/`, `utils/`, `server.py` (Deprecated).
  3. Electron/Web Prototype: `LocalSQL-Studio-Next/` (Deprecated).
- **Files:** `app.py`, `server.py`, `LocalSQLStudio.spec`, `LocalSQL-Studio-Next/`, `requirements.txt`.
- **Impact:** Cluttered root directory, potential confusion regarding entry points, and redundant dependencies.
- **Fix approach:** Phase 6 of the project roadmap (`06-verification-cleanup`) is explicitly dedicated to removing legacy Python and Electron code once the Rust implementation achieves feature parity.

## Known Bugs & Fragile Areas

**SQL Identifier & Keyword Collisions:**
- **Symptoms:** Loading an Excel worksheet with spaces, hyphens, leading numbers, or SQL reserved keywords (e.g., `Order`, `2024-Sales`, `Group By`) can trigger syntax errors during query execution.
- **Root Cause:** Unsanitized string interpolation when generating `CREATE TABLE` statements or views.
- **Fix approach:** In the Rust `db` module, always wrap table identifiers in standard DuckDB double-quotes: `format!("CREATE TABLE \"{}\" AS ...", sanitized_name)` and sanitize internal quotes.

**Windows Path Normalization in SQL:**
- **Symptoms:** Windows backslash file paths (e.g., `C:\Users\Data\file.csv`) can be misinterpreted as escape sequences in SQL statements.
- **Fix approach:** Normalize all filesystem paths to forward slashes (`/`) or use parameterized bindings before passing paths to DuckDB SQL functions (`read_csv_auto`).

## Security Considerations

**Local Filesystem SQL Capabilities:**
- **Risk:** DuckDB SQL provides commands capable of reading and writing local files (`ATTACH`, `COPY TO`, `read_csv`).
- **Current Mitigation:** The production Rust desktop application operates strictly local-first and offline, with no open network sockets, web servers, or external API integrations.
- **Legacy Alert:** The legacy `server.py` prototype bound an HTTP server to `127.0.0.1:8080` with `Access-Control-Allow-Origin: *`, presenting a theoretical cross-origin query risk from malicious browser tabs. Decommissioning `server.py` eliminates this surface.

## Performance Bottlenecks & Mitigations

**Large Worksheet Ingestion Latency:**
- **Problem:** Parsing multi-megabyte Excel files (`.xlsx`) row-by-row in interpreted Python or JSON-serialized Electron IPC created noticeable freezes.
- **Rust Mitigation:** Utilizing `calamine` in Rust provides direct binary decompression and reading of OpenXML sheets into native vectors, streamed directly into DuckDB without intermediate serialization.

**Immediate-Mode GUI Frame Budgets:**
- **Problem:** `egui` re-renders widgets every frame. Attempting to render tens of thousands of table cells each frame will immediately degrade framerates below 60 FPS.
- **Mitigation:** The results grid must implement strict row virtualization (using `egui_extras::TableBuilder` or manual scroll offset calculations) so that only the ~30–50 rows visible in the viewport are drawn.

## Dependencies at Risk

**Legacy JavaScript Dependencies:**
- **Risk:** The legacy `LocalSQL-Studio-Next/package.json` contains `xlsx` (0.18.5) which has known security vulnerabilities and is deprecated on npm, as well as a pre-release version of `@duckdb/duckdb-wasm`.
- **Mitigation:** Rust migration eliminates all npm packages and node runtime dependencies entirely.

## Test Coverage Gaps

**Scaffolded Rust Modules:**
- **Issue:** As the Rust codebase is currently at the scaffolding stage, automated test coverage is 0%.
- **Mitigation:** Comprehensive unit tests in `src/**/mod.rs` and black-box integration tests in `tests/` are required as acceptance criteria across all upcoming execution plans.

---

*Concerns analysis: 2026-10-25*
