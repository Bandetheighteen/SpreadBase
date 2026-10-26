//! SQL Query Editor Component
//!
//! Provides a monospace, multi-line SQL code editor with text selection tracking.

use egui::{Frame, Margin, RichText, Rounding, Stroke, TextEdit, Ui};

use super::theme::{COLOR_BORDER, COLOR_CODE_BG, COLOR_PRIMARY, COLOR_TEXT_MUTED};

/// State and options for the SQL editor.
#[derive(Debug, Clone, Default)]
pub struct SqlEditorState {
    /// Currently highlighted text fragment (if any)
    pub selected_fragment: Option<String>,
}

/// Renders the multi-line SQL query editor and tracks active text selection.
pub fn render_sql_editor(
    ui: &mut Ui,
    state: &mut SqlEditorState,
    query: &mut String,
) {

    // Editor top control bar
    ui.horizontal(|ui| {
        ui.label(RichText::new("SQL QUERY").strong().size(11.0).color(COLOR_TEXT_MUTED));

        if let Some(ref frag) = state.selected_fragment {
            if !frag.trim().is_empty() {
                ui.add_space(8.0);
                ui.label(
                    RichText::new(format!("⚡ [Running selection: {} chars]", frag.len()))
                        .color(COLOR_PRIMARY)
                        .size(11.0),
                );
            }
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button("Clear").clicked() {
                query.clear();
            }
            if ui.small_button("Sample").clicked() {
                *query = "SELECT 1 AS id, 'Apple' AS name, 1.50 AS price\nUNION ALL\nSELECT 2, 'Banana', 0.80;".to_string();
            }
        });
    });

    ui.add_space(4.0);

    // Monospace multi-line text edit
    Frame::none()
        .fill(COLOR_CODE_BG)
        .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
        .rounding(Rounding::same(4.0))
        .inner_margin(Margin::same(8.0))
        .show(ui, |ui| {
            let edit = TextEdit::multiline(query)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .desired_rows(6)
                .code_editor();

            let output = edit.show(ui);

            // Track selection range
            if let Some(cursor_range) = output.cursor_range {
                let p = cursor_range.primary.ccursor.index;
                let s = cursor_range.secondary.ccursor.index;
                let min_idx = p.min(s);
                let max_idx = p.max(s);

                if min_idx < max_idx {
                    let fragment = query
                        .chars()
                        .skip(min_idx)
                        .take(max_idx - min_idx)
                        .collect::<String>();
                    state.selected_fragment = Some(fragment);
                } else {
                    state.selected_fragment = None;
                }
            } else {
                state.selected_fragment = None;
            }
        });
}
