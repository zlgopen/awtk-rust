#!/bin/bash

sh_dir=$(cd "$(dirname "$0")" && pwd)

cd "$sh_dir"

if ! command -v cargo-deny &> /dev/null; then
    echo "install cargo-deny..."
    cargo install cargo-deny --force
fi

echo "check licenses..."
cargo deny check licenses

cd -
