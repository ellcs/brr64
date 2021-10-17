#!/bin/bash
PARENT_DIR="$(dirname "$(realpath "$0")")"
source "${PARENT_DIR}/common.sh"

exec_and_echo "cargo test --verbose"
