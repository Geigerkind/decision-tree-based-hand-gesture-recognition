import pandas as pd

# Manually tested
default_size = 2508

data = pd.read_csv("./size_and_accuracy_data_klisch_int.csv")

max_size_in_bytes = 25000 + default_size

min_size = 99999999
max_accuracy = 0
max_depth = 0
forest_size = 0
for item in data.query("forest_bytes <= " + str(max_size_in_bytes) + " and forest_bytes > 0").iterrows():
    if (item[1].forest_bytes < min_size and item[1].accuracy == max_accuracy) or item[1].accuracy > max_accuracy:
        max_accuracy = item[1].accuracy
        min_size = item[1].forest_bytes
        max_depth = item[1].max_depth
        forest_size = item[1].forest_size


print("Best feasible result:")
print("Max depth: " + str(max_depth))
print("Forest size: " + str(forest_size))
print("Forest bytes: " + str(min_size))
print("Accuracy: " + str(max_accuracy))