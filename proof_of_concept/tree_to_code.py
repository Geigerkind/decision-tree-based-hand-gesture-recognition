# Shamelessly "borrowed" from:
# https://github.com/arturyumaev/Rules-Extraction-from-sklearn-DecisionTreeClassifier
# And slightly modified to fit the purpose of classification for gestures
from sklearn.tree import _tree


def tree_to_code(tree, feature_names=range(0, 40)):
    file = open("decision_tree.c", "w")

    nspaces = 4
    tree_ = tree.tree_
    feature_name = [
        feature_names[i] if i != _tree.TREE_UNDEFINED else "undefined!"
        for i in tree_.feature
    ]

    file.write("unsigned char GestureClassificationDecisionTree(float* features)")
    file.write("{")

    def recurse(node, depth):
        indent = " " * nspaces * depth

        if tree_.feature[node] != _tree.TREE_UNDEFINED:
            name = feature_name[node]
            threshold = tree_.threshold[node]

            file.write("\n{}if (features[{}] <= {:0.9f})".format(indent, name, threshold) + " {")

            recurse(tree_.children_left[node], depth + 1)

            file.write("{}".format(indent) + "} else {")

            recurse(tree_.children_right[node], depth + 1)

            file.write("\n" + "{}".format(indent) + "}\n")
        else:
            classification_index = [ind for ind, x in enumerate(tree_.value[node][0]) if x != 0][0]
            file.write("\n{}return {};\n".format(indent, tree.classes_[classification_index]))

    recurse(0, 1)

    file.write("}\n")
    file.write("#include <stdio.h>\n")
    file.write("int main(int argc, char** argv) {\n")
    # file.write("short int args[27];\n")
    # file.write("for (char i = 0; i < 27; ++i) sscanf(argv[i+1], \"%hd\", &args[i]);\n")
    file.write("float args[12];\n")
    file.write("for (char i = 0; i < 12; ++i) sscanf(argv[i+1], \"%f\", &args[i]);\n")
    file.write("return GestureClassificationDecisionTree(args);\n")
    # file.write("printf(\"Result: %d\\n\", (int) result);\n")
    file.write("}\n")
    file.close()
