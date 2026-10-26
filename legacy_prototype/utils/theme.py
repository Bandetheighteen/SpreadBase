class Theme:
    # Colors
    BACKGROUND = "#1E1E1E"             # VS Code dark background
    SECONDARY_BACKGROUND = "#252526"    # Panel backgrounds
    PANEL_BACKGROUND = "#2D2D30"        # Slightly lighter 
    BORDER = "#3F3F46"                  # Subtle border
    PRIMARY_ACCENT = "#007ACC"          # VS Code Blue
    PRIMARY_ACCENT_HOVER = "#1C97EA"    
    SUCCESS = "#89D185"                 # Green
    WARNING = "#CCA700"                 # Amber
    ERROR = "#F48771"                   # Red
    TEXT_PRIMARY = "#CCCCCC"            # Base text
    TEXT_SECONDARY = "#858585"          # Muted text
    SELECTION_BG = "#264F78"            # Selection blue
    SELECTION_FG = "#FFFFFF"

    # Fonts
    FONT_FAMILY = "Segoe UI"
    SQL_FONT_FAMILY = "Consolas"
    FONT_SIZE = 13
    SQL_FONT_SIZE = 14

    @classmethod
    def get_stylesheet(cls):
        return f"""
        QWidget {{
            background-color: {cls.BACKGROUND};
            color: {cls.TEXT_PRIMARY};
            font-family: "{cls.FONT_FAMILY}";
            font-size: {cls.FONT_SIZE}px;
        }}
        
        QMainWindow {{
            background-color: {cls.BACKGROUND};
        }}
        
        QMenuBar {{
            background-color: {cls.SECONDARY_BACKGROUND};
            border-bottom: 1px solid {cls.BORDER};
        }}
        QMenuBar::item {{
            spacing: 3px; 
            padding: 4px 8px;
            background: transparent;
            border-radius: 4px;
        }}
        QMenuBar::item:selected {{
            background-color: {cls.PANEL_BACKGROUND};
        }}
        QMenuBar::item:pressed {{
            background-color: {cls.PRIMARY_ACCENT};
            color: {cls.SELECTION_FG};
        }}
        
        QMenu {{
            background-color: {cls.SECONDARY_BACKGROUND};
            border: 1px solid {cls.BORDER};
            padding: 4px;
        }}
        QMenu::item {{
            padding: 4px 20px 4px 20px;
            border-radius: 4px;
        }}
        QMenu::item:selected {{
            background-color: {cls.PRIMARY_ACCENT};
            color: {cls.SELECTION_FG};
        }}
        
        QToolBar {{
            background-color: {cls.SECONDARY_BACKGROUND};
            border-bottom: 1px solid {cls.BORDER};
            padding: 4px;
            spacing: 4px;
        }}
        QToolButton {{
            background-color: transparent;
            border: none;
            padding: 4px;
            border-radius: 4px;
        }}
        QToolButton:hover {{
            background-color: {cls.PANEL_BACKGROUND};
        }}
        QToolButton:pressed {{
            background-color: {cls.BORDER};
        }}
        
        QDockWidget::title {{
            background-color: {cls.SECONDARY_BACKGROUND};
            padding: 6px;
            border-bottom: 1px solid {cls.BORDER};
        }}
        
        QTreeWidget, QTableView, QListView {{
            background-color: {cls.BACKGROUND};
            border: 1px solid {cls.BORDER};
            border-radius: 4px;
            alternate-background-color: {cls.SECONDARY_BACKGROUND};
            selection-background-color: {cls.SELECTION_BG};
            selection-color: {cls.SELECTION_FG};
            padding: 2px;
        }}
        QTreeWidget::item:hover, QTableView::item:hover, QListView::item:hover {{
            background-color: {cls.PANEL_BACKGROUND};
        }}
        QTreeWidget::item:selected, QTableView::item:selected, QListView::item:selected {{
            background-color: {cls.SELECTION_BG};
            color: {cls.SELECTION_FG};
        }}
        
        QHeaderView::section {{
            background-color: {cls.SECONDARY_BACKGROUND};
            color: {cls.TEXT_PRIMARY};
            border: none;
            border-right: 1px solid {cls.BORDER};
            border-bottom: 1px solid {cls.BORDER};
            padding: 4px 6px;
            font-weight: bold;
        }}
        
        QTextEdit, QPlainTextEdit {{
            background-color: {cls.BACKGROUND};
            border: 1px solid {cls.BORDER};
            border-radius: 4px;
            padding: 4px;
            selection-background-color: {cls.SELECTION_BG};
            selection-color: {cls.SELECTION_FG};
        }}
        
        QPushButton {{
            background-color: {cls.PRIMARY_ACCENT};
            color: #FFFFFF;
            border: none;
            padding: 6px 16px;
            border-radius: 4px;
            font-weight: bold;
        }}
        QPushButton:hover {{
            background-color: {cls.PRIMARY_ACCENT_HOVER};
        }}
        QPushButton:pressed {{
            background-color: {cls.SELECTION_BG};
        }}
        QPushButton:disabled {{
            background-color: {cls.PANEL_BACKGROUND};
            color: {cls.TEXT_SECONDARY};
        }}
        
        QLineEdit {{
            background-color: {cls.BACKGROUND};
            border: 1px solid {cls.BORDER};
            border-radius: 4px;
            padding: 4px 8px;
            selection-background-color: {cls.SELECTION_BG};
        }}
        QLineEdit:focus {{
            border: 1px solid {cls.PRIMARY_ACCENT};
        }}
        
        QScrollBar:vertical {{
            border: none;
            background: {cls.BACKGROUND};
            width: 14px;
            margin: 0px 0px 0px 0px;
        }}
        QScrollBar::handle:vertical {{
            background: {cls.BORDER};
            min-height: 20px;
            border-radius: 7px;
            margin: 2px;
        }}
        QScrollBar::handle:vertical:hover {{
            background: {cls.TEXT_SECONDARY};
        }}
        QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical {{
            height: 0px;
        }}
        
        QScrollBar:horizontal {{
            border: none;
            background: {cls.BACKGROUND};
            height: 14px;
            margin: 0px 0px 0px 0px;
        }}
        QScrollBar::handle:horizontal {{
            background: {cls.BORDER};
            min-width: 20px;
            border-radius: 7px;
            margin: 2px;
        }}
        QScrollBar::handle:horizontal:hover {{
            background: {cls.TEXT_SECONDARY};
        }}
        QScrollBar::add-line:horizontal, QScrollBar::sub-line:horizontal {{
            width: 0px;
        }}
        
        QTabWidget::pane {{
            border: 1px solid {cls.BORDER};
            border-radius: 4px;
        }}
        QTabBar::tab {{
            background-color: {cls.SECONDARY_BACKGROUND};
            border: 1px solid {cls.BORDER};
            border-bottom: none;
            border-top-left-radius: 4px;
            border-top-right-radius: 4px;
            padding: 6px 12px;
            margin-right: 2px;
        }}
        QTabBar::tab:selected {{
            background-color: {cls.BACKGROUND};
            border-bottom: 2px solid {cls.PRIMARY_ACCENT};
            color: {cls.TEXT_PRIMARY};
        }}
        QTabBar::tab:hover:!selected {{
            background-color: {cls.PANEL_BACKGROUND};
        }}
        """
