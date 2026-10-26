from PySide6.QtGui import QSyntaxHighlighter, QTextCharFormat, QColor, QFont
from PySide6.QtCore import QRegularExpression

class SqlSyntaxHighlighter(QSyntaxHighlighter):
    def __init__(self, document):
        super().__init__(document)

        self.highlighting_rules = []

        # Keywords (Blue)
        keyword_format = QTextCharFormat()
        keyword_format.setForeground(QColor("#569CD6")) 
        keyword_format.setFontWeight(QFont.Bold)
        keywords = [
            "SELECT", "FROM", "WHERE", "JOIN", "INNER", "LEFT", "RIGHT", "OUTER",
            "ON", "AS", "AND", "OR", "NOT", "ORDER", "BY", "GROUP", "HAVING",
            "LIMIT", "OFFSET", "INSERT", "INTO", "VALUES", "UPDATE", "SET",
            "DELETE", "CREATE", "TABLE", "DROP", "ALTER", "INDEX", "VIEW",
            "NULL", "IS", "EXISTS", "IN", "BETWEEN", "LIKE", "ASC", "DESC"
        ]
        for keyword in keywords:
            pattern = QRegularExpression(rf"\b{keyword}\b")
            pattern.setPatternOptions(QRegularExpression.CaseInsensitiveOption)
            self.highlighting_rules.append((pattern, keyword_format))

        # Strings (Orange/Brown)
        string_format = QTextCharFormat()
        string_format.setForeground(QColor("#CE9178")) 
        self.highlighting_rules.append((QRegularExpression(r"\"[^\"]*\""), string_format))
        self.highlighting_rules.append((QRegularExpression(r"'[^']*'"), string_format))

        # Single-line Comments (Greenish Grey)
        comment_format = QTextCharFormat()
        comment_format.setForeground(QColor("#6A9955")) 
        self.highlighting_rules.append((QRegularExpression(r"--[^\n]*"), comment_format))
        
        # Numbers (Light Green)
        number_format = QTextCharFormat()
        number_format.setForeground(QColor("#B5CEA8")) 
        self.highlighting_rules.append((QRegularExpression(r"\b\d+\b"), number_format))
        self.highlighting_rules.append((QRegularExpression(r"\b\d+\.\d+\b"), number_format))

        # Setup for multi-line comments
        self.multi_line_comment_format = comment_format
        self.comment_start_expression = QRegularExpression(r"/\*")
        self.comment_end_expression = QRegularExpression(r"\*/")

    def highlightBlock(self, text):
        # Apply standard rules
        for pattern, format in self.highlighting_rules:
            match_iterator = pattern.globalMatch(text)
            while match_iterator.hasNext():
                match = match_iterator.next()
                self.setFormat(match.capturedStart(), match.capturedLength(), format)

        # Apply multi-line comments
        self.setCurrentBlockState(0)
        start_index = 0
        if self.previousBlockState() != 1:
            match = self.comment_start_expression.match(text)
            start_index = match.capturedStart()

        while start_index >= 0:
            match = self.comment_end_expression.match(text, start_index)
            end_index = match.capturedStart()
            comment_length = 0

            if end_index == -1:
                self.setCurrentBlockState(1)
                comment_length = len(text) - start_index
            else:
                comment_length = end_index - start_index + match.capturedLength()

            self.setFormat(start_index, comment_length, self.multi_line_comment_format)
            match_next = self.comment_start_expression.match(text, start_index + comment_length)
            start_index = match_next.capturedStart()
