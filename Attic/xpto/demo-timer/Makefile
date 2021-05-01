INPUT_FILE=main-debug.js
OUTPUT_FILE=./dist/main-compiled.js

all: build run

pre-build:
	mkdir -p dist
	cp index.html ./dist/
	cat ./js/back-end.js ./js/bundle.js ./js/loader.js > main-debug.js

build: pre-build
	java -jar ~/bin/closure-compiler.jar -O ADVANCED_OPTIMIZATIONS --js $(INPUT_FILE) --js_output_file $(OUTPUT_FILE) --language_in ECMASCRIPT_2018 --language_out ECMASCRIPT_2015

run:
	./scripts/run.sh


clean:
	rm -r ./dist
