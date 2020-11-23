import numpy as np
import pandas as pd
from matplotlib import pyplot as plt

# Manually tested
default_size = 2508

data_size = pd.read_csv("./size_data.csv")

df = pd.DataFrame()

for item in data_size.groupby(["max_depth", "forest_size"]):
    df = df.append({"max_depth": item[1].max_depth.iloc[0], "forest_size": item[1].forest_size.iloc[0],
                    "O0": np.nan if item[1].forest_bytes.iloc[0] == -1 else item[1].forest_bytes.iloc[0] - default_size,
                    "Os": np.nan if item[1].forest_bytes.iloc[1] == -1 else item[1].forest_bytes.iloc[1] - default_size,
                    "O2": np.nan if item[1].forest_bytes.iloc[2] == -1 else item[1].forest_bytes.iloc[2] - default_size,
                    "O3": np.nan if item[1].forest_bytes.iloc[3] == -1 else item[1].forest_bytes.iloc[3] - default_size},
                   ignore_index=True)

for max_depth in range(31)[1:]:
    fig = plt.figure(figsize=(10, 10))
    fig = df.query("forest_size <= 32 and max_depth==" + str(max_depth)).plot(x="forest_size",
                                                                              y=["O0", "Os", "O2", "O3"])
    plt.plot(range(65), np.repeat(32000, 65), '--')
    fig.axes.set_xlim([0, 64])
    fig.axes.set_ylim([0, fig.axes.get_ylim()[1]])
    plt.savefig('./plotting/program_size_plots/forest_bytes_md_' + str(max_depth) + '.png', format='png')
    plt.close('all')
