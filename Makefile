fmt:
	cargo fmt -- --check --verbose

build:
	trunk build --release

requirements:
	cargo install --locked trunk
	rustup target add wasm32-unknown-unknown
