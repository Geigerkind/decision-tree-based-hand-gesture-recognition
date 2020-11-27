import numpy as np
import pandas as pd
from matplotlib import pyplot as plt

# Manually tested
default_size = 2508

data_size = pd.read_csv("./size_and_accuracy_data_klisch_int.csv")

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

plt.figure(figsize=(10, 10))
figur, ax1 = plt.subplots()
plt.title("Decision tree")
fig = tree_data.plot(ax=ax1, x="max_depth", y=["O0", "Os", "O2", "O3"])
ax1.set_xlabel("Max depth")
ax1.set_ylabel("Size in bytes")
ax1.set_xlim([1, 30])

ax2 = ax1.twinx()
data.plot(ax=ax2, x="max_depth", y="Accuracy", colormap="Pastel1")
ax2.set_ylabel("Accuracy")
ax2.set_ylim([0, 1])

ax1.legend(loc="upper left", bbox_to_anchor=(0, 0.93))
ax2.legend(loc="upper left", bbox_to_anchor=(0, 1))

figur.tight_layout()
plt.savefig('./plotting/program_size_plots/forest_bytes_md_0.png', format='png')
plt.close('all')


# Draw x=Forest size y= Size in bytes vs. Accuracy graphs with growing
for max_depth in range(31)[1:]:
    plt.figure(figsize=(10, 10))
    figur, ax1 = plt.subplots()
    plt.title("Decision forest - Max depth: " + str(max_depth))

    data = df.query("forest_size <= 32 and max_depth==" + str(max_depth))
    fig = data.plot(ax=ax1, x="forest_size", y=["O0", "Os", "O2", "O3"], ylabel="Size in bytes", xlabel="Forest size")
    plt.plot(range(65), np.repeat(32000, 65), '--')
    fig.axes.set_xlim([1, 16])
    # fig.axes.set_ylim([0, 32000])
    fig.axes.set_ylim([0, fig.axes.get_ylim()[1]])

    ax2 = fig.axes.twinx()
    data.plot(ax=ax2, x="forest_size", y="Accuracy", colormap="Pastel1", ylabel="Accuracy")
    ax2.set_ylim([0, 1])

    ax1.legend(loc="upper left", bbox_to_anchor=(0, 0.93))
    ax2.legend(loc="upper left", bbox_to_anchor=(0, 1))

    figur.tight_layout()

    plt.savefig('./plotting/program_size_plots/forest_bytes_md_' + str(max_depth) + '.png', format='png')
    plt.close('all')
