#!/usr/bin/env bash
set -e

SRC=$(realpath $1)

cd crates
cargo run -p harold-fmt $SRC
cargo run -p harold-as $SRC -o target/rom.bin
cargo run -p harold-vm target/rom.bin
cd ..
