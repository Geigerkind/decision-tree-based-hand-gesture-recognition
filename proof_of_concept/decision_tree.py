import pandas as pd
from matplotlib import pyplot as plt
from sklearn import tree
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split


def evaluate_predicted(predicted, y_test):
    true_positive = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    false_positive = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    for i in range(len(y_test)):
        if predicted[i] == y_test[i]:
            true_positive[predicted[i]] += 1
        else:
            false_positive[predicted[i]] += 1

    total_gestures = len(y_test)
    for gesture_type in [1, 2, 3, 4, 9]:
        amount_of_gesture = y_test.tolist().count(gesture_type)
        print("GestureType: " + str(gesture_type))
        print("True Positive: %.3f" % (100 * (true_positive[gesture_type] / amount_of_gesture)))
        print("False Positive: %.3f" % (100 * (false_positive[gesture_type] / total_gestures)))


# Import all the data
# Generated from the extractor
storage_path = "../prepare_data/gesture_extractor/model_data"
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

# Specifying the features
X = pd.concat([lsos_x, lsos_y, darkness_dist_6xy_geom, brightness_dist_6xy_geom, motion_history], axis=1).values
y = result
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.3, random_state=0)


# Create the decision tree and train it
def decision_tree(X_train, y_train, X_test, y_test):
    clf = tree.DecisionTreeClassifier()
    clf = clf.fit(X_train, y_train)

    plt.figure()
    tree.plot_tree(clf)
    plt.savefig('tree.png', format='png')

    predicted = clf.predict(X_test)

    print("Evaluating DecisionTreeClassifier:")
    evaluate_predicted(predicted, y_test)


def random_forest(X_train, y_train, X_test, y_test):
    clf = RandomForestClassifier(criterion='entropy', n_estimators=64, random_state=1, n_jobs=16)
    clf = clf.fit(X_train, y_train)

    predicted = clf.predict(X_test)

    print("Evaluating RandomForestClassifier:")
    evaluate_predicted(predicted, y_test)


decision_tree(X_train, y_train, X_test, y_test)
random_forest(X_train, y_train, X_test, y_test)
