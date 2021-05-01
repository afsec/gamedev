TARGET=wasm32-unknown-unknown
WASM=./target/$(TARGET)/release/chip8.wasm

all: backend frontend run

pre-build:
	mkdir -p dist

frontend: pre-build
	cp ./www/index.html ./dist/

backend: pre-build
	TARGET=wasm32-unknown-unknown

	cargo build --target $(TARGET) --release
	wasm-strip $(WASM)
	wasm-opt -o ./dist/chip8.wasm -Oz $(WASM)

check:
	cargo check --target $(TARGET) --release

test:
	cargo test --release --package wasm_stdlib

run:
	ls -lh ./dist
	./scripts/run.sh


clean:
	rm -rf ./dist
	cargo clean
