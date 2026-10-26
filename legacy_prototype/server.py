import os
import sys
import webbrowser
import threading
import json
import duckdb
import pandas as pd
from http.server import HTTPServer, SimpleHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
import cgi

# Global DuckDB connection
conn = duckdb.connect(':memory:')
registered_tables = []

def get_resource_path():
    if hasattr(sys, '_MEIPASS'):
        return sys._MEIPASS
    return os.path.dirname(os.path.abspath(__file__))

class APIServer(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

    def do_OPTIONS(self):
        self.send_response(200, "ok")
        self.end_headers()
        
    def translate_path(self, path):
        # Serve React frontend from dist folder
        path = super().translate_path(path)
        relpath = os.path.relpath(path, os.getcwd())
        dist_path = os.path.join(get_resource_path(), 'dist', relpath)
        if not os.path.exists(dist_path) and not '.' in relpath:
            return os.path.join(get_resource_path(), 'dist', 'index.html')
        return dist_path

    def do_POST(self):
        if self.path == '/api/query':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data.decode('utf-8'))
            query = data.get('query', '')
            
            try:
                res = conn.execute(query).fetchdf()
                # Convert datetime to string for json serialization if needed
                res = res.astype(str)
                result_json = res.to_dict(orient='records')
                self.send_response(200)
                self.send_header('Content-type', 'application/json')
                self.end_headers()
                self.wfile.write(json.dumps(result_json).encode('utf-8'))
            except Exception as e:
                self.send_response(500)
                self.send_header('Content-type', 'application/json')
                self.end_headers()
                self.wfile.write(json.dumps({"error": str(e)}).encode('utf-8'))
                
        elif self.path == '/api/load':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data.decode('utf-8'))
            file_path = data.get('filePath', '')
            
            try:
                # Load Excel
                xl = pd.ExcelFile(file_path)
                tables_loaded = []
                for sheet in xl.sheet_names:
                    df = pd.read_excel(file_path, sheet_name=sheet)
                    table_name = "sheet_" + "".join([c if c.isalnum() else "_" for c in sheet]).lower()
                    
                    # Create table using AS SELECT * FROM df (duckdb automatically uses pandas)
                    conn.register('temp_df', df)
                    conn.execute(f"CREATE TABLE {table_name} AS SELECT * FROM temp_df")
                    conn.unregister('temp_df')
                    
                    tables_loaded.append(table_name)
                    registered_tables.append(table_name)
                    
                self.send_response(200)
                self.send_header('Content-type', 'application/json')
                self.end_headers()
                self.wfile.write(json.dumps({"success": True, "tables": tables_loaded}).encode('utf-8'))
            except Exception as e:
                self.send_response(500)
                self.send_header('Content-type', 'application/json')
                self.end_headers()
                self.wfile.write(json.dumps({"error": str(e)}).encode('utf-8'))

def start_server():
    server = HTTPServer(('127.0.0.1', 8080), APIServer)
    server.serve_forever()

if __name__ == '__main__':
    threading.Thread(target=start_server, daemon=True).start()
    webbrowser.open('http://127.0.0.1:8080')
    
    import tkinter as tk
    root = tk.Tk()
    root.title("LocalSQL Studio Background Server")
    root.geometry("400x150")
    tk.Label(root, text="LocalSQL Studio is running in your browser.\n\nClose this window to shut down the server.", pady=30).pack()
    root.mainloop()
