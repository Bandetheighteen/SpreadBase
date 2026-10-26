//! Desktop GUI components — eframe/egui application shell and panels.

pub mod app;
pub mod theme;

pub use app::LocalSqlApp;

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
