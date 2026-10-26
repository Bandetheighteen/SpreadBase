from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QTreeWidget, QTreeWidgetItem, 
    QLineEdit, QLabel, QComboBox
)
from PySide6.QtCore import Qt

class ExplorerPanel(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setup_ui()

    def setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        
        # Search bar
        self.search_bar = QLineEdit()
        self.search_bar.setPlaceholderText("Search files or tables...")
        layout.addWidget(self.search_bar)
        
        # Filter combo
        self.filter_combo = QComboBox()
        self.filter_combo.addItems(["All", "Sources", "Tables", "Queries"])
        layout.addWidget(self.filter_combo)
        
        # Tree widget
        self.tree = QTreeWidget()
        self.tree.setHeaderHidden(True)
        layout.addWidget(self.tree)
        
        # Initialize sections
        self.sources_item = QTreeWidgetItem(self.tree, ["Sources"])
        self.tables_item = QTreeWidgetItem(self.tree, ["Tables"])
        self.views_item = QTreeWidgetItem(self.tree, ["Views"])
        self.queries_item = QTreeWidgetItem(self.tree, ["Queries"])
        
        self.sources_item.setExpanded(True)
        self.tables_item.setExpanded(True)
        self.views_item.setExpanded(True)
        self.queries_item.setExpanded(True)
        
        # Workspace selector at bottom
        self.workspace_combo = QComboBox()
        self.workspace_combo.addItem("Default Workspace")
        layout.addWidget(self.workspace_combo)
