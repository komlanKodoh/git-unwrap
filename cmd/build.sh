#!/bin/bash

cd "$(dirname "$0")" && cd ..

cargo build --release && cp ./target/release/git-unwrap ./bin