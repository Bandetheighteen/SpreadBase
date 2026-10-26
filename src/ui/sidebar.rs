//! Sidebar Schema Tree & Data Explorer
//!
//! Renders loaded files, sheets/tables, and columns with inferred data types
//! in a responsive, collapsible tree hierarchy.

use std::collections::BTreeMap;

use egui::{CollapsingHeader, Layout, RichText, ScrollArea, TextEdit, Ui};

use super::theme::{COLOR_PRIMARY, COLOR_TEXT_MUTED, COLOR_TEXT_PRIMARY};
use super::worker::TableInfo;

/// Actions emitted by the sidebar when users interact with tables.
#[derive(Debug, Clone, PartialEq)]
pub enum SidebarAction {
    /// Select a table to display or generate query for
    SelectTable(String),
    /// Insert a SELECT * template into the editor
    InsertQuery(String),
}

/// State for the sidebar explorer.
#[derive(Debug, Clone, Default)]
pub struct SidebarState {
    /// Name of currently selected table
    pub selected_table: Option<String>,
    /// Search filter query
    pub search_query: String,
}

/// Renders the sidebar explorer and returns any triggered user actions.
pub fn render_sidebar(
    ui: &mut Ui,
    state: &mut SidebarState,
    tables: &[TableInfo],
) -> Option<SidebarAction> {
    let mut action = None;

    // Header with table count badge
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("DATA EXPLORER")
                .size(11.0)
                .strong()
                .color(COLOR_TEXT_MUTED),
        );
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{}", tables.len()))
                    .size(11.0)
                    .color(COLOR_PRIMARY)
                    .strong(),
            );
        });
    });

    ui.add_space(6.0);

    // Search filter input box
    ui.horizontal(|ui| {
        ui.add(
            TextEdit::singleline(&mut state.search_query)
                .hint_text("🔍 Filter tables or columns...")
                .desired_width(ui.available_width()),
        );
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(6.0);

    if tables.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            ui.label(RichText::new("No files loaded").color(COLOR_TEXT_MUTED).size(13.0));
            ui.add_space(6.0);
            ui.label(
                RichText::new("Click '📂 Open File(s)' above\nto load Excel or CSV files.")
                    .color(COLOR_TEXT_MUTED)
                    .size(11.0),
            );
        });
        return None;
    }

    // Filter tables based on search query
    let query_lower = state.search_query.trim().to_lowercase();
    let filtered_tables: Vec<&TableInfo> = tables
        .iter()
        .filter(|t| {
            if query_lower.is_empty() {
                true
            } else {
                t.table_name.to_lowercase().contains(&query_lower)
                    || t.file_name.to_lowercase().contains(&query_lower)
                    || t.columns.iter().any(|(c, _)| c.to_lowercase().contains(&query_lower))
            }
        })
        .collect();

    // Group tables by source file
    let mut grouped_by_file: BTreeMap<&str, Vec<&TableInfo>> = BTreeMap::new();
    for tbl in &filtered_tables {
        grouped_by_file
            .entry(&tbl.file_name)
            .or_default()
            .push(tbl);
    }

    ScrollArea::vertical().show(ui, |ui| {
        for (file_name, file_tables) in grouped_by_file {
            let file_header = format!("📁 {} ({})", file_name, file_tables.len());

            CollapsingHeader::new(RichText::new(file_header).strong().color(COLOR_TEXT_PRIMARY))
                .default_open(true)
                .show(ui, |ui| {
                    for tbl in file_tables {
                        let is_selected = state.selected_table.as_deref() == Some(&tbl.table_name);
                        let table_label = format!("📊 {} ({})", tbl.table_name, tbl.row_count);

                        CollapsingHeader::new(
                            RichText::new(table_label)
                                .color(if is_selected { COLOR_PRIMARY } else { COLOR_TEXT_PRIMARY })
                        )
                        .default_open(false)
                        .show(ui, |ui| {
                            // Quick Action: Insert Query Template
                            if ui.button(RichText::new("⚡ Query Table").size(11.0).color(COLOR_PRIMARY)).clicked() {
                                state.selected_table = Some(tbl.table_name.clone());
                                action = Some(SidebarAction::InsertQuery(tbl.table_name.clone()));
                            }

                            ui.add_space(4.0);

                            // Columns list
                            for (col_name, col_type) in &tbl.columns {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("🔹").size(10.0).color(COLOR_PRIMARY));
                                    ui.label(RichText::new(col_name).size(12.0).color(COLOR_TEXT_PRIMARY));
                                    ui.label(RichText::new(col_type).size(11.0).color(COLOR_TEXT_MUTED));
                                });
                            }
                            ui.add_space(4.0);
                        });
                    }
                });
            ui.add_space(4.0);
        }
    });

    action
}
