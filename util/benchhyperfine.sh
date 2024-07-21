#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

set -e

test -d ./util/
test -f ./util/benchgen.sh

# shellcheck source=common.sh
. ./util/common.sh

plus_header "Building project"
cargo build -p x800 --release --features sink

plus_header "Installing hyperfine"
cargo install --quiet hyperfine

plus_header "Running with random input"
touch /tmp/moves
hyperfine \
	--prepare './util/benchgen.sh /tmp/moves' \
	--warmup=256 \
	--runs=256 \
	--input='/tmp/moves' \
	-N \
	'./target/release/x800'

GOLD_FILE="$(realpath ./fuzz/corpus/roger/gold)"
GOLD_LOOPS="1000"
GOLD_REALPATH="$(realpath fuzz/corpus/roger/gold)"
plus_header "Running with gold file input"
printf "PATH:           %s\nGAME SAVE SIZE: %s\nGAMES PER RUN:  %s\n" "${GOLD_REALPATH}" "$(du -b "${GOLD_REALPATH}" | cut -f1)" "${GOLD_LOOPS}"

hyperfine \
	--warmup=32 \
	--runs=32 \
	-N \
	"./target/release/ryan ${GOLD_FILE} ${GOLD_LOOPS}"
