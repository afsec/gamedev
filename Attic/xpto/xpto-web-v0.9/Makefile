JS_INPUT_FILE=main-debug.js
JS_OUTPUT_FILE=./dist/main-compiled.js

TARGET=wasm32-unknown-unknown
WASM=./target/$(TARGET)/release/backend.wasm

all: backend frontend run

pre-build:
	mkdir -p dist

frontend: pre-build
	cp index.html ./dist/
	cat ./js/back-end.js ./js/bundle.js ./js/loader.js > main-debug.js
	#java -jar ~/bin/closure-compiler.jar -O ADVANCED_OPTIMIZATIONS --js $(JS_INPUT_FILE) --js_output_file $(JS_OUTPUT_FILE) --language_in ECMASCRIPT_2018 --language_out ECMASCRIPT_2015
	cp -v $(JS_INPUT_FILE) $(JS_OUTPUT_FILE)

backend: pre-build
	TARGET=wasm32-unknown-unknown

	cargo build --target $(TARGET) --release
	wasm-strip $(WASM)
	wasm-opt -o ./dist/xpto.wasm -Oz $(WASM)

run:
	ls -lh ./dist
	./scripts/run.sh


clean:
	rm -r ./dist
