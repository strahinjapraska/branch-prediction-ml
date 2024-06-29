import matplotlib.pyplot as plt
import numpy as np

# Data
class_a = [479, 864, 91, 0, 5413, 35999]
print(class_a)
class_b = [2952, 197, 17209, 60589, 19572, 17724]

# Normalize data
for i in range(len(class_a)): 
    if i != 2: 
        class_a[i] = 100.0 - (class_a[i] / 400000.0) * 100.0
        class_b[i] = 100.0 - (class_b[i] / 400000.0) * 100.0
    else: 
        class_a[i] = 100.0 - (class_a[i] / 300000.0) * 100.0
        class_b[i] = 100.0 - (class_b[i] / 300000.0) * 100.0

print(class_a)
# Combine data and colors
values = []
colors = []

for a, b in zip(class_a, class_b):
    values.append(a)
    colors.append('blue')
    values.append(b)
    colors.append('red')

positions = np.arange(len(values))

# Create the bar chart
plt.bar(positions, values, color=colors)

# Add title and labels
plt.xlabel('Index')
plt.ylabel('Values')

# Set custom x-axis labels
plt.xticks(positions, ['A1', 'B1', 'A2', 'B2', 'A3', 'B3', 'A4', 'B4', 'A5', 'B5', 'A6', 'B6'])

# Create proxy artists for the legend
blue_patch = plt.Line2D([0], [0], color='blue', lw=4)
red_patch = plt.Line2D([0], [0], color='red', lw=4)

# Add the legend
plt.legend([blue_patch, red_patch], ['Logistic Regression', 'Perceptron'])

# Show plot
plt.show()
