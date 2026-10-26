//! LocalSqlApp — Main Desktop Application Shell
//!
//! Provides the primary IDE layout using eframe/egui:
//! - Top toolbar with file opening, query execution, and status
//! - Left sidebar for database and table exploration
//! - Central panel for query editing and result inspection
//! - Bottom status bar with engine metrics and row counts

use eframe::App;
use egui::{
    vec2, Align, Button, CentralPanel, Color32, Context, Frame, Layout, Margin,
    RichText, SidePanel, Stroke, TopBottomPanel, Ui,
};

use super::theme::{
    apply_theme, COLOR_BORDER, COLOR_CODE_BG, COLOR_PANEL_BG, COLOR_PRIMARY,
    COLOR_SUCCESS, COLOR_TEXT_MUTED, COLOR_TEXT_PRIMARY,
};

/// Main application state struct.
pub struct LocalSqlApp {
    /// Status message displayed in the status bar
    pub status_message: String,
    /// Active SQL query string
    pub query_text: String,
    /// Total tables loaded in memory
    pub table_count: usize,
    /// Total rows returned by the last query
    pub last_row_count: usize,
    /// Execution time in milliseconds of the last query
    pub last_exec_ms: f64,
    /// Whether the background worker is currently busy
    pub is_busy: bool,
    /// Flag indicating whether theme has been applied
    theme_applied: bool,
}

impl Default for LocalSqlApp {
    fn default() -> Self {
        Self {
            status_message: "Ready — Open an Excel or CSV file to begin".to_string(),
            query_text: "-- Select a table on the left or type your SQL query here\nSELECT 1 AS status, 'LocalSQL Studio Ready' AS message;".to_string(),
            table_count: 0,
            last_row_count: 0,
            last_exec_ms: 0.0,
            is_busy: false,
            theme_applied: false,
        }
    }
}

impl LocalSqlApp {
    /// Creates a new instance of the application.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Renders the top toolbar with action buttons.
    fn render_top_toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // App Branding
            ui.add_space(4.0);
            ui.label(
                RichText::new("⚡ LocalSQL Studio")
                    .strong()
                    .color(COLOR_PRIMARY)
                    .size(15.0),
            );

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);

            // Open File(s) Action Button
            let open_btn = ui.add(
                Button::new(RichText::new("📂 Open File(s)").color(COLOR_TEXT_PRIMARY))
                    .min_size(vec2(100.0, 26.0)),
            );
            if open_btn.clicked() {
                self.status_message = "Opening file dialog...".to_string();
                // Handled in plan 02-02 via rfd
            }

            ui.add_space(8.0);

            // Run Query Action Button
            let run_btn = ui.add(
                Button::new(RichText::new("▶ Run Query (F5)").color(Color32::WHITE).strong())
                    .fill(COLOR_PRIMARY)
                    .min_size(vec2(120.0, 26.0)),
            );
            if run_btn.clicked() {
                self.status_message = "Executing query...".to_string();
                // Handled in phase 3
            }

            // Right-aligned engine status indicator
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(8.0);
                if self.is_busy {
                    ui.label(RichText::new("⏳ Processing...").color(COLOR_TEXT_MUTED));
                } else {
                    ui.label(RichText::new("● Engine Ready").color(COLOR_SUCCESS).size(12.0));
                }
            });
        });
    }

    /// Renders the bottom status bar with row counts and query metrics.
    fn render_status_bar(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Engine status icon and message
            ui.label(RichText::new("ℹ").color(COLOR_PRIMARY));
            ui.label(RichText::new(&self.status_message).color(COLOR_TEXT_MUTED).size(12.0));

            // Right-aligned statistics
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(8.0);
                ui.label(
                    RichText::new("DuckDB (In-Memory)")
                        .color(COLOR_TEXT_MUTED)
                        .size(11.0),
                );
                ui.separator();

                if self.last_row_count > 0 || self.last_exec_ms > 0.0 {
                    ui.label(
                        RichText::new(format!(
                            "{} rows in {:.2} ms",
                            self.last_row_count, self.last_exec_ms
                        ))
                        .color(COLOR_TEXT_PRIMARY)
                        .size(12.0),
                    );
                    ui.separator();
                }

                ui.label(
                    RichText::new(format!("Tables: {}", self.table_count))
                        .color(COLOR_TEXT_MUTED)
                        .size(12.0),
                );
            });
        });
    }
}

impl App for LocalSqlApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.theme_applied {
            apply_theme(ctx);
            self.theme_applied = true;
        }

        // Top Toolbar
        TopBottomPanel::top("top_toolbar")
            .frame(
                Frame::none()
                    .fill(COLOR_PANEL_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .inner_margin(Margin::symmetric(10.0, 8.0)),
            )
            .show(ctx, |ui| {
                self.render_top_toolbar(ui);
            });

        // Bottom Status Bar
        TopBottomPanel::bottom("status_bar")
            .frame(
                Frame::none()
                    .fill(COLOR_PANEL_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .inner_margin(Margin::symmetric(10.0, 5.0)),
            )
            .show(ctx, |ui| {
                self.render_status_bar(ui);
            });

        // Left Sidebar: File and Table Explorer
        SidePanel::left("sidebar_panel")
            .resizable(true)
            .min_width(220.0)
            .default_width(260.0)
            .frame(
                Frame::none()
                    .fill(COLOR_PANEL_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .inner_margin(Margin::same(10.0)),
            )
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("DATA EXPLORER")
                        .size(11.0)
                        .strong()
                        .color(COLOR_TEXT_MUTED),
                );
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                if self.table_count == 0 {
                    ui.vertical_centered(|ui| {
                        ui.add_space(24.0);
                        ui.label(
                            RichText::new("No files loaded")
                                .color(COLOR_TEXT_MUTED)
                                .size(13.0),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new("Click 'Open File(s)' above to load Excel or CSV files.")
                                .color(COLOR_TEXT_MUTED)
                                .size(11.0),
                        );
                    });
                }
            });

        // Central Workspace: Query Editor & Results Table
        CentralPanel::default()
            .frame(Frame::none().fill(COLOR_CODE_BG).inner_margin(Margin::same(12.0)))
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("SQL WORKSPACE")
                        .size(11.0)
                        .strong()
                        .color(COLOR_TEXT_MUTED),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new(&self.query_text)
                        .monospace()
                        .color(COLOR_TEXT_PRIMARY),
                );
            });
    }
}
