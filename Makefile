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

all: gen_features gen_tree test_tree

gen_features: install_rust_nightly
	$(CARGO) run --release

gen_tree:
	$(PYTHON) proof_of_concept/decision_tree.py
	$(GCC) -O3 decision_tree.c -o decision_tree

test_tree:
	./decision_tree -2 16 102 -14 81 92 7 6 3 3 8 8 3 4 1 4 3 3 256 512 1024 512 1024 2048 512 1024 2048
