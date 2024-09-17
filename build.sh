#!/bin/sh

export CARGO_MANIFEST_DIR=$(pwd)

cargo build -Zbuild-std=core,alloc --target x86_64-unknown-none

cargo bootimage

