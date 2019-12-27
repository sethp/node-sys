.PHONY: check
check:
	RUSTFLAGS="-Dwarnings" cargo check --all-features --all-targets --benches --bins --examples --tests --workspace

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets --benches --bins --examples --tests --workspace -- -D warnings

.PHONY: test
test:
	wasm-pack test --node -- --locked --all-features --all-targets --benches --bins --examples --tests --workspace
