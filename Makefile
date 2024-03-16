.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: run
run: build
	cargo run -- $(ARGS)

.PHONY: test
test: 
	cargo test

.PHONY: clean
clean:  
	rm -rf target