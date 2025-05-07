#!/bin/bash

set -e

# Try running Rust binary
# exec ./target/release/<your-rust-binary-name> 2>/dev/null || \

# Try running C/C++ binary
# exec ./main 2>/dev/null || \

# Try running Go binary
# exec ./app 2>/dev/null || \

# Fallback if none found
# echo "No executable found. Did you run build.sh?" && exit 1

MALLOC_CONF="thp:always,metadata_thp:always" ./sorter/target/release/sorter
