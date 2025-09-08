#!/bin/sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# x800 shell benchmark
#
# usage:
# ./util/bench-shell.sh [ARGS]
#
set -e

# Check without argument whether
# the first argument given is executable
exists() {
    command -v "$1" >/dev/null
}

# Validated a provided executable name, with preference for GNU coreutils.
# A GNU version of a tool 'TOOL' is often named 'gTOOL'. Prefer any such explicitly GNU tool.
# The fallback is any valid executable of the provided name.
compat() {
	if exists "g$1"; then
		echo "g$1"
	elif exists "$1"; then
		echo "$1"
	else
		echo "Tool not found: $1" >/dev/stderr
		exit 1
	fi
}

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
