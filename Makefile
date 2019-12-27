.PHONY: check
check:
	RUSTFLAGS="-Dwarnings" cargo check --all-features --all-targets --benches --bins --examples --tests --workspace

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets --benches --bins --examples --tests --workspace -- -D warnings

.PHONY: fmt
fmt:
	cargo +nightly fmt --all

.PHONY: test
test:
	RUSTFLAGS="-Dwarnings" wasm-pack test --node -- --locked --all-features --all-targets --benches --bins --examples --tests --workspace
