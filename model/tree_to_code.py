# Shamelessly "borrowed" from:
# https://github.com/arturyumaev/Rules-Extraction-from-sklearn-DecisionTreeClassifier
# And slightly modified to fit the purpose of classification for gestures
from sklearn.tree import _tree
import numpy as np


def tree_to_code(file, tree, classes, function_name, feature_set, return_voting, feature_names=range(0, 40)):
    nspaces = 4
    tree_ = tree.tree_
    feature_name = [
        feature_names[i] if i != _tree.TREE_UNDEFINED else "undefined!"
        for i in tree_.feature
    ]

    if return_voting:
        if feature_set == 1 or feature_set == 7 or feature_set == 8 or feature_set == 9:
            file.write("void " + function_name + "(float* features, float* result)")
        elif feature_set == 6:
            file.write("void " + function_name + "(float* float_features, long* long_features, float* result)")
        else:
            file.write("void " + function_name + "(long* features, float* result)")
    else:
        if feature_set == 1 or feature_set == 7 or feature_set == 8 or feature_set == 9:
            file.write("unsigned char " + function_name + "(float* features)")
        elif feature_set == 6:
            file.write("unsigned char " + function_name + "(float* float_features, long* long_features)")
        else:
            file.write("unsigned char " + function_name + "(long* features)")
    file.write("{")

    def recurse(node, depth):
        indent = " " * nspaces * depth

        if tree_.feature[node] != _tree.TREE_UNDEFINED:
            name = feature_name[node]
            threshold = tree_.threshold[node]

            if feature_set == 1 or feature_set == 7 or feature_set == 8 or feature_set == 9:
                file.write("\n{}if (features[{}] <= {:0.9f})".format(indent, name, threshold) + " {")
            elif feature_set == 6:
                if int(name) < 10:
                    file.write("\n{}if (float_features[{}] <= {:0.9f})".format(indent, name, threshold) + " {")
                else:
                    file.write("\n{}if (long_features[{}] <= {:0.0f})".format(indent, name, threshold) + " {")
            else:
                file.write("\n{}if (features[{}] <= {:0.0f})".format(indent, name, threshold) + " {")

            recurse(tree_.children_left[node], depth + 1)

            file.write("{}".format(indent) + "} else {")

            recurse(tree_.children_right[node], depth + 1)

            file.write("\n" + "{}".format(indent) + "}\n")
        else:
            if return_voting:
                # Return probability of each class
                sample_sum = sum(tree_.value[node][0])
                for index, sample in enumerate(tree_.value[node][0]):
                    file.write("\n{}result[{}] = {};\n".format(indent, index, sample/sample_sum))
                file.write("\n{}return;\n".format(indent))
            else:
                # Return Class of that has most samples
                classification_index = np.where(tree_.value[node][0] == max(tree_.value[node][0]))[0][0]
                file.write("\n{}return {};\n".format(indent, int(classes[classification_index])))

    recurse(0, 1)
    file.write("}\n")
