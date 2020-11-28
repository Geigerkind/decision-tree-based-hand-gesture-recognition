import pandas as pd

# Manually tested
default_size = 2508

data = pd.read_csv("./size_and_accuracy_data.csv").query("feature_set == 2 and ensamble_technique == 1")

max_size_in_bytes = 45000 + default_size

min_size = 99999999
max_accuracy = 0
max_depth = 0
forest_size = 0
set_fraction = 0
ensamble_technique = 0
feature_set = 0
for item in data.query("forest_bytes <= " + str(max_size_in_bytes) + " and forest_bytes > 0").iterrows():
    if (item[1].forest_bytes < min_size and item[1].accuracy == max_accuracy) or item[1].accuracy > max_accuracy:
        max_accuracy = item[1].accuracy
        min_size = item[1].forest_bytes
        max_depth = item[1].max_depth
        forest_size = item[1].forest_size
        feature_set = item[1].feature_set
        ensamble_technique = item[1].ensamble_technique
        set_fraction = item[1].set_fraction

print("Best feasible result:")
print("Max depth: " + str(max_depth))
print("Forest size: " + str(forest_size))
print("Forest bytes: " + str(min_size))
print("Set fraction: " + str(set_fraction))
print("Ensamble technique: " + str(ensamble_technique))
print("Feature set: " + str(feature_set))
print("Accuracy: " + str(max_accuracy))
