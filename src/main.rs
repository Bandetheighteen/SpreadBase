//! LocalSQL Studio — Application Entry Point

fn main() -> Result<(), eframe::Error> {
    localsql_studio::ui::run_native_app()
}
