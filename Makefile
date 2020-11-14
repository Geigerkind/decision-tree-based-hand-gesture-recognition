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
	$(CARGO) run --release --bin gesture_extractor

test_gen_features:
	$(CARGO) test

gen_tree:
	$(PYTHON) proof_of_concept/decision_tree.py
	$(GCC) -O3 decision_tree.c -o decision_tree

test_tree:
	./decision_tree -0.005887781804276335 -0.3834534038101755 -0.5237253907563562 0.3613164317951552 0.42083984039758776 0.047112207974439525 0.06720038456791605 0.3244179796532372 0.5289210626186946 0.4284159988947223 0.10567014692191572 0.061454696051594405
