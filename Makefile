.PHONY: all build doc clean

all: build

build:
	cargo build

doc:
	cargo doc

clean:
	cargo clean

check:
	cargo test
