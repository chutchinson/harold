#!/usr/bin/env bash

cd crates
cargo run -p harold-fmt ../examples/fib.asm
cargo run -p harold-as
cargo run -p harold-vm
cd ..