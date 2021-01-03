## Setup all variables
CARGO = cargo
PYTHON = python3
GCC = gcc

## Development tasks

all: gen_features gen_tree test_gen_features test_tree

gen: gen_features gen_tree

test: test_gen_features test_tree

gen_features:
	DATA_PATH="." $(CARGO) run --release --bin feature_extractor

test_gen_features:
	DATA_PATH=".." $(CARGO) test

gen_tree:
	-$(PYTHON) model/decision_tree.py 14 10 1 4 1 1 0.5 0.0 4 0 "" 16 1
	#$(GCC) -O2 decision_tree.c -o decision_tree
	$(GCC) -O2 decision_forest.c -o decision_forest

test_tree:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set1" -- --nocapture

test_tree_kubik:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test test_kubik_test_by_annotation --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set1" -- --nocapture

test_tree_klisch:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test test_klisch_test_by_annotation --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set2" -- --nocapture

test_tree_dymel:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test test_dymel_test --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set10" -- --nocapture

test_tree_light:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test test_dymel_test_light --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set2" -- --nocapture

test_tree_contrast:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test test_dymel_test_vanashing_contrast --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set2" -- --nocapture

playground:
	DATA_PATH="." $(CARGO) run --release --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set1"

doc:
	DATA_PATH="." $(CARGO) doc --open --no-deps

reader:
	DATA_PATH="." $(CARGO) run --release --bin serial_reader

recorder:
	DATA_PATH="." $(CARGO) run --release --bin gesture_recorder