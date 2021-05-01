PRG_NAME=xpto
TARGET=wasm32-unknown-unknown

all: build run

check:
	cargo check --release --target=x86_64-unknown-linux-musl


build:
	@mkdir -p ./dist
	@cargo build --release --target=$(TARGET)
	@cp -v ./target/$(TARGET)/release/$(PRG_NAME).wasm ./dist
	@wasm-strip ./dist/$(PRG_NAME).wasm
	@ls -lh ./dist/


run:
	@cd ./dist/ ; python3 -mhttp.server
