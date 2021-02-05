.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo +nightly-2020-10-06 test --release --all

.PHONY: run
run:
	 cargo run --release -- --dev --tmp

.PHONY: build
build:
	 cargo +nightly-2020-10-06 build --release

.PHONY: benchmark
benchmark:
	 cargo +nightly-2020-10-06 build --manifest-path node/Cargo.toml --release --features runtime-benchmarks

