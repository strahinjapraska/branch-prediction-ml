import pandas as pd
import os 


def clean_data(file_path):
    whole_df = pd.read_csv(file_path)
    total_columns = len(whole_df.columns)

    columns = list(range(1,33)) + [total_columns - 1]
    df = pd.read_csv(file_path, usecols= columns)
    df = df.astype(int)

    to_int32 = lambda row: int(''.join(row.astype(str)),2)
    df['pc'] = df.iloc[:, :32].apply(to_int32, axis = 1)
    df.rename(columns = {df.columns[-2]: 'outcome'}, inplace=True)

    df = df[['pc', 'outcome']]
    df.to_csv(file_path, index = False)


files = os.listdir('.')
csv_files = [file for file in files if file.endswith('.csv')]


for csv in csv_files:
    clean_data(csv)