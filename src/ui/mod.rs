//! Desktop GUI components — eframe/egui application shell and panels.

pub mod app;
pub mod editor;
pub mod grid;
pub mod sidebar;
pub mod theme;
pub mod worker;

pub use app::LocalSqlApp;
pub use editor::{render_sql_editor, SqlEditorState};
pub use grid::render_results_table;
pub use sidebar::{render_sidebar, SidebarAction, SidebarState};
pub use worker::{start_worker, TableInfo, WorkerCommand, WorkerEvent};

/// Configures and launches the native desktop window.
pub fn run_native_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("LocalSQL Studio")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "LocalSQL Studio",
        options,
        Box::new(|cc| Ok(Box::new(LocalSqlApp::new(cc)))),
    )
}
