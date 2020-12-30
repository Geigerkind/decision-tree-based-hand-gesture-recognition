import pandas as pd
from matplotlib import pyplot as plt

# Manually tested
default_size = 2500

data = pd.read_csv("./ccp_alpha_small.csv")
data["Größe relativ zu ccp_alpha = 0.0"] = data["Größe relativ zu ccp_alpha = 0.0"] - default_size
data["Größe relativ zu ccp_alpha = 0.0"] = 100.0 * (data["Größe relativ zu ccp_alpha = 0.0"] / data["Größe relativ zu ccp_alpha = 0.0"].iloc[0])
print(data)

plt.rcParams.update({'font.size': 30})

plt.figure(figsize=(30, 30))
figur, ax1 = plt.subplots()
fig = data.plot(ax=ax1, x="ccp_alpha", y=["Größe relativ zu ccp_alpha = 0.0"])
ax1.set_xlabel("ccp_alpha")
ax1.set_ylabel("Relative Programmgröße in %")
ax1.set_ylim([0, ax1.get_ylim()[1]])
ax1.set_xlim([0.00050828125, 0.5])
plt.xscale('log', base=2, nonpositive='clip')

ax2 = ax1.twinx()
data.plot(ax=ax2, x="ccp_alpha", y="Baumhöhe", colormap="Pastel1")
ax2.set_ylabel("Baumhöhe")
ax2.set_ylim([0, 16])

ax1.legend(loc="upper left", bbox_to_anchor=(0, 0.9))
ax2.legend(loc="upper left", bbox_to_anchor=(0, 1))

figur.set_size_inches(15, 11)
plt.savefig('./plotting/size_plots/ccp_small_set.png', format='png', dpi=100, pad_inches=0.2,
            bbox_inches="tight")
plt.close('all')