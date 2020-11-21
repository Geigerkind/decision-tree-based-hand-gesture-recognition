import pandas as pd
from matplotlib import pyplot as plt

data_size = pd.read_csv("./size_data_test.csv")

df = pd.DataFrame()

for item in data_size.groupby(["max_depth", "forest_size"]):
    df = df.append({"max_depth": item[1].max_depth.iloc[0], "forest_size": item[1].forest_size.iloc[0],
                    "O0": item[1].forest_bytes.iloc[0], "Os": item[1].forest_bytes.iloc[1],
                    "O2": item[1].forest_bytes.iloc[2], "O3": item[1].forest_bytes.iloc[3]}, ignore_index=True)


for max_depth in range(51)[1:]:
    plt.figure(figsize=(10, 10))
    df.query("max_depth==" + str(max_depth)).plot(x="forest_size", y=["O0", "Os", "O2", "O3"])
    plt.savefig('./plotting/program_size_plots/forest_bytes_md_' + str(max_depth) + '.png', format='png')
    plt.close('all')

