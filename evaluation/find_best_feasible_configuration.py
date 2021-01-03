import pandas as pd

# Manually tested
default_size = 2500

max_size_in_bytes = 150000 + default_size
# data = pd.read_csv("./saad_c8.csv").query("forest_bytes <= " + str(max_size_in_bytes) + " and forest_bytes > 0")
data = pd.read_csv("./saad_c8.csv")

res_max_height = []
res_forest_size = []
res_ccp_alpha = []
res_min_samples_leaf = []
res_min_size = []
res_acc_klisch = []
res_acc_dymel_gesture = []
res_acc_dymel_null = []
res_best_acc_klisch = []
for i in range(4):
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
    combined_accuracy = 0
    best_acc_klisch = 0
    for item in data.query("feature_set == 2 and ensemble_technique == " + str(i+1)).iterrows():
        temp_comb = item[1].accuracy_klisch + item[1].accuracy_dymel_null + item[1].accuracy_dymel_gesture
        if (item[1].forest_bytes < min_size and temp_comb == combined_accuracy) or temp_comb > combined_accuracy:
            # Max ist hier nicht mehr passend!
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
            combined_accuracy = temp_comb

        if best_acc_klisch < item[1].accuracy_klisch:
            best_acc_klisch = item[1].accuracy_klisch

    res_max_height.append(max_depth)
    res_forest_size.append(forest_size)
    res_ccp_alpha.append(ccp_alpha)
    res_min_samples_leaf.append(min_leaf_sample)
    res_min_size.append(min_size)
    res_acc_klisch.append(max_accuracy_klisch)
    res_acc_dymel_gesture.append(max_accuracy_dymel_gesture)
    res_acc_dymel_null.append(max_accuracy_dymel_null)
    res_best_acc_klisch.append(best_acc_klisch)

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
    print("Best Klisch: " + str(best_acc_klisch))
    print("-------------------------------------------------------------")

print("\\begin{table}[h!]")
print("\\centering")
print("\\begin{tabular}{ | c | c | c | c | c |}")
print("    \\hline")
print("     & RandomForest & Boosting & Bagging & ExtraTrees \\\\\\hline")
print("    Maximalhöhe & " + str(res_max_height[0]) + " & " + str(res_max_height[1]) + " & " + str(res_max_height[2]) + " & " + str(res_max_height[3]) + " \\\\\\hline")
print("    Waldgröße & " + str(res_forest_size[0]) + " & " + str(res_forest_size[1]) + " & " + str(res_forest_size[2]) + " & " + str(res_forest_size[3]) + " \\\\\\hline")
print("    ccp\\_alpha & " + str(res_ccp_alpha[0]) + " & " + str(res_ccp_alpha[1]) + " & " + str(res_ccp_alpha[2]) + " & " + str(res_ccp_alpha[3]) + " \\\\\\hline")
print("    min\\_samples\\_leaf & " + str(res_min_samples_leaf[0]) + " & " + str(res_min_samples_leaf[1]) + " & " + str(res_min_samples_leaf[2]) + " & " + str(res_min_samples_leaf[3]) + " \\\\\\hline")
# print("    Beste Genauigkeit Klisch & {:.1f}\\% & {:.1f}\\% & {:.1f}\\% & {:.1f}\\% \\\\\\hline".format(100 * res_best_acc_klisch[0], 100 * res_best_acc_klisch[1], 100 * res_best_acc_klisch[2], 100 * res_best_acc_klisch[3]))
print("    \\textbf{Ohne Optimierung} &  &  &  &  \\\\\\hline")
print("    Größe in Bytes & " + str(res_min_size[0] - default_size if res_min_size[0] >= 0 else '-') + " & " + str(res_min_size[1] - default_size if res_min_size[1] >= 0 else '-') + " & " + str(res_min_size[2] - default_size if res_min_size[2] >= 0 else '-') + " & " + str(res_min_size[3] - default_size if res_min_size[3] >= 0 else '-') + " \\\\\\hline")
print("    Genauigkeit Klisch & {:.1f}\\% & {:.1f}\\% & {:.1f}\\% & {:.1f}\\% \\\\\\hline".format(100 * res_acc_klisch[0], 100 * res_acc_klisch[1], 100 * res_acc_klisch[2], 100 * res_acc_klisch[3]))
print("    Genauigkeit Dymel Gesture & {:.1f}\\% & {:.1f}\\% & {:.1f}\\% & {:.1f}\\% \\\\\\hline".format(100 * res_acc_dymel_gesture[0], 100 * res_acc_dymel_gesture[1], 100 * res_acc_dymel_gesture[2], 100 * res_acc_dymel_gesture[3]))
print("    Genauigkeit Dymel Null & {:.1f}\\% & {:.1f}\\% & {:.1f}\\% & {:.1f}\\% \\\\\\hline".format(100 * res_acc_dymel_null[0], 100 * res_acc_dymel_null[1], 100 * res_acc_dymel_null[2], 100 * res_acc_dymel_null[3]))
print("    \\textbf{Mit Optimierung} &  &  &  &  \\\\\\hline")
print("    Größe in Bytes & - & - & - & - \\\\\\hline")
print("    Genauigkeit Klisch & - & - & - & - \\\\\\hline")
print("    Genauigkeit Dymel Gesten & - & - & - & - \\\\\\hline")
print("    Genauigkeit Dymel Null & - & - & - & - \\\\\\hline")
print("\\end{tabular}")
print("\\caption{Beste Konfigurationen je Ensemble-Methode der TODO.}")
print("\\label{tab:TODO}")
print("\\end{table}")