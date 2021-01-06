import pandas as pd
import numpy as np

d1 = pd.read_csv("./saad_c8.csv").query(
    "optimization_level == 'Os' and ensemble_technique != 3 and feature_set != 3 and feature_set != 5 and feature_set != 6 and feature_set != 7")
d2 = pd.read_csv("./saad_c9.csv").query("optimization_level == 'Os' and ensemble_technique != 3 and feature_set != 7")
d3 = pd.read_csv("./saad_c10.csv").query("optimization_level == 'Os' and feature_set != 2 and feature_set != 7")
d4 = pd.read_csv("./saad_c11.csv").query("optimization_level == 'Os'")

#data = pd.concat([d1, d2, d3, d4], ignore_index=True).query("feature_set == 7 and forest_size <= 7 and max_depth <= 15")
data = pd.concat([d1, d2, d3, d4], ignore_index=True).query("feature_set == 7")

data["sort_val"] = data["accuracy_klisch"] + data["accuracy_dymel_gesture"] + data["accuracy_dymel_null"]
data = data.dropna()

data.loc[(data["sort_val"]).sort_values().index].iloc[::-1].to_csv("./best_configs.csv")