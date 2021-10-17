#!/bin/bash
PARENT_DIR="$(dirname "$(realpath "$0")")"
source "${PARENT_DIR}/common.sh"

BRR64_IMAGE_NAME="rust-alpine-with-bash"

function brr64_podman_build() {
  tmp_container_name=$(buildah from docker.io/rust:alpine)
  buildah run "$tmp_container_name" -- apk update
  buildah run "$tmp_container_name" -- apk add bash

  buildah commit "$tmp_container_name" "$BRR64_IMAGE_NAME"
}

function brr64_podman_run() {
  podman run --rm -v $(pwd):/code -it "$BRR64_IMAGE_NAME" /bin/bash /code/sh/runner_github_action.sh
}

eval "brr64_podman_$1"
