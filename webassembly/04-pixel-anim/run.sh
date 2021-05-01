#!/bin/bash
SRC=./src
DIST=./dist



create_dist() {
    mkdir -p ${DIST}
}

compile() {
    wat2wasm ${SRC}/main.wat -o ${DIST}/main.wasm
}

copy_files() {
    cp -v ${SRC}/main.html ${DIST}
    cp -v ${SRC}/main.js ${DIST}
}

show_files() {
    tree ${DIST}
}

run_server() {
    cd ${DIST}
    python3 -mhttp.server
}

main() {
    create_dist;
    compile;
    copy_files;
    show_files;
    run_server;


}


main

