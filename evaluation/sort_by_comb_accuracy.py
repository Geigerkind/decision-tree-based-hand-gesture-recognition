import pandas as pd
import numpy as np

data = pd.read_csv("./saad_c8.csv").query("optimization_level == 'Os' and feature_set == 2 and forest_size <= 10 and ensemble_technique != 3")
data["sort_val"] = data["accuracy_klisch"] + data["accuracy_dymel_gesture"] + data["accuracy_dymel_null"]
data = data.dropna()

data.loc[(data["sort_val"]).sort_values().index].iloc[::-1].to_csv("./best_configs.csv")