import multiprocessing
import sys

import pandas as pd
from create_forest import *
from create_tree import *
from sklearn import tree
from sklearn.experimental import enable_hist_gradient_boosting  # noqa
from sklearn.ensemble import RandomForestClassifier, AdaBoostClassifier, BaggingClassifier, GradientBoostingClassifier, ExtraTreesClassifier, HistGradientBoostingClassifier
from sklearn.model_selection import train_test_split, cross_val_score
from xgboost.sklearn import XGBClassifier

_, max_depth, num_trees, with_io, ensamble_kind, only_ensamble, feature_set = sys.argv
max_depth = int(max_depth)
num_trees = int(num_trees)
with_io = 1 == int(with_io)
ensamble_kind = int(ensamble_kind)
only_ensamble = 1 == int(only_ensamble)
feature_set = int(feature_set)


# This is a helper function to quickly print some results of the tree's performance.
def evaluate_predicted(predicted, y_test):
    true_positive = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    false_positive = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    for i in range(len(y_test)):
        if predicted[i] == y_test[i]:
            true_positive[predicted[i]] += 1
        else:
            false_positive[predicted[i]] += 1

    total_gestures = len(y_test)
    amount_correct = 0
    for gesture_type in [1, 2, 3, 4, 9]:
        amount_of_gesture = y_test.tolist().count(gesture_type)
        print("GestureType: " + str(gesture_type))
        if amount_of_gesture > 0:
            print("True Positive: %.3f" % (100 * (true_positive[gesture_type] / amount_of_gesture)))
            print("False Positive: %.3f" % (100 * (false_positive[gesture_type] / total_gestures)))
            amount_correct += true_positive[gesture_type]
    print("Total accuracy: %.3f" % (100 * (amount_correct / total_gestures)))


# Import all the data
# Generated from the extractor
storage_path = "./model_data"
result = pd.read_csv(storage_path + "/result", dtype=int).values.flatten()
# lsos_x = pd.read_csv(storage_path + "/LocalSumOfSlopesX", dtype=int)
# lsos_y = pd.read_csv(storage_path + "/LocalSumOfSlopesY", dtype=int)
# darkness_dist_3x = pd.read_csv(storage_path + "/DarknessDistribution3X", dtype=float)
# darkness_dist_6x = pd.read_csv(storage_path + "/DarknessDistribution6X", dtype=float)
# darkness_dist_3y = pd.read_csv(storage_path + "/DarknessDistribution3Y", dtype=float)
# darkness_dist_6y = pd.read_csv(storage_path + "/DarknessDistribution6Y", dtype=float)
# darkness_dist_6xy = pd.read_csv(storage_path + "/DarknessDistribution6XY", dtype=int)
# darkness_dist_6xy_quadrant = pd.read_csv(storage_path + "/DarknessDistribution6XYQuadrant", dtype=int)
# brightness_dist_3x = pd.read_csv(storage_path + "/BrightnessDistribution3X", dtype=float)
# brightness_dist_6x = pd.read_csv(storage_path + "/BrightnessDistribution6X", dtype=float)
# brightness_dist_3y = pd.read_csv(storage_path + "/BrightnessDistribution3Y", dtype=float)
# brightness_dist_6y = pd.read_csv(storage_path + "/BrightnessDistribution6Y", dtype=float)
# brightness_dist_6xy = pd.read_csv(storage_path + "/BrightnessDistribution6XY", dtype=int)
# brightness_dist_6xy_quadrant = pd.read_csv(storage_path + "/BrightnessDistribution6XYQuadrant", dtype=int)
# mean_value = pd.read_csv(storage_path + "/MeanValue", dtype=int)
# minimum_value = pd.read_csv(storage_path + "/MinimumValue", dtype=int)
# maximum_value = pd.read_csv(storage_path + "/MaximumValue", dtype=int)
# standard_deviation = pd.read_csv(storage_path + "/StandardDeviation", dtype=float)
# average_amplitude_change = pd.read_csv(storage_path + "/AverageAmplitudeChange", dtype=int)
# direction_map_x = pd.read_csv(storage_path + "/DirectionMapX", dtype=int)
# direction_map_y = pd.read_csv(storage_path + "/DirectionMapY", dtype=int)
# sum_of_slopes = pd.read_csv(storage_path + "/SumOfSlopes", dtype=int)

# Specifying the features
if feature_set == 1:
    center_of_gravity_distribution_float_x = pd.read_csv(storage_path + "/CenterOfGravityDistributionFloatX", dtype=float)
    center_of_gravity_distribution_float_y = pd.read_csv(storage_path + "/CenterOfGravityDistributionFloatY", dtype=float)
    X = pd.concat([center_of_gravity_distribution_float_x, center_of_gravity_distribution_float_y], axis=1).values
elif feature_set == 2:
    center_of_gravity_distribution_x = pd.read_csv(storage_path + "/CenterOfGravityDistributionX", dtype=int)
    center_of_gravity_distribution_y = pd.read_csv(storage_path + "/CenterOfGravityDistributionY", dtype=int)
    X = pd.concat([center_of_gravity_distribution_x, center_of_gravity_distribution_y], axis=1).values
else:
    motion_history = pd.read_csv(storage_path + "/MotionHistory", dtype=int)
    brightness_dist_6xy_geom = pd.read_csv(storage_path + "/BrightnessDistribution6XYGeom", dtype=int)
    darkness_dist_6xy_geom = pd.read_csv(storage_path + "/DarknessDistribution6XYGeom", dtype=int)
    X = pd.concat([darkness_dist_6xy_geom, brightness_dist_6xy_geom, motion_history], axis=1).values


# Interestingly seems the order to effect the accuracy
#new_x = []
#for item in X:
#    new_x.append([item[0], item[5], item[1], item[6], item[2], item[7], item[3], item[8], item[4], item[9]])
#X = new_x

y = result
X_train, X_test_and_opt, y_train, y_test_and_opt = train_test_split(X, y, test_size=0.3, random_state=0)

# For cherry picking we will optimize on XX_opt and later validate on XX_test
XX_opt, XX_test, yy_opt, yy_test = train_test_split(X_test_and_opt, y_test_and_opt, test_size=0.5, random_state=0)


# This function is used to fit the decision tree classifier to the training set
def evaluate_tree(id):
    clf = tree.DecisionTreeClassifier(criterion="entropy", max_depth=max_depth, random_state=id)
    clf = clf.fit(X_train, y_train)

    predicted = clf.predict(XX_opt)

    correct = 0
    for i in range(len(yy_opt)):
        if predicted[i] == yy_opt[i]:
            correct += 1

    accuracy = correct / len(yy_opt)

    return clf, accuracy


# Create the decision tree and train it
def decision_tree():
    # Fit a bunch of trees in parallel
    amount_tests = 1
    print("Test " + str(amount_tests) + " different trees, and cherry pick best...")
    pool = multiprocessing.Pool(processes=multiprocessing.cpu_count() - 1)
    trees = pool.map(evaluate_tree, range(amount_tests))
    trees.sort(key=lambda x: x[1], reverse=True)

    # Take the best
    clf = trees[0][0]

    # plt.figure(figsize=(40, 40))
    # tree.plot_tree(clf, impurity=False, filled=True)
    # plt.savefig('tree.png', format='png')

    print("Evaluating DecisionTreeClassifier:")
    print("Max depth: " + str(clf.tree_.max_depth))
    predicted = clf.predict(XX_test)
    evaluate_predicted(predicted, yy_test)

    file = open("decision_tree.c", "w")
    create_tree_native_main(file, clf, with_io, feature_set == 1)
    file.close()

    file = open("ino_tree/decision_tree.cpp", "w")
    create_tree_ino_evaluate(file, clf, feature_set == 1)
    file.close()

    file = open("ino_tree2/decision_tree.cpp", "w")
    create_tree_ino_evaluate(file, clf, feature_set == 1)
    file.close()

    file = open("ino_tree3/decision_tree.cpp", "w")
    create_tree_ino_evaluate(file, clf, feature_set == 1)
    file.close()


def evaluate_forest(id):
    # ccp_alpha=0.001, min_samples_leaf=2 ?
    clf = RandomForestClassifier(max_depth=max_depth, criterion='entropy', n_estimators=num_trees, random_state=id, n_jobs=1)
    clf = clf.fit(X_train, y_train)

    predicted = clf.predict(XX_opt)

    correct = 0
    for i in range(len(yy_opt)):
        if predicted[i] == yy_opt[i]:
            correct += 1

    accuracy = correct / len(yy_opt)

    return clf, accuracy


def create_ensamble_tree(clf):
    file = open("decision_forest.c", "w")
    create_forest_native_main(file, clf.estimators_, clf.classes_, num_trees, with_io, feature_set == 1)
    file.close()

    file = open("ino_tree/decision_forest.cpp", "w")
    create_forest_ino_evaluate(file, clf.estimators_, clf.classes_, num_trees, feature_set == 1)
    file.close()

    file = open("ino_tree2/decision_forest.cpp", "w")
    create_forest_ino_evaluate(file, clf.estimators_, clf.classes_, num_trees, feature_set == 1)
    file.close()

    file = open("ino_tree3/decision_forest.cpp", "w")
    create_forest_ino_evaluate(file, clf.estimators_, clf.classes_, num_trees, feature_set == 1)
    file.close()


def random_forest():
    amount_tests = 256
    print("Test " + str(amount_tests) + " different forest, and cherry pick best...")
    pool = multiprocessing.Pool(processes=multiprocessing.cpu_count() - 1)
    trees = pool.map(evaluate_forest, range(amount_tests))
    trees.sort(key=lambda x: x[1], reverse=True)

    clf = trees[0][0]

    predicted = clf.predict(XX_test)

    print("Evaluating RandomForestClassifier:")
    evaluate_predicted(predicted, yy_test)

    create_ensamble_tree(clf)

# This is like GBM, but just more performant and applies regularization to avoid overfitting
def xgboost_decision_tree():
    xgb1 = XGBClassifier(
        learning_rate=0.01,
        n_estimators=num_trees,
        max_depth=max_depth,
        min_child_weight=1,
        gamma=0.5,
        subsample=0.9,
        colsample_bytree=0.9,
        reg_alpha=0.005,
        objective='binary:logistic',
        nthread=16,
        seed=42)
    xgb1.fit(X_train, y_train, eval_metric="auc")
    scores = cross_val_score(xgb1, XX_test, yy_test, cv=5, n_jobs=1)
    print("XGBClassifier")
    print("Mean cross-validation score: %.2f" % scores.mean())


def adaboost_decision_tree():
    clf = AdaBoostClassifier(base_estimator=tree.DecisionTreeClassifier(max_depth=max_depth, criterion="entropy"),
                               n_estimators=num_trees, random_state=1, learning_rate=0.01)
    clf.fit(X_train, y_train)
    print("AdaBoostClassifier: ")
    print(clf.score(XX_test, yy_test))

    create_ensamble_tree(clf)


# Difference to RandomForestClassifier is, that this does not select a set of features
def bagging_decision_tree():
    clf = BaggingClassifier(base_estimator=tree.DecisionTreeClassifier(max_depth=max_depth, criterion="entropy"),
                              n_estimators=num_trees, random_state=1)
    clf.fit(X_train, y_train)
    print("BaggingClassifier: ")
    print(clf.score(XX_test, yy_test))

    create_ensamble_tree(clf)


# Uses DecisionTreeRegressor under the hood o.o
def gradient_boosting_decision_tree():
    clf = GradientBoostingClassifier(n_estimators=num_trees, random_state=1, learning_rate=0.01, max_depth=max_depth)
    clf.fit(X_train, y_train)
    print("GradientBoostingClassifier: ")
    print(clf.score(XX_test, yy_test))


# Uses extra trees, they seem to differ from normal decision trees
# Creates random splits for each feature and tries to select the best split (if this method is enabled)
# Scales well with a big max-depth
# It can also use the normal decision tree as estimator. Not sure where the difference to the random forest is then
def extra_trees():
    clf = ExtraTreesClassifier(n_estimators=num_trees, random_state=1, n_jobs=16, max_depth=max_depth, max_features=10)
    # clf.base_estimator = tree.DecisionTreeClassifier(max_depth=max_depth, criterion="entropy")
    clf.fit(X_train, y_train)
    print("ExtraTreesClassifier: ")
    print(clf.score(XX_test, yy_test))

    create_ensamble_tree(clf)


# Not so sure what this is, but it works well and should be based on decision trees
# Documentation: https://scikit-learn.org/stable/modules/generated/sklearn.ensemble.HistGradientBoostingClassifier.html#sklearn.ensemble.HistGradientBoostingClassifier
def hist_gradient_boosting_decision_tree():
    clf = HistGradientBoostingClassifier(random_state=1, learning_rate=0.01, max_depth=max_depth, max_iter=num_trees)
    clf.fit(X_train, y_train)
    print("HistGradientBoostingClassifier: ")
    print(clf.score(XX_test, yy_test))


if not only_ensamble:
    decision_tree()

if ensamble_kind == 1:
    random_forest()
elif ensamble_kind == 2:
    adaboost_decision_tree()
elif ensamble_kind == 3:
    bagging_decision_tree()
elif ensamble_kind == 4:
    extra_trees()
else:
    gradient_boosting_decision_tree()
    xgboost_decision_tree()
    hist_gradient_boosting_decision_tree()
