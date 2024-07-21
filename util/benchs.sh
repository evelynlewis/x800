#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# x800 shell benchmark
#
# usage:
# ./util/benchs.sh [ARGS]
#
set -e

# source environmental compatibility script
# shellcheck source=compat.sh
. ./util/compat.sh

# Setup tools
BASE32=$(compat base32)
TR=$(compat tr)
DD=$(compat dd)

# Build
echo "Building x800 executable."
cargo build --release --features sink "${@}"

# Other params
BS="8192"
SLEEP="16"
echo "Running benchmark for ${SLEEP} seconds."

# Configure cleanup
trap 'kill $(pgrep -P $$) 2> /dev/null' EXIT INT

# Run worker
${BASE32} </dev/urandom |
    ${TR} -dC 'ASDW' |
    ${DD} conv=lcase bs=${BS} status=progress |
    sh -c 'while true; do ./target/release/x800; done' &

# Sleep and exit
sleep ${SLEEP}
exit 0
