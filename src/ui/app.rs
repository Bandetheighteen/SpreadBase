//! LocalSqlApp — Main Desktop Application Shell
//!
//! Provides the primary IDE layout using eframe/egui:
//! - Top toolbar with file opening, query execution, and status
//! - Left sidebar for database and table exploration
//! - Central panel for query editing and result inspection
//! - Bottom status bar with engine metrics and row counts

use std::thread;

use crossbeam_channel::{Receiver, Sender};
use eframe::App;
use egui::{
    vec2, Align, Button, CentralPanel, Color32, Context, Frame, Key, Layout, Margin,
    RichText, Rounding, SidePanel, Stroke, TopBottomPanel, Ui,
};

use crate::db::types::QueryResult;
use super::editor::{render_sql_editor, SqlEditorState};
use super::grid::render_results_table;
use super::sidebar::{render_sidebar, SidebarAction, SidebarState};
use super::theme::{
    apply_theme, COLOR_BORDER, COLOR_CODE_BG, COLOR_ERROR, COLOR_PANEL_BG, COLOR_PRIMARY,
    COLOR_SUCCESS, COLOR_TEXT_MUTED, COLOR_TEXT_PRIMARY,
};
use super::worker::{start_worker, TableInfo, WorkerCommand, WorkerEvent};

/// Main application state struct.
pub struct LocalSqlApp {
    /// Channel to send commands to the background worker
    pub worker_tx: Sender<WorkerCommand>,
    /// Channel to receive events from the background worker
    pub worker_rx: Receiver<WorkerEvent>,
    /// Ingested table information displayed in the sidebar
    pub tables: Vec<TableInfo>,
    /// State of the left sidebar explorer
    pub sidebar_state: SidebarState,
    /// State of the SQL editor (e.g. selection)
    pub editor_state: SqlEditorState,
    /// Current query execution result (if any)
    pub current_result: Option<QueryResult>,
    /// Active query error message (if any)
    pub error_message: Option<String>,
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
        let (worker_tx, worker_rx) = start_worker();
        Self {
            worker_tx,
            worker_rx,
            tables: Vec::new(),
            sidebar_state: SidebarState::default(),
            editor_state: SqlEditorState::default(),
            current_result: None,
            error_message: None,
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

    /// Triggers native OS file picker and dispatches selection to the background worker.
    pub fn pick_and_load_files(&mut self) {
        let tx = self.worker_tx.clone();
        self.is_busy = true;
        self.status_message = "Selecting files...".to_string();

        thread::spawn(move || {
            if let Some(files) = rfd::FileDialog::new()
                .add_filter("Data Files (*.xlsx, *.xls, *.csv)", &["xlsx", "xls", "csv"])
                .set_title("Select Excel or CSV Files")
                .pick_files()
            {
                let _ = tx.send(WorkerCommand::IngestFiles(files));
            }
        });
    }

    /// Dispatches query execution to background worker.
    /// Runs selected text fragment if highlighted; otherwise runs full query buffer.
    pub fn trigger_query_execution(&mut self) {
        let sql = if let Some(ref frag) = self.editor_state.selected_fragment {
            if !frag.trim().is_empty() {
                frag.clone()
            } else {
                self.query_text.clone()
            }
        } else {
            self.query_text.clone()
        };

        if sql.trim().is_empty() {
            self.status_message = "Query is empty".to_string();
            return;
        }

        self.is_busy = true;
        self.error_message = None;
        self.status_message = "Executing query...".to_string();
        let _ = self.worker_tx.send(WorkerCommand::ExecuteQuery {
            query_id: 1,
            sql,
        });
    }

    /// Exports active query results to an Excel workbook.
    pub fn export_excel(&mut self) {
        if let Some(ref res) = self.current_result {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name("results.xlsx")
                .add_filter("Excel Workbook (*.xlsx)", &["xlsx"])
                .save_file()
            {
                match crate::export::export_to_excel(res, &path, Some("Results")) {
                    Ok(_) => {
                        let fname = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        self.status_message = format!("Exported {} rows to {}", res.row_count, fname);
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Export failed: {}", e));
                    }
                }
            }
        }
    }

    /// Exports active query results to a CSV file.
    pub fn export_csv(&mut self) {
        if let Some(ref res) = self.current_result {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name("results.csv")
                .add_filter("CSV File (*.csv)", &["csv"])
                .save_file()
            {
                match crate::export::export_to_csv(res, &path) {
                    Ok(_) => {
                        let fname = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        self.status_message = format!("Exported {} rows to {}", res.row_count, fname);
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Export failed: {}", e));
                    }
                }
            }
        }
    }

    /// Exports active query results to a JSON file.
    pub fn export_json(&mut self) {
        if let Some(ref res) = self.current_result {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name("results.json")
                .add_filter("JSON File (*.json)", &["json"])
                .save_file()
            {
                match crate::export::export_to_json(res, &path) {
                    Ok(_) => {
                        let fname = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        self.status_message = format!("Exported {} rows to {}", res.row_count, fname);
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Export failed: {}", e));
                    }
                }
            }
        }
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
                self.pick_and_load_files();
            }

            ui.add_space(8.0);

            // Run Query Action Button
            let run_btn = ui.add(
                Button::new(RichText::new("▶ Run Query (F5)").color(Color32::WHITE).strong())
                    .fill(COLOR_PRIMARY)
                    .min_size(vec2(120.0, 26.0)),
            );
            if run_btn.clicked() {
                self.trigger_query_execution();
            }

            // Export Actions (enabled only when rows are present)
            let has_rows = self.current_result.as_ref().map(|r| r.row_count > 0).unwrap_or(false);
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            ui.add_enabled_ui(has_rows, |ui| {
                if ui.button(RichText::new("📊 Export Excel").color(COLOR_TEXT_PRIMARY)).clicked() {
                    self.export_excel();
                }
                if ui.button(RichText::new("💾 Export CSV").color(COLOR_TEXT_PRIMARY)).clicked() {
                    self.export_csv();
                }
                if ui.button(RichText::new("📄 Export JSON").color(COLOR_TEXT_PRIMARY)).clicked() {
                    self.export_json();
                }
            });

            // Right-aligned engine status indicator
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(8.0);
                if self.is_busy {
                    ui.label(RichText::new("⏳ Working...").color(COLOR_TEXT_MUTED));
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

        // Global Keyboard Shortcuts (F5 or Ctrl+Enter to execute query)
        if ctx.input(|i| i.key_pressed(Key::F5) || (i.modifiers.command && i.key_pressed(Key::Enter))) {
            self.trigger_query_execution();
        }

        // Poll incoming messages from background worker
        while let Ok(event) = self.worker_rx.try_recv() {
            self.is_busy = false;
            match event {
                WorkerEvent::FilesIngested { tables, errors } => {
                    let count = tables.len();
                    for t in tables {
                        if let Some(pos) = self.tables.iter().position(|existing| existing.table_name == t.table_name) {
                            self.tables[pos] = t;
                        } else {
                            self.tables.push(t);
                        }
                    }
                    self.table_count = self.tables.len();

                    if !errors.is_empty() {
                        self.status_message = format!("Ingested {} table(s) with warnings: {}", count, errors.join("; "));
                    } else if count > 0 {
                        self.status_message = format!("Successfully loaded {} table(s)", count);
                        if self.sidebar_state.selected_table.is_none() {
                            if let Some(first) = self.tables.first() {
                                self.sidebar_state.selected_table = Some(first.table_name.clone());
                                self.query_text = format!("SELECT * FROM \"{}\" LIMIT 100;", first.table_name);
                            }
                        }
                    } else {
                        self.status_message = "No tables found in selected files.".to_string();
                    }
                }
                WorkerEvent::QueryFinished { query_id: _, result } => {
                    match result {
                        Ok(qr) => {
                            self.last_row_count = qr.row_count;
                            self.last_exec_ms = qr.execution_time_ms;
                            self.status_message = format!(
                                "Query executed: {} rows returned in {:.2} ms",
                                qr.row_count, qr.execution_time_ms
                            );
                            self.error_message = None;
                            self.current_result = Some(qr);
                        }
                        Err(err) => {
                            self.status_message = "Query Failed".to_string();
                            self.error_message = Some(err);
                        }
                    }
                }
            }
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
                if let Some(action) = render_sidebar(ui, &mut self.sidebar_state, &self.tables) {
                    match action {
                        SidebarAction::SelectTable(tbl) | SidebarAction::InsertQuery(tbl) => {
                            self.query_text = format!("SELECT * FROM \"{}\" LIMIT 100;", tbl);
                            self.status_message = format!("Selected table: \"{}\"", tbl);
                            self.trigger_query_execution();
                        }
                    }
                }
            });

        // Central Workspace: Query Editor & Live Results Grid
        CentralPanel::default()
            .frame(Frame::none().fill(COLOR_CODE_BG).inner_margin(Margin::same(12.0)))
            .show(ctx, |ui| {
                // Diagnostic Error Banner (if query failed)
                if let Some(ref err) = self.error_message {
                    let mut dismiss = false;
                    Frame::none()
                        .fill(Color32::from_rgb(45, 20, 20))
                        .stroke(Stroke::new(1.0_f32, COLOR_ERROR))
                        .rounding(Rounding::same(4.0))
                        .inner_margin(Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("⚠ Query Error:").color(COLOR_ERROR).strong());
                                ui.label(RichText::new(err).color(COLOR_TEXT_PRIMARY).monospace());
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.small_button("✖").clicked() {
                                        dismiss = true;
                                    }
                                });
                            });
                        });
                    if dismiss {
                        self.error_message = None;
                    }
                    ui.add_space(8.0);
                }

                // Multi-line SQL Query Editor
                render_sql_editor(ui, &mut self.editor_state, &mut self.query_text);

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Live Results Table Grid
                if let Some(ref res) = self.current_result {
                    render_results_table(ui, res);
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.label(
                            RichText::new("No query results yet")
                                .color(COLOR_TEXT_MUTED)
                                .size(13.0),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new("Click '▶ Run Query (F5)' above or press Ctrl+Enter to execute.")
                                .color(COLOR_TEXT_MUTED)
                                .size(11.0),
                        );
                    });
                }
            });
    }
}
