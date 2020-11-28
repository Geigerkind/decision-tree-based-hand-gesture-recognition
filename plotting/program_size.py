import numpy as np
import pandas as pd
from matplotlib import pyplot as plt

# Manually tested
default_size = 2508

data_size = pd.read_csv("./size_and_accuracy_data.csv").query("ensamble_technique == 1 and feature_set == 1")

df = pd.DataFrame()

for item in data_size.groupby(["max_depth", "forest_size"]):
    df = df.append({"max_depth": item[1].max_depth.iloc[0], "forest_size": item[1].forest_size.iloc[0],
                    "O0": np.nan if item[1].forest_bytes.iloc[0] == -1 else item[1].forest_bytes.iloc[0] - default_size,
                    "Os": np.nan if item[1].forest_bytes.iloc[1] == -1 else item[1].forest_bytes.iloc[1] - default_size,
                    "O2": np.nan if item[1].forest_bytes.iloc[2] == -1 else item[1].forest_bytes.iloc[2] - default_size,
                    "O3": np.nan if item[1].forest_bytes.iloc[3] == -1 else item[1].forest_bytes.iloc[3] - default_size,
                    "Accuracy": item[1].accuracy.iloc[0]},
                   ignore_index=True)

# Draw x=Tree depth y= Size in bytes vs Accuracy
tree_data = data = df.query("forest_size == 1")

plt.rcParams.update({'font.size': 30})

plt.figure(figsize=(30, 30))
figur, ax1 = plt.subplots()
# plt.title("Decision tree")
fig = tree_data.plot(ax=ax1, x="max_depth", y=["O0", "Os", "O2", "O3"])
ax1.set_xlabel("Max-Tiefe")
ax1.set_ylabel("Größe in bytes")
ax1.set_xlim([1, 25])
ax1.set_ylim([0, ax1.get_ylim()[1]])
plt.plot([9, 9], [0, ax1.get_ylim()[1]], '--')

ax2 = ax1.twinx()
data.plot(ax=ax2, x="max_depth", y="Accuracy", colormap="Pastel1")
ax2.set_ylabel("Genauigkeit")
ax2.set_ylim([0, 1])

ax1.legend(loc="upper left", bbox_to_anchor=(0, 0.9))
ax2.legend(loc="upper left", bbox_to_anchor=(0, 1))

#figur.tight_layout()
figur.set_size_inches(15, 11)
plt.savefig('./plotting/program_size_plots/forest_bytes_md_0.png', format='png', dpi=100, pad_inches=0.2, bbox_inches="tight")
plt.close('all')


# Draw x=Forest size y= Size in bytes vs. Accuracy graphs with growing
for max_depth in range(31)[1:]:
    plt.figure(figsize=(15, 15))
    figur, ax1 = plt.subplots()
    # plt.title("Decision forest - Max depth: " + str(max_depth))

    data = df.query("forest_size <= 32 and max_depth==" + str(max_depth))
    fig = data.plot(ax=ax1, x="forest_size", y=["O0", "Os", "O2", "O3"], ylabel="Größe in bytes", xlabel="Waldgröße")
    fig.axes.set_xlim([1, 16])
    #fig.axes.set_ylim([0, 32000])
    fig.axes.set_ylim([0, fig.axes.get_ylim()[1]])
    plt.plot(range(65), np.repeat(32000, 65), '--')
    plt.plot([10, 10], [0, fig.axes.get_ylim()[1]], '--')

    ax2 = fig.axes.twinx()
    data.plot(ax=ax2, x="forest_size", y="Accuracy", colormap="Pastel1", ylabel="Genauigkeit")
    ax2.set_ylim([0, 1])

    ax1.legend(loc="upper left", bbox_to_anchor=(0, 0.9))
    ax2.legend(loc="upper left", bbox_to_anchor=(0, 1))

    # figur.tight_layout(pad=-1)
    figur.set_size_inches(15, 11)

    plt.savefig('./plotting/program_size_plots/forest_bytes_md_' + str(max_depth) + '.png', format='png', dpi=100, pad_inches=0.2, bbox_inches="tight")
    plt.close('all')
