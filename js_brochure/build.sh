#!/bin/bash
rm -f Book-debug.md

cat *.md > Book-debug.md

pandoc -s -V papersize:a5 -V geometry:landscape Book-debug.md -o js_book.pdf
