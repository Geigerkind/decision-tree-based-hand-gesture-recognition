import pandas as pd
from sklearn.ensemble import RandomForestClassifier

# from sklearn import tree
# from matplotlib import pyplot as plt

# Import all the training and validation data
# Generated from the extractor
storage_path = "../prepare_data/gesture_extractor/model_data"
train_result = pd.read_csv(storage_path + "/train/result", dtype=int).values.flatten()
train_lsos_x = pd.read_csv(storage_path + "/train/LocalSumOfSlopesX", dtype=int)
train_lsos_y = pd.read_csv(storage_path + "/train/LocalSumOfSlopesY", dtype=int)
train_darkness_dist_3x = pd.read_csv(storage_path + "/train/DarknessDistribution3X", dtype=float)
train_darkness_dist_6x = pd.read_csv(storage_path + "/train/DarknessDistribution6X", dtype=float)
train_darkness_dist_3y = pd.read_csv(storage_path + "/train/DarknessDistribution3Y", dtype=float)
train_darkness_dist_6y = pd.read_csv(storage_path + "/train/DarknessDistribution6Y", dtype=float)
train_darkness_dist_6xy = pd.read_csv(storage_path + "/train/DarknessDistribution6XY", dtype=int)
train_darkness_dist_6xy_geom = pd.read_csv(storage_path + "/train/DarknessDistribution6XYGeom", dtype=int)
train_darkness_dist_6xy_quadrant = pd.read_csv(storage_path + "/train/DarknessDistribution6XYQuadrant", dtype=int)
train_brightness_dist_3x = pd.read_csv(storage_path + "/train/BrightnessDistribution3X", dtype=float)
train_brightness_dist_6x = pd.read_csv(storage_path + "/train/BrightnessDistribution6X", dtype=float)
train_brightness_dist_3y = pd.read_csv(storage_path + "/train/BrightnessDistribution3Y", dtype=float)
train_brightness_dist_6y = pd.read_csv(storage_path + "/train/BrightnessDistribution6Y", dtype=float)
train_brightness_dist_6xy = pd.read_csv(storage_path + "/train/BrightnessDistribution6XY", dtype=int)
train_brightness_dist_6xy_geom = pd.read_csv(storage_path + "/train/BrightnessDistribution6XYGeom", dtype=int)
train_brightness_dist_6xy_quadrant = pd.read_csv(storage_path + "/train/BrightnessDistribution6XYQuadrant", dtype=int)
train_motion_history = pd.read_csv(storage_path + "/train/MotionHistory", dtype=int)
train_mean_value = pd.read_csv(storage_path + "/train/MeanValue", dtype=int)
train_minimum_value = pd.read_csv(storage_path + "/train/MinimumValue", dtype=int)
train_maximum_value = pd.read_csv(storage_path + "/train/MaximumValue", dtype=int)
train_standard_deviation = pd.read_csv(storage_path + "/train/StandardDeviation", dtype=float)
train_average_amplitude_change = pd.read_csv(storage_path + "/train/AverageAmplitudeChange", dtype=int)
train_direction_map_x = pd.read_csv(storage_path + "/train/DirectionMapX", dtype=int)
train_direction_map_y = pd.read_csv(storage_path + "/train/DirectionMapY", dtype=int)

validate_result = pd.read_csv(storage_path + "/validate/result", dtype=int).values.flatten()
validate_lsos_x = pd.read_csv(storage_path + "/validate/LocalSumOfSlopesX", dtype=int)
validate_lsos_y = pd.read_csv(storage_path + "/validate/LocalSumOfSlopesY", dtype=int)
validate_darkness_dist_3x = pd.read_csv(storage_path + "/validate/DarknessDistribution3X", dtype=float)
validate_darkness_dist_6x = pd.read_csv(storage_path + "/validate/DarknessDistribution6X", dtype=float)
validate_darkness_dist_3y = pd.read_csv(storage_path + "/validate/DarknessDistribution3Y", dtype=float)
validate_darkness_dist_6y = pd.read_csv(storage_path + "/validate/DarknessDistribution6Y", dtype=float)
validate_darkness_dist_6xy = pd.read_csv(storage_path + "/validate/DarknessDistribution6XY", dtype=int)
validate_darkness_dist_6xy_geom = pd.read_csv(storage_path + "/validate/DarknessDistribution6XYGeom", dtype=int)
validate_darkness_dist_6xy_quadrant = pd.read_csv(storage_path + "/validate/DarknessDistribution6XYQuadrant", dtype=int)
validate_brightness_dist_3x = pd.read_csv(storage_path + "/validate/BrightnessDistribution3X", dtype=float)
validate_brightness_dist_6x = pd.read_csv(storage_path + "/validate/BrightnessDistribution6X", dtype=float)
validate_brightness_dist_3y = pd.read_csv(storage_path + "/validate/BrightnessDistribution3Y", dtype=float)
validate_brightness_dist_6y = pd.read_csv(storage_path + "/validate/BrightnessDistribution6Y", dtype=float)
validate_brightness_dist_6xy = pd.read_csv(storage_path + "/validate/BrightnessDistribution6XY", dtype=int)
validate_brightness_dist_6xy_geom = pd.read_csv(storage_path + "/validate/BrightnessDistribution6XYGeom", dtype=int)
validate_brightness_dist_6xy_quadrant = pd.read_csv(storage_path + "/validate/BrightnessDistribution6XYQuadrant",
                                                    dtype=int)
validate_motion_history = pd.read_csv(storage_path + "/validate/MotionHistory", dtype=int)
validate_mean_value = pd.read_csv(storage_path + "/validate/MeanValue", dtype=int)
validate_minimum_value = pd.read_csv(storage_path + "/validate/MinimumValue", dtype=int)
validate_maximum_value = pd.read_csv(storage_path + "/validate/MaximumValue", dtype=int)
validate_standard_deviation = pd.read_csv(storage_path + "/validate/StandardDeviation", dtype=float)
validate_average_amplitude_change = pd.read_csv(storage_path + "/validate/AverageAmplitudeChange", dtype=int)
validate_direction_map_x = pd.read_csv(storage_path + "/validate/DirectionMapX", dtype=int)
validate_direction_map_y = pd.read_csv(storage_path + "/validate/DirectionMapY", dtype=int)

# Create the decision tree and train it
# clf = tree.DecisionTreeClassifier()
clf = RandomForestClassifier(criterion='entropy', n_estimators=64, random_state=1, n_jobs=16)
train_features = pd.concat(
    [train_lsos_x, train_lsos_y, train_darkness_dist_6xy_geom, train_brightness_dist_6xy_geom, train_motion_history],
    axis=1)
clf = clf.fit(train_features.values, train_result)

# plt.figure()
# tree.plot_tree(clf)
# plt.savefig('tree.png', format='png')

# Testing the validation set for accuracy
validate_features = pd.concat(
    [validate_lsos_x, validate_lsos_y, validate_darkness_dist_6xy_geom, validate_brightness_dist_6xy_geom,
     validate_motion_history],
    axis=1)
predicted = clf.predict(validate_features.values)
correct = 0
for i in range(len(validate_result)):
    if predicted[i] == validate_result[i]:
        correct = correct + 1
print(100 * (correct / len(validate_result)))
