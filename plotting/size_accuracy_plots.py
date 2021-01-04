import pandas as pd
from matplotlib import pyplot as plt

d1 = pd.read_csv("./saad_c8.csv").query(
    "optimization_level == 'O0' and ensemble_technique != 3 and feature_set != 3 and feature_set != 5 and feature_set != 6 and feature_set != 7")
d2 = pd.read_csv("./saad_c9.csv").query("optimization_level == 'O0' and ensemble_technique != 3")
d3 = pd.read_csv("./saad_c10.csv").query("optimization_level == 'O0'")

data_size = pd.concat([d1, d2, d3], ignore_index=True)

for feature_set in range(9):
    current_data = data_size.query("feature_set == " + str(feature_set + 1))
    df = pd.DataFrame()
    for item in current_data.iterrows():
        df = df.append({"max_depth": item[1].max_depth, "forest_size": item[1].forest_size,
                        "ccp_alpha": item[1].ccp_alpha, "min_leaf_sample": item[1].min_leaf_sample,
                        "ensemble_technique": item[1].ensemble_technique,
                        "Klisch": item[1].accuracy_klisch,
                        "Dymel Gesten": item[1].accuracy_dymel_gesture,
                        "Dymel Null": item[1].accuracy_dymel_null,
                        "Durchschnitt": (item[1].accuracy_dymel_null + item[1].accuracy_klisch + item[1].accuracy_dymel_gesture) / 3},
                       ignore_index=True)

    if df.size == 0:
        continue

    data = df.groupby("forest_size").apply(lambda idf: idf.iloc[idf.Durchschnitt.argmax()])

    plt.figure(figsize=(15, 15))
    figur, ax1 = plt.subplots()

    fig = data.plot(ax=ax1, x="forest_size", y=["Klisch", "Dymel Gesten", "Dymel Null", "Durchschnitt"],
                    ylabel="Erkennungsgenauigkeit",
                    xlabel="Waldgröße")
    fig.axes.set_xlim([1, 16])
    fig.axes.set_ylim([0.0, 1])

    # max_acc = data.iloc[data["Durchschnitt"].argmax()]
    # plt.plot([max_acc.forest_size, max_acc.forest_size], [0, ax1.get_ylim()[1]], '--')

    ax1.legend(loc="lower left", bbox_to_anchor=(0, 0))

    figur.set_size_inches(10, 6)

    plt.savefig('./plotting/size_accuracy_plots/feature_set_' + str(feature_set + 1) + '/per_forest_size.png',
                format='png', dpi=100, pad_inches=0.2, bbox_inches="tight")
    plt.close('all')
