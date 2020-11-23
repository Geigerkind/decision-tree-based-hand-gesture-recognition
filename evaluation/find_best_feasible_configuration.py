import pandas as pd

# Manually tested
default_size = 2508

data_size = pd.read_csv("./size_data.csv")
data_accuracy = pd.read_csv("./accuracy_data.csv")

max_size_in_bytes = 25000 + default_size

min_size = 99999999
max_accuracy = 0
max_depth = 0
forest_size = 0
for item in data_size.query("forest_bytes <= " + str(max_size_in_bytes) + " and forest_bytes > 0").iterrows():
    # Find matching accuracy
    accuracy = data_accuracy.query(
        "max_depth == " + str(item[1].max_depth) + " and forest_size == " + str(item[1].forest_size)).accuracy.iloc[
        0]
    if item[1].forest_bytes <= min_size or accuracy > max_accuracy:
        max_accuracy = accuracy
        min_size = item[1].forest_bytes
        max_depth = item[1].max_depth
        forest_size = item[1].forest_size


print("Best feasible result:")
print("Max depth: " + str(max_depth))
print("Forest size: " + str(forest_size))
print("Forest bytes: " + str(min_size))
print("Accuracy: " + str(max_accuracy))