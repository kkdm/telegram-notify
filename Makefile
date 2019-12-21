_VERSION := v0.1.0

all: build

build:
	cargo build --release

clean: 
	@rm -rf target

.PHONY: build
