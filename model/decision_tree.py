import multiprocessing

import pandas as pd
from create_forest import *
from create_tree import *
from matplotlib import pyplot as plt
from sklearn import tree
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split


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
lsos_x = pd.read_csv(storage_path + "/LocalSumOfSlopesX", dtype=int)
lsos_y = pd.read_csv(storage_path + "/LocalSumOfSlopesY", dtype=int)
darkness_dist_3x = pd.read_csv(storage_path + "/DarknessDistribution3X", dtype=float)
darkness_dist_6x = pd.read_csv(storage_path + "/DarknessDistribution6X", dtype=float)
darkness_dist_3y = pd.read_csv(storage_path + "/DarknessDistribution3Y", dtype=float)
darkness_dist_6y = pd.read_csv(storage_path + "/DarknessDistribution6Y", dtype=float)
darkness_dist_6xy = pd.read_csv(storage_path + "/DarknessDistribution6XY", dtype=int)
darkness_dist_6xy_geom = pd.read_csv(storage_path + "/DarknessDistribution6XYGeom", dtype=int)
darkness_dist_6xy_quadrant = pd.read_csv(storage_path + "/DarknessDistribution6XYQuadrant", dtype=int)
brightness_dist_3x = pd.read_csv(storage_path + "/BrightnessDistribution3X", dtype=float)
brightness_dist_6x = pd.read_csv(storage_path + "/BrightnessDistribution6X", dtype=float)
brightness_dist_3y = pd.read_csv(storage_path + "/BrightnessDistribution3Y", dtype=float)
brightness_dist_6y = pd.read_csv(storage_path + "/BrightnessDistribution6Y", dtype=float)
brightness_dist_6xy = pd.read_csv(storage_path + "/BrightnessDistribution6XY", dtype=int)
brightness_dist_6xy_geom = pd.read_csv(storage_path + "/BrightnessDistribution6XYGeom", dtype=int)
brightness_dist_6xy_quadrant = pd.read_csv(storage_path + "/BrightnessDistribution6XYQuadrant", dtype=int)
motion_history = pd.read_csv(storage_path + "/MotionHistory", dtype=int)
mean_value = pd.read_csv(storage_path + "/MeanValue", dtype=int)
minimum_value = pd.read_csv(storage_path + "/MinimumValue", dtype=int)
maximum_value = pd.read_csv(storage_path + "/MaximumValue", dtype=int)
standard_deviation = pd.read_csv(storage_path + "/StandardDeviation", dtype=float)
average_amplitude_change = pd.read_csv(storage_path + "/AverageAmplitudeChange", dtype=int)
direction_map_x = pd.read_csv(storage_path + "/DirectionMapX", dtype=int)
direction_map_y = pd.read_csv(storage_path + "/DirectionMapY", dtype=int)
sum_of_slopes = pd.read_csv(storage_path + "/SumOfSlopes", dtype=int)
center_of_gravity_distribution_x = pd.read_csv(storage_path + "/CenterOfGravityDistributionX", dtype=int)
center_of_gravity_distribution_y = pd.read_csv(storage_path + "/CenterOfGravityDistributionY", dtype=int)
center_of_gravity_distribution_float_x = pd.read_csv(storage_path + "/CenterOfGravityDistributionFloatX", dtype=float)
center_of_gravity_distribution_float_y = pd.read_csv(storage_path + "/CenterOfGravityDistributionFloatY", dtype=float)

# Specifying the features
# X = pd.concat([darkness_dist_6xy_geom, brightness_dist_6xy_geom, motion_history, center_of_gravity_distribution_float_x, center_of_gravity_distribution_float_y], axis=1).values
# X = pd.concat([center_of_gravity_distribution_float_x, center_of_gravity_distribution_float_y], axis=1).values
X = pd.concat([center_of_gravity_distribution_x, center_of_gravity_distribution_y], axis=1).values
y = result
X_train, X_test_and_opt, y_train, y_test_and_opt = train_test_split(X, y, test_size=0.3, random_state=0)

# For cherry picking we will optimize on XX_opt and later validate on XX_test
XX_opt, XX_test, yy_opt, yy_test = train_test_split(X_test_and_opt, y_test_and_opt, test_size=0.5, random_state=0)


# This function is used to fit the decision tree classifier to the training set
def evaluate_tree(id):
    clf = tree.DecisionTreeClassifier()
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
    amount_tests = 1024
    print("Test " + str(amount_tests) + " different trees, and cherry pick best...")
    pool = multiprocessing.Pool(processes=multiprocessing.cpu_count() - 1)
    trees = pool.map(evaluate_tree, range(amount_tests))
    trees.sort(key=lambda x: x[1], reverse=True)

    # Take the best
    clf = trees[0][0]

    plt.figure(figsize=(40, 40))
    tree.plot_tree(clf)
    plt.savefig('tree.png', format='png')

    print("Evaluating DecisionTreeClassifier:")
    print("Max depth: " + str(clf.tree_.max_depth))
    predicted = clf.predict(XX_test)
    evaluate_predicted(predicted, yy_test)

    file = open("decision_tree.c", "w")
    create_tree_native_main(file, clf)
    file.close()

    file = open("ino_tree/decision_tree.cpp", "w")
    create_tree_ino_evaluate(file, clf)
    file.close()

    file = open("ino_tree2/decision_tree.cpp", "w")
    create_tree_ino_evaluate(file, clf)
    file.close()

    file = open("decision_forest.c", "w")
    create_forest_native_main(file, trees, 64)
    file.close()

    file = open("ino_tree/decision_forest.cpp", "w")
    create_forest_ino_evaluate(file, trees, 32)
    file.close()

    file = open("ino_tree2/decision_forest.cpp", "w")
    create_forest_ino_evaluate(file, trees, 32)
    file.close()


def random_forest():
    clf = RandomForestClassifier(criterion='entropy', n_estimators=64, random_state=1, n_jobs=16)
    clf = clf.fit(X_train, y_train)

    predicted = clf.predict(XX_test)

    print("Evaluating RandomForestClassifier:")
    evaluate_predicted(predicted, yy_test)


decision_tree()
random_forest()
