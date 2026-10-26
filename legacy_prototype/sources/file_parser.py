import pandas as pd
import openpyxl
import os

class FileParser:
    @staticmethod
    def get_excel_sheets(file_path):
        """Returns a list of sheet names from an Excel file."""
        if not os.path.exists(file_path):
            raise FileNotFoundError(f"File not found: {file_path}")
        try:
            wb = openpyxl.load_workbook(file_path, read_only=True, data_only=True)
            sheet_names = wb.sheetnames
            wb.close()
            return sheet_names
        except Exception as e:
            raise Exception(f"Failed to read excel sheets from {file_path}: {e}")

    @staticmethod
    def get_schema(file_path, sheet_name=None):
        """Returns a list of dictionaries with column names and inferred SQL types."""
        if not os.path.exists(file_path):
            raise FileNotFoundError(f"File not found: {file_path}")
        
        try:
            # Read a small chunk of the file to infer types
            if file_path.lower().endswith('.csv'):
                df = pd.read_csv(file_path, nrows=100)
            elif file_path.lower().endswith(('.xls', '.xlsx')):
                df = pd.read_excel(file_path, sheet_name=sheet_name, nrows=100)
            else:
                raise ValueError("Unsupported file extension. Only CSV and Excel files are supported.")
            
            if df.empty:
                return []
                
            schema = []
            for col in df.columns:
                dtype = df[col].dtype
                sql_type = FileParser._map_dtype_to_sql(dtype)
                schema.append({"name": str(col), "type": sql_type})
                
            return schema
        except pd.errors.EmptyDataError:
            return [] # Empty CSV
        except Exception as e:
            raise Exception(f"Failed to infer schema for {file_path}: {e}")

    @staticmethod
    def _map_dtype_to_sql(dtype):
        """Maps pandas datatypes to SQL types."""
        if pd.api.types.is_integer_dtype(dtype):
            return "INTEGER"
        elif pd.api.types.is_float_dtype(dtype):
            return "DOUBLE"
        elif pd.api.types.is_bool_dtype(dtype):
            return "BOOLEAN"
        elif pd.api.types.is_datetime64_any_dtype(dtype):
            return "DATE"
        else:
            return "VARCHAR"
