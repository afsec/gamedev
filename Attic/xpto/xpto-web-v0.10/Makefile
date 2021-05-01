TARGET=wasm32-unknown-unknown
WASM=./target/$(TARGET)/release/wasm_sample.wasm

all: backend frontend run

pre-build:
	mkdir -p dist

frontend: pre-build
	cp ./www/index.html ./dist/

backend: pre-build
	TARGET=wasm32-unknown-unknown

	cargo build --target $(TARGET) --release
	wasm-strip $(WASM)
	wasm-opt -o ./dist/demo.wasm -Oz $(WASM)

run:
	ls -lh ./dist
	./scripts/run.sh


clean:
	rm -r ./dist
