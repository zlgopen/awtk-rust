#!/bin/bash

sh_dir=$(cd "$(dirname "$0")" && pwd)

# Usage: $0 <source_dir> <awtk_rust_gen>
source_dir="$sh_dir/../../awtk"
awtk_rust_gen="$sh_dir/../awtk_rust_gen/target/debug/awtk_rust_gen.exe"

if [ $# -gt 1 ]; then
    source_dir="$1"
    if [ $# -gt 2 ]; then
        awtk_rust_gen="$2"
    fi
fi

echo "source_dir : $source_dir"
echo "awtk_rust_gen : $awtk_rust_gen"

mkdir -p "$sh_dir/libs"

libs=("awtk.lib" "awtk.dll" "awtk.so" "awtk.dylib")
for lib in "${libs[@]}" ; do
    src="$source_dir/bin/$lib"
    dst="$sh_dir/libs/$lib"
    if [ -f "$src" ]; then
        cp "$src" "$dst"
        echo "$src => $dst"
    fi
done

"$awtk_rust_gen" -h "$source_dir/src/awtk.h" -i "$source_dir/tools/idl_gen/idl.json" -p "$source_dir/awtk_config.py" -o "$sh_dir/src/awtk.rs"
