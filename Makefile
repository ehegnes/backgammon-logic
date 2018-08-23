CARGO_PATH ?= $(HOME)/.cargo

all: coverage

build:
	cargo build

test:
	cargo test

$(CARGO_PATH)/bin/cargo-coverage:
	cargo install cargo-travis

coverage: $(CARGO_PATH)/bin/cargo-coverage test
	cargo coverage

clean:
	cargo clean
