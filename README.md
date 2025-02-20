# Decision tree based gesture recognition
## Introduction
This project is split into several sub projects to abstract from the domain of gesture recognition on the Arduino Uno. 

### Documentation
Please refer to the `Makefile` on how to build the documentation for all projects, for a more detailed description of the 
individual functions and structures. Below a brief summary of each module.

### lib_gesture
Gesture (candidates) on a 3x3 light sensor array were collected by people before this work. These gestures include LeftToRight, 
RightToLeft, TopToBottom and BottomToTop movements and can be found in the `data` directory. A line in such a file contains between 9 and 27 values.
A detailed description of these values can be obtained from Dr. Venzke. The library parses the first 9 as the 3x3 light sensors, and the 10th value 
as gesture. It defines what is a `GestureType`, `Frame` and `Gesture`(Candidate). It provides one method to parse a stream of frames 
to a gesture (candidate) by the threshold `(GestureReader)`, one method to parse a file by threshold `parse_gestures_by_threshold`, and one 
method to parse a file by annotation `parse_gestures_by_annotation`.

### lib_feature
Once these gesture (candidates) are parsed, features can be extracted from them. This library defines how a certain parameter (features) can be 
extracted from a gesture. Once a feature is defined in terms of the `Feature` trait, it has to be added to the `FeatureType` in order to be 
automatically generated by the `feature_extractor`.

### lib_data_set
This library standardizes what a data set is and provides utility constants to automatically get a collection of the desired data set to work with. 
A `DataSet` is a collection of `DataSetEntry` with different `CoveringObject`s (e.g. Hand, Finger), `CameraDistance`s, `BrightnessLevel`s and 
`AdditionalSpecification`s like fast or slow. A set can be used by importing the data set and using it as a constant, e.g. 
`EVA_9PIXEL.get(&ParsingMethod::ByAnnotation).unwrap()`.

### lib_evaluation
This library provides a utility structure called `Evaluation`. It contains `EvaluationEntry` and uses the value objects defined in `lib_data_set` to 
construct an evaluation that can be as detailed as it can be and as undetailed as just printing out the accuracy for a whole data set.

### feature_extractor
A utility using above libraries to extract all defined features from specified data sets and puts them into `model_data`. For each feature a file 
is created with the feature's structure name. `result` is the `GestureType` that was annotated in the test data.

### model
This project uses `Scikit-learn` to train a `DecisionTreeClassifier` multiple times and uses the best to generate C-code for a decision tree and 
decision forest. It uses the features generated by the `feature_extractor`.

### simulation
This is a program that listens to the serial port `/dev/ttyACMO` (depending on your operating system, you need to change this) and waits for a stream of 
frames from the Arduino Uno. It then feeds the frames into the `GestureReader`, and calculates, once a `Gesture`(Candidate) is recognized the feature vector, 
and finally feeds it into the decision tree or decision forest that is generated in `model/decision_tree.py`.  
Furthermore, it tests the aforementioned tree classifier on the data sets and prints out `Evaluation`s for each data set parsed `ByAnnotation` and `ByThreshold`.

### ino_tree
This project is the final product. It combines all the work and provides the chain of gesture candidate parsing, feature extraction and classification using either 
the decision tree or decision forest. It finally outputs its prediction to the serial interface.

### ino_tree2
The same as `ino_tree` but uses Dr. Venzke`s gesture recognizer.

### ino_tree3
The same as `ino_tree2` but calculates parts of the features during frame collection.

### serial_reader
This program only prints each line that it encounters in the serial data stream from the Arduino.

### gesture_recorder
This program uses the serial stream from the Arduino and the `GestureReader` to collect labeled gestures.

## Toolchain
* Cargo nightly (I used `cargo 1.49.0-nightly (79b397d72 2020-10-15)`)
* Python 3.8 (also matplotlib, scikit-learn etc.)
* make
* gcc

## Makefile
This is a utility to simplify the toolchain of this project to a single command.

### doc
Opens documentation for all rust sub projects.

### gen_features
Generates the features with the `feature_extractor`.

### gen_tree
Generates the decision tree and decision forest using our `model` and compiles it to a binary.

### gen
Executes gen_features, then gen_tree

### test_gen_features
Executes all tests in the libraries.

### test_tree
Evaluates all data sets on the previously generated trees.

### test_tree_kubik
Only evaluates the `KUBIK_TEST` data set.

### test_tree_klisch
Only evaluates the `KLISCH_TEST` data set.

### test
Executes test_gen_features, then test_tree

### playground
Starts the `simulation` program and listens for frames from the Arduino Uno.

### reader
Starts the `serial_reader` program and prints the serial output from the Arduino Uno.

### recorder
Starts the `gesture_recorder` program.

### all
Executes in this order: gen_features, gen_tree, test_gen_features, test_tree
