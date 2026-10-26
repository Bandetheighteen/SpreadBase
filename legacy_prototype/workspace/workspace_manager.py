import json
import os

class WorkspaceManager:
    def __init__(self, workspace_file="workspace.json"):
        self.workspace_file = workspace_file
        self.state = {
            "sources": {},    # registered files/folders
            "tables": {},     # table mappings
            "history": [],    # query history
            "saved_queries": {}
        }
        self.load_workspace()

    def load_workspace(self):
        if os.path.exists(self.workspace_file):
            try:
                with open(self.workspace_file, "r") as f:
                    self.state = json.load(f)
            except Exception:
                pass

    def save_workspace(self):
        with open(self.workspace_file, "w") as f:
            json.dump(self.state, f, indent=4)

    def add_source(self, path, source_type):
        self.state["sources"][path] = {"type": source_type}
        self.save_workspace()

    def remove_source(self, path):
        if path in self.state["sources"]:
            del self.state["sources"][path]
            self.save_workspace()

    def add_table(self, table_name, file_path, sheet_name=None):
        self.state["tables"][table_name] = {
            "path": file_path,
            "sheet": sheet_name
        }
        self.save_workspace()

    def log_query(self, sql, success, execution_time):
        self.state["history"].append({
            "sql": sql,
            "success": success,
            "time": execution_time
        })
        # Keep only last 100 queries
        if len(self.state["history"]) > 100:
            self.state["history"].pop(0)
        self.save_workspace()
