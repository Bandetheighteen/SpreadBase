import pandas as pd

class Exporter:
    @staticmethod
    def export_to_csv(data, headers, file_path):
        df = pd.DataFrame(data, columns=headers)
        df.to_csv(file_path, index=False)
        return True

    @staticmethod
    def export_to_xlsx(data, headers, file_path):
        df = pd.DataFrame(data, columns=headers)
        df.to_excel(file_path, index=False)
        return True

    @staticmethod
    def export_to_json(data, headers, file_path):
        df = pd.DataFrame(data, columns=headers)
        df.to_json(file_path, orient="records", indent=4)
        return True

    @staticmethod
    def export_to_parquet(data, headers, file_path):
        df = pd.DataFrame(data, columns=headers)
        df.to_parquet(file_path, index=False)
        return True
