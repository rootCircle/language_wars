#!/bin/bash

set -e

# Rust
# cargo build --release

# C/C++
# gcc -O3 -DNDEBUG -o main main.c

# Go
# GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -ldflags="-s -w" -o app main.go

cd sorter
MALLOC_CONF="thp:always,metadata_thp:always" cargo build --release

