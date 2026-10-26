from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QTabWidget, QTableView,
    QPushButton, QLabel, QComboBox, QTextEdit, QHeaderView, QLineEdit
)
from PySide6.QtCore import Qt, QAbstractTableModel, QSortFilterProxyModel
from PySide6.QtGui import QIcon

class VirtualTableModel(QAbstractTableModel):
    def __init__(self, data=None, headers=None, parent=None):
        super().__init__(parent)
        self._data = data or []
        self._headers = headers or []

    def set_data(self, data, headers):
        self.beginResetModel()
        self._data = data
        self._headers = headers
        self.endResetModel()

    def rowCount(self, parent=None):
        return len(self._data)

    def columnCount(self, parent=None):
        return len(self._headers)

    def data(self, index, role=Qt.DisplayRole):
        if not index.isValid():
            return None
            
        if role == Qt.DisplayRole:
            val = self._data[index.row()][index.column()]
            return str(val) if val is not None else "NULL"
            
        # Add alternating colors or custom alignment if needed
        return None

    def headerData(self, section, orientation, role=Qt.DisplayRole):
        if role == Qt.DisplayRole:
            if orientation == Qt.Horizontal:
                return self._headers[section]
            if orientation == Qt.Vertical:
                return str(section + 1)
        return None

class ResultsGridPanel(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setup_ui()

    def setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(0)
        
        self.tabs = QTabWidget()
        layout.addWidget(self.tabs)
        
        # Results Tab
        self.results_tab = QWidget()
        results_layout = QVBoxLayout(self.results_tab)
        results_layout.setContentsMargins(0, 0, 0, 0)
        results_layout.setSpacing(0)
        
        # Toolbar for results
        res_toolbar = QHBoxLayout()
        res_toolbar.setContentsMargins(8, 4, 8, 4)
        
        self.search_input = QLineEdit()
        self.search_input.setPlaceholderText("Filter current view...")
        self.search_input.setMaximumWidth(250)
        res_toolbar.addWidget(self.search_input)
        res_toolbar.addStretch()
        
        results_layout.addLayout(res_toolbar)
        
        self.table_view = QTableView()
        self.table_view.setAlternatingRowColors(True)
        self.table_view.horizontalHeader().setStretchLastSection(True)
        self.table_view.verticalHeader().setDefaultSectionSize(28)
        results_layout.addWidget(self.table_view)
        
        self.source_model = VirtualTableModel()
        self.proxy_model = QSortFilterProxyModel()
        self.proxy_model.setSourceModel(self.source_model)
        self.proxy_model.setFilterCaseSensitivity(Qt.CaseInsensitive)
        self.proxy_model.setFilterKeyColumn(-1) # Filter all columns
        
        self.table_view.setModel(self.proxy_model)
        self.table_view.setSortingEnabled(True)
        
        self.search_input.textChanged.connect(self.proxy_model.setFilterWildcard)
        
        # Messages Tab
        self.messages_tab = QWidget()
        messages_layout = QVBoxLayout(self.messages_tab)
        messages_layout.setContentsMargins(0, 0, 0, 0)
        
        self.messages_text = QTextEdit()
        self.messages_text.setReadOnly(True)
        self.messages_text.setStyleSheet("font-family: Consolas, monospace;")
        messages_layout.addWidget(self.messages_text)
        
        self.tabs.addTab(self.results_tab, "Results")
        self.tabs.addTab(self.messages_tab, "Messages")

    def set_data(self, data, headers):
        self.source_model.set_data(data, headers)
        self.tabs.setCurrentWidget(self.results_tab)
        
    def show_message(self, text, switch_tab=True, is_error=False):
        prefix = "❌ " if is_error else "✅ "
        self.messages_text.append(f"{prefix}{text}")
        if switch_tab:
            self.tabs.setCurrentWidget(self.messages_tab)
            
    def clear_messages(self):
        self.messages_text.clear()
