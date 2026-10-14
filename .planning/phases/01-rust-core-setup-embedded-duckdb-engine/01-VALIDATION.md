---
phase: "1"
slug: "rust-core-setup-embedded-duckdb-engine"
status: draft
nyquist_compliant: true
wave_0_complete: false
created: "2026-10-14"
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Cargo Test (built-in Rust test runner) |
| **Config file** | `Cargo.toml` |
| **Quick run command** | `cargo check` |
| **Full suite command** | `cargo test --test test_duckdb_engine` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo check`
- **After every plan wave:** Run `cargo test`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 10 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | ENGINE-01 | T-01-SC | Dependency legitimacy check | build | `cargo check` | ❌ W0 | ⬜ pending |
| 01-01-02 | 01 | 1 | ENGINE-01 | — | Clean project root compilation | build | `cargo check` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 2 | ENGINE-01 | — | Column type mapping and serialization | unit | `cargo test --test test_duckdb_engine test_types` | ❌ W0 | ⬜ pending |
| 01-02-02 | 02 | 2 | ENGINE-01, ENGINE-02 | T-01-01 | In-memory database manager queries | unit | `cargo test --test test_duckdb_engine test_basic_query` | ❌ W0 | ⬜ pending |
| 01-02-03 | 02 | 2 | ENGINE-02 | T-01-02 | Analytical SQL (CTEs, windows, joins) | integration | `cargo test --test test_duckdb_engine` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `Cargo.toml` — Rust dependencies and package configuration
- [ ] `tests/test_duckdb_engine.rs` — Integration test suite verifying DuckDB engine

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| *None* | — | — | All Phase 1 behaviors have automated verification via `cargo test`. |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 10s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
