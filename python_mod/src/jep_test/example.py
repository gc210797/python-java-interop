import pandas as pd

def slice_csv(url: str) -> pd.DataFrame:
    return pd.read_csv(url).to_numpy()
