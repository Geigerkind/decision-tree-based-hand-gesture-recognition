## Setup all variables
RUSTUP = rustup
NIGHTLY_TOOLCHAIN = nightly
CARGO = cargo
PYTHON = python
GCC = gcc

INSTALLED_TOOLCHAINS = $(shell $(RUSTUP) toolchain list)
INSTALLED_COMPONENTS = $(shell $(RUSTUP) component list --installed --toolchain $(NIGHTLY_TOOLCHAIN))
INSTALLED_NIGHTLY_COMPONENTS = $(shell $(RUSTUP) component list --installed --toolchain $(NIGHTLY_TOOLCHAIN))

## Dev environment
install_rust_nightly:
ifeq (,$(findstring $(NIGHTLY_TOOLCHAIN),$(INSTALLED_TOOLCHAINS)))
	$(RUSTUP) install $(NIGHTLY_TOOLCHAIN)
endif

## Initial setup

setup: install_rust_nightly

## Development tasks

all: gen_features gen_tree test_gen_features test_tree

gen: gen_features gen_tree

test: test_gen_features test_tree

gen_features: install_rust_nightly
	DATA_PATH="." $(CARGO) run --release --bin feature_extractor

test_gen_features:
	DATA_PATH=".." $(CARGO) test

gen_tree:
	-$(PYTHON) model/decision_tree.py 22 3 1 2 0 5 0.3 0.0 1 0 "" 16
	$(GCC) -O2 decision_tree.c -o decision_tree
	$(GCC) -O2 decision_forest.c -o decision_forest

test_tree:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set1" -- --nocapture

test_tree_kubik:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test test_kubik_test_by_annotation --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set1" -- --nocapture

test_tree_klisch:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test test_klisch_test_by_annotation --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set1" -- --nocapture

test_tree_dymel:
	DATA_PATH=".." PROGRAM_PATH="./.." $(CARGO) test test_dymel_test --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set1" -- --nocapture

playground:
	DATA_PATH="." $(CARGO) run --release --bin simulation --manifest-path "simulation/Cargo.toml" --features "feature_set1"

doc:
	DATA_PATH="." $(CARGO) doc --open --no-deps

reader:
	DATA_PATH="." $(CARGO) run --release --bin serial_reader

recorder:
	DATA_PATH="." $(CARGO) run --release --bin gesture_recorder