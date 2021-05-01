#!/bin/bash
SRC=./src
DIST=./dist



create_dist() {
    mkdir -p ${DIST}
}

compile() {
    wat2wasm ${SRC}/main.wat -o ${DIST}/main.wasm
}


show_files() {
    tree ${DIST}
    printf "\n\n\n"
}

run() {
    wasmtime ${DIST}/main.wasm
}

main() {
    create_dist;
    compile;
    show_files;
    run;


}


main

