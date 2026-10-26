//! Live Results Table Grid Component
//!
//! Provides virtualized, high-performance tabular rendering using `egui_extras::TableBuilder`.
//! Capable of displaying 50,000+ rows at 60 FPS with horizontal scrolling, column resizing,
//! and distinctive styling for NULL values and numeric data.

use egui::{Align, Layout, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::db::types::{QueryResult, SqlValue};
use super::theme::{COLOR_SUCCESS, COLOR_TEXT_MUTED, COLOR_TEXT_PRIMARY};

/// Renders the virtualized results table grid.
pub fn render_results_table(ui: &mut Ui, result: &QueryResult) {
    if result.columns.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(RichText::new("Query executed successfully (no columns returned).").color(COLOR_TEXT_MUTED));
        });
        return;
    }

    if result.row_count == 0 {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(RichText::new("Query returned 0 rows.").color(COLOR_TEXT_MUTED));
        });
        return;
    }

    // Results Header Info Bar
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!(
                "RESULTS ({} rows in {:.2} ms)",
                result.row_count, result.execution_time_ms
            ))
            .strong()
            .size(11.0)
            .color(COLOR_TEXT_MUTED),
        );
    });

    ui.add_space(4.0);

    // Build the virtualized table
    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(Layout::left_to_right(Align::Center))
        .column(Column::exact(45.0)) // Row index column
        .columns(
            Column::initial(130.0).at_least(70.0).resizable(true),
            result.columns.len(),
        );

    table
        .header(26.0, |mut header| {
            // Row number header
            header.col(|ui| {
                ui.label(RichText::new("#").strong().size(11.0).color(COLOR_TEXT_MUTED));
            });

            // Column headers
            for col in &result.columns {
                header.col(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&col.name).strong().size(12.0).color(COLOR_TEXT_PRIMARY));
                        ui.label(RichText::new(&col.data_type).size(10.0).color(COLOR_TEXT_MUTED));
                    });
                });
            }
        })
        .body(|body| {
            body.rows(22.0, result.row_count, |mut row| {
                let row_idx = row.index();

                // 1. Row index column
                row.col(|ui| {
                    ui.label(
                        RichText::new(format!("{}", row_idx + 1))
                            .size(11.0)
                            .color(COLOR_TEXT_MUTED),
                    );
                });

                // 2. Data columns
                if let Some(row_data) = result.rows.get(row_idx) {
                    for cell in row_data {
                        row.col(|ui| {
                            render_cell(ui, cell);
                        });
                    }
                }
            });
        });
}

/// Formats and renders an individual SqlValue cell with type-specific styling.
fn render_cell(ui: &mut Ui, cell: &SqlValue) {
    match cell {
        SqlValue::Null => {
            ui.label(
                RichText::new("NULL")
                    .italics()
                    .color(COLOR_TEXT_MUTED)
                    .size(11.0),
            );
        }
        SqlValue::Boolean(b) => {
            let (label, color) = if *b {
                ("TRUE", COLOR_SUCCESS)
            } else {
                ("FALSE", COLOR_TEXT_MUTED)
            };
            ui.label(RichText::new(label).strong().color(color).size(11.0));
        }
        SqlValue::TinyInt(n) => {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.monospace(n.to_string());
            });
        }
        SqlValue::SmallInt(n) => {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.monospace(n.to_string());
            });
        }
        SqlValue::Integer(n) => {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.monospace(n.to_string());
            });
        }
        SqlValue::BigInt(n) => {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.monospace(n.to_string());
            });
        }
        SqlValue::HugeInt(n) => {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.monospace(n.to_string());
            });
        }
        SqlValue::Float(f) => {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.monospace(format!("{:.4}", f));
            });
        }
        SqlValue::Double(d) => {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.monospace(format!("{:.4}", d));
            });
        }
        SqlValue::Text(s) => {
            ui.label(RichText::new(s).color(COLOR_TEXT_PRIMARY).size(12.0));
        }
        SqlValue::Date(d) => {
            ui.monospace(d.to_string());
        }
        SqlValue::Timestamp(ts) => {
            ui.monospace(ts.format("%Y-%m-%d %H:%M:%S").to_string());
        }
        SqlValue::Blob(bytes) => {
            ui.label(
                RichText::new(format!("<Blob {} bytes>", bytes.len()))
                    .color(COLOR_TEXT_MUTED)
                    .size(11.0),
            );
        }
    }
}
