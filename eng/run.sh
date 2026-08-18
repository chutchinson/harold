#!/usr/bin/env bash

cd crates
cargo run -p harold-fmt ../examples/debug.asm
cargo run -p harold-as
cd ..