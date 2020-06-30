#!/bin/bash

set -e
set -o nounset
set -o pipefail

SCRIPT_DIR=$(cd "$(dirname "$0")" ; pwd -P)
EXECUTABLE=$(basename "$SCRIPT_DIR")

_pretest() {
  type bats > /dev/null 2>&1 || { echo "run brew install bats to install bats"; exit 1; }
  type "${EXECUTABLE}" > /dev/null 2>&1 || { echo "run ./go install-cli to install the executable"; exit 1; }
}

goal_install-cli() {
  cargo install --path .
}

goal_test() {
  _pretest
  bats tests.bats
}

goal_build-release() {
  TARGET=x86_64-unknown-linux-musl
  # Dark sorcery to get a binary that can be used in linux
  CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=$TARGET
  upx -9 "target/$TARGET/release/$EXECUTABLE"
}

validate-args() {
  acceptable_args="$(declare -F | sed -n "s/declare -f goal_//p" | tr '\n' ' ')"

  if [[ -z $1 ]]; then
    echo "usage: $0 <goal>"
    # shellcheck disable=SC2059
    printf "\n$(declare -F | sed -n "s/declare -f goal_/ - /p")"
    exit 1
  fi

  if [[ ! " $acceptable_args " =~ .*\ $1\ .* ]]; then
    echo "Invalid argument: $1"
    # shellcheck disable=SC2059
    printf "\n$(declare -F | sed -n "s/declare -f goal_/ - /p")"
    exit 1
  fi
}

CMD=${1:-}
shift || true
if validate-args "${CMD}"; then
  "goal_${CMD}"
  exit 0
fi
