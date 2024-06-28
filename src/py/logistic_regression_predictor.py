import numpy as np
import pandas as pd
from sklearn.linear_model import SGDClassifier
from sklearn.metrics import accuracy_score
import time 


files = ["I04.csv", "INT03.csv", "MM03.csv", "MM05.csv", "S02.csv", "S04.csv"]

start_time = time.time()

for f in files: 
    df = pd.read_csv('../../data/'+f)

    X = df['pc'].values
    y = df['outcome'].values

    model = SGDClassifier(loss='log_loss', learning_rate='constant', eta0=0.01)


    X = np.array(X)
    y = np.array(y)

    mispredictions = 0

    for i in range(len(X)):
        X_sample = np.array([[X[i]]])
        y_sample = np.array([y[i]])

        model.partial_fit(X_sample, y_sample, classes=[0, 1])

        prediction = model.predict(X_sample)[0]
        
        if prediction != y_sample:
            mispredictions += 1
    
    print(f+":"+ str(mispredictions))

end_time = time.time()
print(end_time - start_time)