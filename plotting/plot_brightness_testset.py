import pandas as pd
from matplotlib import pyplot as plt

data = pd.read_csv("./light_eval.csv").query("scaling == 0")

df = pd.DataFrame(columns=["ansatz", "offset", "scaling", "accuracy"])
for item in data.iterrows():
    scaling = 100.0 if item[1].scaling < 0.001 else item[1].scaling
    row = df.query(
        "ansatz == " + str(item[1].ansatz) + " and offset == " + str(item[1].offset) + " and scaling == " + str(scaling))

    if row.index.size == 0:
        df = df.append({"ansatz": item[1].ansatz, "offset": item[1].offset,
                        "scaling": scaling, "accuracy": item[1].accuracy},
                       ignore_index=True)
    else:
        df.iloc[row.index[0]].accuracy = df.iloc[row.index[0]].accuracy + item[1].accuracy

amount_same = data.query("ansatz == 1 and offset == 0 and scaling == 0").index.size
df["accuracy"] = df["accuracy"] / amount_same
df["scaling"] = df["scaling"] / 100

df2 = pd.DataFrame()
for item in df.groupby(["offset", "scaling"]):
    df2 = df2.append({"offset": item[1].offset.iloc[0], "scaling": item[1].scaling.iloc[0],
                      "Helligkeitsverteilung": item[1].accuracy.iloc[0],
                      "Motion History": item[1].accuracy.iloc[1],
                      "Schwerpunktverteilung mit Gleitkommazahlen": item[1].accuracy.iloc[2],
                      "Schwerpunktverteilung mit Ganzzahlen": item[1].accuracy.iloc[3],
                      "Kombinierte Schwerpunktverteilung": item[1].accuracy.iloc[4]},
                     ignore_index=True)

plt.rcParams.update({'font.size': 30})

plt.figure(figsize=(60, 45))
figur, ax1 = plt.subplots()
#fig = df2.plot(ax=ax1, x="scaling", y=["Helligkeitsverteilung", "Motion History",
#                                       "Schwerpunktverteilung mit Gleitkommazahlen",
#                                       "Schwerpunktverteilung mit Ganzzahlen", "Kombinierte Schwerpunktverteilung"])

fig = df2.plot(ax=ax1, x="offset", y=["Schwerpunktverteilung mit Gleitkommazahlen",
                                       "Schwerpunktverteilung mit Ganzzahlen", "Kombinierte Schwerpunktverteilung"])
ax1.set_xlabel("Offset")
ax1.set_ylabel("Klassifizierungsgenauigkeit")
ax1.set_ylim([0, ax1.get_ylim()[1]])
ax1.set_xlim([0, 800])

ax1.legend(loc="lower left", bbox_to_anchor=(0, 0))

figur.set_size_inches(20, 15)
plt.savefig('./plotting/brightness_offset.png', format='png', dpi=100, pad_inches=0.2,
            bbox_inches="tight")
plt.close('all')
