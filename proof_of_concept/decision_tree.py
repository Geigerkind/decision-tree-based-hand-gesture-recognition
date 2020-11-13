import pandas as pd
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split

# from sklearn import tree
# from matplotlib import pyplot as plt

# Import all the training and validation data
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
# clf = tree.DecisionTreeClassifier()
clf = RandomForestClassifier(criterion='entropy', n_estimators=64, random_state=1, n_jobs=16)
clf = clf.fit(X_train, y_train)

# plt.figure()
# tree.plot_tree(clf)
# plt.savefig('tree.png', format='png')

# Testing the validation set for accuracy
predicted = clf.predict(X_test)
correct = 0
for i in range(len(y_test)):
    if predicted[i] == y_test[i]:
        correct = correct + 1
print(100 * (correct / len(y_test)))
