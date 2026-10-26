import duckdb
import pandas as pd
import threading
from PySide6.QtCore import QObject, Signal, QRunnable, QThreadPool

class QueryWorkerSignals(QObject):
    finished = Signal(dict)
    error = Signal(str)

class QueryWorker(QRunnable):
    def __init__(self, db_manager, query):
        super().__init__()
        self.db_manager = db_manager
        self.query = query
        self.signals = QueryWorkerSignals()
        
    def run(self):
        try:
            result = self.db_manager._execute_sync(self.query)
            self.signals.finished.emit(result)
        except Exception as e:
            self.signals.error.emit(str(e))

class DuckDBManager:
    def __init__(self):
        # We use a file-backed or memory database. Let's stick to memory for the IDE.
        self.conn = duckdb.connect(':memory:')
        self.registered_tables = {}
        # Keep references to pandas dataframes so DuckDB can query them by variable name
        self.dataframes = {}

    def register_csv(self, file_path, table_name):
        """Registers a CSV natively in DuckDB for fastest performance."""
        file_path_clean = file_path.replace("'", "''")
        query = f"CREATE OR REPLACE VIEW {table_name} AS SELECT * FROM read_csv_auto('{file_path_clean}');"
        self._execute_sync(query)
        self.registered_tables[table_name] = {"type": "csv", "path": file_path}

    def register_excel(self, file_path, sheet_name, table_name):
        """Reads an Excel sheet into Pandas and loads it into a DuckDB table."""
        df = pd.read_excel(file_path, sheet_name=sheet_name)
        # Create a real table from the DataFrame so it's accessible globally across threads
        self.conn.execute(f"CREATE TABLE {table_name} AS SELECT * FROM df")
        self.registered_tables[table_name] = {"type": "excel", "path": file_path, "sheet": sheet_name}

    def unregister(self, table_name):
        if table_name in self.registered_tables:
            if self.registered_tables[table_name]['type'] == 'excel':
                self.conn.execute(f"DROP TABLE IF EXISTS {table_name}")
            else:
                self._execute_sync(f"DROP VIEW IF EXISTS {table_name}")
            del self.registered_tables[table_name]

    def _execute_sync(self, query):
        """Internal synchronous execution."""
        # DuckDB conn can be shared across threads if properly handled, or we create a cursor
        # Actually DuckDB connections are thread-safe for reading but better to create a cursor for concurrent execution
        cursor = self.conn.cursor()
        cursor.execute(query)
        
        # Depending on query type, fetchall might not return data (like CREATE VIEW)
        description = cursor.description
        if description:
            columns = [desc[0] for desc in description]
            # Fetch as pandas df or raw tuples? Tuples for GUI model
            data = cursor.fetchall()
            cursor.close()
            return {"success": True, "columns": columns, "data": data}
        else:
            cursor.close()
            return {"success": True, "columns": [], "data": []}

    def execute_query_async(self, query, callback_success, callback_error):
        """Executes a query in a background thread and calls signals."""
        worker = QueryWorker(self, query)
        worker.signals.finished.connect(callback_success)
        worker.signals.error.connect(callback_error)
        QThreadPool.globalInstance().start(worker)

    def get_schema(self, table_name):
        try:
            res = self._execute_sync(f"DESCRIBE {table_name};")
            return res.get("data", [])
        except Exception:
            return []
