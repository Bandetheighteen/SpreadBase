from PySide6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QTabWidget, 
    QPlainTextEdit, QPushButton, QToolBar
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QFont, QKeySequence, QShortcut
from utils.theme import Theme
from ui.line_number_area import SqlCodeEditor
from ui.syntax_highlighter import SqlSyntaxHighlighter

class SQLEditorPanel(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setup_ui()

    def setup_ui(self):
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        
        # Toolbar
        self.toolbar = QToolBar("SQL Toolbar")
        self.run_btn = QPushButton("Run (F5)")
        self.explain_btn = QPushButton("Explain")
        self.format_btn = QPushButton("Format SQL")
        
        self.toolbar.addWidget(self.run_btn)
        self.toolbar.addWidget(self.explain_btn)
        self.toolbar.addWidget(self.format_btn)
        
        layout.addWidget(self.toolbar)
        
        # Tab Widget
        self.tabs = QTabWidget()
        self.tabs.setTabsClosable(True)
        self.tabs.tabCloseRequested.connect(self.close_tab)
        layout.addWidget(self.tabs)
        
        # Add initial tab
        self.add_tab("Query 1")
        
        # Shortcuts
        QShortcut(QKeySequence("F5"), self, self.run_btn.click)
        QShortcut(QKeySequence("Ctrl+T"), self, lambda: self.add_tab("New Query"))

    def add_tab(self, name, content=""):
        editor = SqlCodeEditor()
        # The font is handled inside SqlCodeEditor via stylesheet, but we can override or just keep it
        editor.setPlainText(content)
        editor.setPlaceholderText("Write your SQL query here...")
        
        # Apply syntax highlighter
        highlighter = SqlSyntaxHighlighter(editor.document())
        
        # Keep a reference so it isn't garbage collected
        editor._highlighter = highlighter
        
        self.tabs.addTab(editor, name)
        self.tabs.setCurrentWidget(editor)

    def close_tab(self, index):
        if self.tabs.count() > 1:
            self.tabs.removeTab(index)

    def get_current_sql(self):
        current_editor = self.tabs.currentWidget()
        if current_editor:
            selected_text = current_editor.textCursor().selectedText()
            if selected_text:
                return selected_text
            return current_editor.toPlainText()
        return ""
