import pandas as pd 
import numpy as np 
import tensorflow as tf 

from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense
from tensorflow.keras.optimizers import Adam


df = pd.read_csv('../../data/I04.csv')

X = df['pc'].values
y = df['outcome'].values 

model = Sequential()
model.add(Dense(16, input_dim =1, activation='relu'))
model.add(Dense(1, activation = 'sigmoid'))

model.compile(optimizer=Adam(learning_rate =0.01), loss = 'binary_crossentropy', metrics=['accuracy'])

epochs = 1 
batch_size = 1 

X = np.array(X)
y = np.array(y) 

mispredictions = 0 

for i in range(len(X)): 
    X_sample = np.array([[X[i]]])
    y_sample = np.array([[y[i]]])
    
    prediction = model.predict(X_sample)
    predicted_label = 1 if prediction >= 0.5 else 0 

    if predicted_label != y_sample: 
        mispredictions+=1 

    model.train_on_batch(X_sample, y_sample)

print(mispredictions)