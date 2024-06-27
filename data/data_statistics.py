import pandas as pd 

file = 'INT03.csv'
df = pd.read_csv(file)

pc_counts = df['pc'].value_counts()

print("Dataset statistics:")
print(pc_counts)