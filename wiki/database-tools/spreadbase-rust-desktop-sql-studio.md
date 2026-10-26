# SpreadBase: Standalone Rust Desktop SQL Studio for Spreadsheets

> Sources: [Local SQL Queries on CSV and Excel Files](local-sql-on-csv-excel.md)
> Archived: 2026-10-26

## Overview

A synthesis of the architecture, engineering design, and implementation of SpreadBase, an open-source, 100% offline desktop application built in pure Rust for querying local multi-sheet Excel and CSV files via an embedded in-memory DuckDB SQL engine.

## Problem Context

Spreadsheets remain the standard format for tabular business, operational, and research data. However, analyzing cross-sheet relationships in traditional spreadsheet software requires fragile lookup formulas (`VLOOKUP`, `XLOOKUP`, `INDEX/MATCH`) that degrade in performance on large datasets. Cloud-based SaaS database tools require uploading sensitive files to remote third-party servers, posing data privacy risks. Setting up local SQL servers (such as PostgreSQL or MySQL) introduces administrative overhead, schema definitions, and background services.

## Architecture & Implementation

SpreadBase (released at v1.0.0 on GitHub under Bandetheighteen/SpreadBase) solves these challenges with a single-binary, local-first architecture:

### 1. Ingestion & Schema Reconciliation
* **Excel Ingestion**: Uses `calamine` to parse `.xlsx` and legacy `.xls` workbooks. Each sheet is registered as an independent relational table, namespaced as `"filename.xlsx".SheetName`.
* **CSV Ingestion**: Uses `csv` streaming reader with dynamic type inference for headers and records.
* **Type Reconciler**: Normalizes cell types across columns into `BIGINT`, `DOUBLE`, `VARCHAR`, `BOOLEAN`, and `TIMESTAMP`, handling missing and heterogeneous values safely.

### 2. In-Memory Analytical Engine
* **DuckDB Embedded**: An in-process, vectorized columnar engine running entirely in RAM. Supports analytical SQL (`JOIN`, `GROUP BY`, window functions, CTEs).
* **Zero Disk Footprint**: Database state resides strictly in-memory during the session; no temporary database files are left behind.

### 3. Desktop User Interface
* **Framework**: Built with `egui` and `eframe` using immediate-mode rendering over `wgpu`.
* **Non-Blocking Concurrency**: File parsing and query execution run on an isolated background OS thread via `crossbeam-channel`, ensuring the UI maintains 60 FPS without stuttering.
* **Virtualized Grid**: Utilizes `egui_extras::TableBuilder` to render only rows visible in the viewport, effortlessly scrolling through 50,000+ rows.
* **SQL Editor & Diagnostics**: Multi-line editor with query execution (`F5` / `Ctrl+Enter`), text-selection fragment execution, and syntax error alerts.

### 4. Multi-Format Exporter
* Exports query results to Excel (`.xlsx` via `rust_xlsxwriter`), RFC 4180 CSV (`.csv`), and JSON (`.json`) via native OS file save dialogs (`rfd`).

## See Also

- [Local SQL Queries on CSV and Excel Files](local-sql-on-csv-excel.md)
- [Go Language CLI Tools for Local SQL](go-language-cli-tools-for-local-sql.md)
