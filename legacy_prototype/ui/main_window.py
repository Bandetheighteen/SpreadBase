from PySide6.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, 
    QSplitter, QStatusBar, QToolBar, QLabel, QPushButton, QFileDialog, QMessageBox, QSizePolicy
)
from PySide6.QtCore import Qt, QSize
from utils.theme import Theme
from ui.explorer import ExplorerPanel
from ui.sql_editor import SQLEditorPanel
from ui.results_grid import ResultsGridPanel
from database.duckdb_manager import DuckDBManager
from export.exporter import Exporter
import os
import time

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("LocalSQL Studio")
        self.resize(1536, 1024)
        self.setMinimumSize(1100, 700)
        
        # Application Icon
        logo_path = os.path.join(os.path.dirname(os.path.dirname(__file__)), "sources", "LOGO.png")
        if os.path.exists(logo_path):
            from PySide6.QtGui import QIcon
            self.setWindowIcon(QIcon(logo_path))

        self.setStyleSheet(Theme.get_stylesheet())
        
        self.db = DuckDBManager()
        self.current_results = None
        self.current_headers = None
        
        # Setup UI regions
        self.setup_toolbar()
        self.setup_workspace()
        self.setup_statusbar()
        
        # Connect signals
        self.editor_panel.run_btn.clicked.connect(self.run_query)
        self.add_file_btn.clicked.connect(self.add_file)
        self.export_btn.clicked.connect(self.export_results)

    def setup_toolbar(self):
        toolbar = QToolBar("Main Toolbar")
        toolbar.setMovable(False)
        toolbar.setIconSize(QSize(20, 20))
        self.addToolBar(Qt.TopToolBarArea, toolbar)
        
        # Left controls
        from utils.icon_manager import IconManager
        
        self.add_file_btn = QPushButton(IconManager.get_icon("file"), "Add File")
        self.add_folder_btn = QPushButton(IconManager.get_icon("folder"), "Add Folder")
        self.refresh_btn = QPushButton(IconManager.get_icon("refresh"), "Refresh")
        
        toolbar.addWidget(self.add_file_btn)
        toolbar.addWidget(self.add_folder_btn)
        toolbar.addWidget(self.refresh_btn)
        
        # Spacer
        spacer = QWidget()
        spacer.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Preferred)
        toolbar.addWidget(spacer)
        
        # Center status
        self.offline_label = QLabel("🟢 Offline Mode: All data is local")
        self.offline_label.setStyleSheet(f"color: {Theme.SUCCESS}; font-weight: bold;")
        toolbar.addWidget(self.offline_label)
        
        # Spacer
        spacer2 = QWidget()
        spacer2.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Preferred)
        toolbar.addWidget(spacer2)
        
        # Right controls
        self.save_query_btn = QPushButton(IconManager.get_icon("file"), "Save Query")
        self.export_btn = QPushButton(IconManager.get_icon("export"), "Export")
        self.history_btn = QPushButton(IconManager.get_icon("query"), "History")
        self.settings_btn = QPushButton(IconManager.get_icon("settings"), "Settings")
        
        toolbar.addWidget(self.save_query_btn)
        toolbar.addWidget(self.export_btn)
        toolbar.addWidget(self.history_btn)
        toolbar.addWidget(self.settings_btn)

    def setup_workspace(self):
        main_splitter = QSplitter(Qt.Horizontal)
        
        self.explorer_panel = ExplorerPanel()
        self.explorer_panel.setMinimumWidth(380)
        self.explorer_panel.setStyleSheet(f"background-color: {Theme.SECONDARY_BACKGROUND}; padding: 8px;")
        
        right_splitter = QSplitter(Qt.Vertical)
        self.editor_panel = SQLEditorPanel()
        self.results_panel = ResultsGridPanel()
        
        right_splitter.addWidget(self.editor_panel)
        right_splitter.addWidget(self.results_panel)
        right_splitter.setSizes([400, 600])
        
        main_splitter.addWidget(self.explorer_panel)
        main_splitter.addWidget(right_splitter)
        main_splitter.setSizes([380, 1156])
        
        self.setCentralWidget(main_splitter)

    def setup_statusbar(self):
        self.status = QStatusBar()
        self.setStatusBar(self.status)
        self.status.showMessage("Ready | Default Workspace")

    def run_query(self):
        query = self.editor_panel.get_current_sql()
        if not query.strip():
            return
            
        self.status.showMessage("Executing query...")
        self.editor_panel.run_btn.setEnabled(False)
        self.start_time = time.time()
        
        self.db.execute_query_async(query, self._on_query_success, self._on_query_error)

    def _on_query_success(self, result):
        self.editor_panel.run_btn.setEnabled(True)
        elapsed = time.time() - self.start_time
        
        if result["success"]:
            self.current_results = result["data"]
            self.current_headers = result["columns"]
            self.results_panel.set_data(result["data"], result["columns"])
            row_count = len(result["data"])
            self.results_panel.show_message(f"Success: Returned {row_count} rows in {elapsed:.3f} seconds.")
            self.status.showMessage(f"Ready | {row_count} rows | {elapsed:.3f}s")
        else:
            self._on_query_error(result.get("error", "Unknown error"))

    def _on_query_error(self, error_msg):
        self.editor_panel.run_btn.setEnabled(True)
        self.results_panel.show_message(f"Error: {error_msg}", switch_tab=True, is_error=True)
        self.status.showMessage("Error executing query")

    def add_file(self):
        file_paths, _ = QFileDialog.getOpenFileNames(
            self, "Select Data Files", "", "Supported Files (*.csv *.xlsx);;CSV Files (*.csv);;Excel Files (*.xlsx)"
        )
        if not file_paths:
            return
            
        for path in file_paths:
            filename = os.path.basename(path)
            base_name = os.path.splitext(filename)[0]
            table_name = "".join([c if c.isalnum() else "_" for c in base_name]).lower()
            
            try:
                if path.endswith(".csv"):
                    self.db.register_csv(path, table_name)
                    self.results_panel.show_message(f"Registered CSV: {path} as table '{table_name}'")
                elif path.endswith((".xlsx", ".xls")):
                    # For MVP: just register the first sheet, or all sheets.
                    from sources.file_parser import FileParser
                    sheets = FileParser.get_excel_sheets(path)
                    for sheet in sheets:
                        sheet_safe = "".join([c if c.isalnum() else "_" for c in sheet]).lower()
                        full_table_name = f"{table_name}_{sheet_safe}"
                        self.db.register_excel(path, sheet, full_table_name)
                        self.results_panel.show_message(f"Registered Excel Sheet '{sheet}' as table '{full_table_name}'")
            except Exception as e:
                self.results_panel.show_message(f"Failed to load {filename}: {str(e)}", switch_tab=True, is_error=True)

    def export_results(self):
        if not self.current_results:
            QMessageBox.warning(self, "Export", "No results to export. Run a query first.")
            return

        file_path, selected_filter = QFileDialog.getSaveFileName(
            self, "Export Results", "", 
            "CSV (*.csv);;Excel (*.xlsx);;JSON (*.json);;Parquet (*.parquet)"
        )
        
        if not file_path:
            return
            
        try:
            if file_path.endswith(".csv"):
                Exporter.export_to_csv(self.current_results, self.current_headers, file_path)
            elif file_path.endswith(".xlsx"):
                Exporter.export_to_xlsx(self.current_results, self.current_headers, file_path)
            elif file_path.endswith(".json"):
                Exporter.export_to_json(self.current_results, self.current_headers, file_path)
            elif file_path.endswith(".parquet"):
                Exporter.export_to_parquet(self.current_results, self.current_headers, file_path)
                
            self.results_panel.show_message(f"Successfully exported {len(self.current_results)} rows to {file_path}")
        except Exception as e:
            QMessageBox.critical(self, "Export Error", f"Failed to export: {str(e)}")
