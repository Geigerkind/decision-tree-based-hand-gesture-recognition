import pandas as pd

# Manually tested
default_size = 2508

data = pd.read_csv("./saad_c4.csv")

max_size_in_bytes = 45000 + default_size

min_size = 99999999
max_accuracy_klisch = 0
max_accuracy_dymel_gesture = 0
max_accuracy_dymel_null = 0
max_depth = 0
forest_size = 0
set_fraction = 0
ensemble_technique = 0
feature_set = 0
ccp_alpha = 0
min_leaf_sample = 0
for item in data.query("forest_bytes <= " + str(max_size_in_bytes) + " and forest_bytes > 0").iterrows():
    if (item[1].forest_bytes < min_size and item[1].accuracy_klisch == max_accuracy_klisch) or item[1].accuracy_klisch > max_accuracy_klisch:
        max_accuracy_klisch = item[1].accuracy_klisch
        max_accuracy_dymel_gesture = item[1].accuracy_dymel_gesture
        max_accuracy_dymel_null = item[1].accuracy_dymel_null
        min_size = item[1].forest_bytes
        max_depth = item[1].max_depth
        forest_size = item[1].forest_size
        feature_set = item[1].feature_set
        ensemble_technique = item[1].ensemble_technique
        set_fraction = item[1].set_fraction
        ccp_alpha = item[1].ccp_alpha
        min_leaf_sample = item[1].min_leaf_sample

print("Best feasible result:")
print("Max depth: " + str(max_depth))
print("Forest size: " + str(forest_size))
print("Forest bytes: " + str(min_size))
print("Set fraction: " + str(set_fraction))
print("Ensemble technique: " + str(ensemble_technique))
print("Feature set: " + str(feature_set))
print("CCP Alpha: " + str(ccp_alpha))
print("Min Leaf Sample: " + str(min_leaf_sample))
print("Accuracy Klisch: " + str(max_accuracy_klisch))
print("Accuracy Dymel Gesture: " + str(max_accuracy_dymel_gesture))
print("Accuracy Dymel Null: " + str(max_accuracy_dymel_null))
