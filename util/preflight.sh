#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Pre-push tests
#
# usage:
# ./util/preflight.sh
#

set -e

xcall() {
	sh -cx "cargo clean"
	sh -cx "$*"
}

test -d ./util/
# shellcheck source=common.sh
. ./util/common.sh

plus_line
echo "Starting pre-flight validation"
plus_line
echo

plus_header "Now validating environment"
test -d ./fuzz
test -d ./util
plus_ok

plus_header "Now validating git"
if [ "$(git status --porcelain=v1 2>/dev/null | wc -l)" -ne 0 ]; then
	echo "ERROR: untracked files:"
	echo
	git status
	echo
	exit 1
fi
plus_ok

plus_header "Now validating build"
xcall cargo build
plus_ok

plus_header "Now validiating clippy"
xcall cargo clippy -- -Dwarnings
plus_ok

plus_header "Now validating build without default features"
xcall cargo build --no-default-features
plus_ok

plus_header "Now validating build with sink feature"
xcall cargo build --features sink
plus_ok

plus_header "Now validating build with save feature"
xcall cargo build --features save
plus_ok

plus_header "Now validating test"
xcall cargo test
plus_ok

plus_header "Now validating docs"
xcall cargo doc --document-private-items
plus_ok

# Build and run timed benchmarks
plus_header "Now validating timed benchmarks"
xcall cargo bench --bench timed
plus_ok

# Build and run ryan benchmarks
plus_header "Now validating ryan benchmarks"
xcall ./util/benchr.sh
plus_ok

# Run fuzzer on known inputs
plus_header "Now validating fuzzer in default mode"
cargo install cargo-fuzz
xcall cargo fuzz run roger --sanitizer none --release -- -runs=0
plus_ok

plus_header "Now validating fuzzer with a minimal interface"
cargo install cargo-fuzz
xcall cargo fuzz run roger --sanitizer none --release --no-default-features -- -runs=0
plus_ok

plus_header "Now validating score using ryan"
xcall cargo run --features score --release --bin ryan fuzz/corpus/roger/gold
plus_ok

echo
plus_line
echo "ALL OK"
echo "Passed all pre-flight validation"
plus_line
