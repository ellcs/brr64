#!/bin/bash
PARENT_DIR="$(dirname "$(realpath "$0")")"
source "${PARENT_DIR}/common.sh"

exec_and_echo "cargo build -p brr64-wasm --target wasm32-unknown-unknown --release"
exec_and_echo "wasm-gc target/wasm32-unknown-unknown/release/wasm_brr64.wasm"
exec_and_echo "mv target/wasm32-unknown-unknown/release/wasm_brr64.wasm docs/"
exec_and_echo "sed -i \"s/CURRENT_COMMIT_HASH/\$(git rev-parse --short HEAD)/g\" docs/index.html"
