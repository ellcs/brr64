#!/bin/bash
pushd /code
bash sh/rustup.sh
bash sh/tests.sh
bash sh/build_wasm.sh
popd
