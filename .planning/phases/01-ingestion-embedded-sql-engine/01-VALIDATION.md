---
phase: "1"
slug: "ingestion-embedded-sql-engine"
status: ready
nyquist_compliant: true
wave_0_complete: false
created: "2026-10-26"
---

# Phase 1 — Validation Strategy (Ingestion & Embedded SQL Engine)

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Cargo Test (built-in Rust test runner) |
| **Config file** | `Cargo.toml` |
| **Quick run command** | `cargo check` |
| **Full suite command** | `cargo test` |
| **Estimated runtime** | ~5-10 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo check`
- **After Plan 01:** Run `cargo test --test test_duckdb_engine`
- **After Plan 02:** Run `cargo test --test test_ingestion_and_query`
- **Before Phase Completion:** Full suite `cargo test` must be 100% green
- **Max feedback latency:** 10 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | ENGINE-01 | — | SQL types and result structure | unit | `cargo check` | ❌ W0 | ⬜ pending |
| 01-01-02 | 01 | 1 | ENGINE-01 | T-01-01 | In-memory DbManager initialization & query | unit | `cargo check` | ❌ W0 | ⬜ pending |
| 01-01-03 | 01 | 1 | ENGINE-02 | T-01-02 | Analytical SQL execution (CTEs, windows, joins) | integration | `cargo test --test test_duckdb_engine` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 2 | INGEST-01, INGEST-02, INGEST-03, INGEST-04 | T-02-01 | Excel sheet discovery, headers as variables, sanitization | unit | `cargo check` | ❌ W0 | ⬜ pending |
| 01-02-02 | 02 | 2 | INGEST-05 | T-02-02 | CSV delimiter and type inference | unit | `cargo check` | ❌ W0 | ⬜ pending |
| 01-02-03 | 02 | 2 | INGEST-01..05, VERIFY-01, VERIFY-02 | T-02-01 | Multi-sheet Excel and CSV JOIN queries | integration | `cargo test --test test_ingestion_and_query` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tests/test_duckdb_engine.rs` — Integration test suite verifying DuckDB engine
- [ ] `tests/test_ingestion_and_query.rs` — Integration test suite verifying multi-sheet Excel & CSV JOINs

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| *None* | — | — | All Phase 1 behaviors have automated verification via `cargo test`. |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all test references
- [x] No watch-mode flags
- [x] Feedback latency < 10s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** ready
