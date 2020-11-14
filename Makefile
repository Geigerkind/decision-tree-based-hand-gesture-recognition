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

test: test_gen_features test_tree

gen_features: install_rust_nightly
	DATA_PATH="." $(CARGO) run --release --bin feature_extractor

test_gen_features:
	DATA_PATH=".." $(CARGO) test

gen_tree:
	$(PYTHON) model/decision_tree.py
	$(GCC) -O3 decision_tree.c -o decision_tree
	$(GCC) -O3 decision_forest.c -o decision_forest

test_tree:
	DATA_PATH=".." $(CARGO) test --bin simulation -- --nocapture

test_tree_kubik:
	DATA_PATH=".." $(CARGO) test test_kubik_test_by_annotation --bin simulation -- --nocapture