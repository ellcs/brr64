#!/bin/bash
PARENT_DIR="$(dirname "$(realpath "$0")")"
source "${PARENT_DIR}/common.sh"

exec_and_echo "rustup override set nightly"
exec_and_echo "rustup target add wasm32-unknown-unknown"
exec_and_echo "cargo install wasm-gc"

