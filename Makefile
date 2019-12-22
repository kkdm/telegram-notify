_VERSION := v0.1.0

all: build

release:
	git tag $(_VERSION)
	git push origin $(_VERSION)

test:
	cargo test

build:
	cargo build --release

clean: 
	@rm -rf target

.PHONY: build test
