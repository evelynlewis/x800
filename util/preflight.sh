#!/bin/sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Pre-push tests
#
# usage:
# ./util/preflight.sh
#

set -e

# `cargo clean` and run
xcall() {
	sh -cx "cargo clean"
	sh -cx "$*"
}

# `cargo` without clean
xcargo() {
	sh -cx "cargo $*"
}

test -d ./util/
# shellcheck source=util/util.sh
. ./util/util.sh

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
xcall cargo clippy --locked --all-targets -- -Dwarnings
plus_ok

plus_header "Now validiating fmt"
xcall cargo fmt --all --check --verbose
plus_ok

plus_header "Now validating build with various features"
xcargo build --no-default-features --features posix,save,sink
xcargo build --no-default-features --features default
xcargo build --no-default-features --features draw
xcargo build --no-default-features --features posix
xcargo build --no-default-features --features draw,posix
xcargo build --no-default-features --features save
xcargo build --no-default-features --features default,save
xcargo build --no-default-features --features draw,save
xcargo build --no-default-features --features posix,save
xcargo build --no-default-features --features draw,posix,save
xcargo build --no-default-features --features score
xcargo build --no-default-features --features default,score
xcargo build --no-default-features --features posix,score
xcargo build --no-default-features --features save,score
xcargo build --no-default-features --features default,save,score
xcargo build --no-default-features --features posix,save,score
xcargo build --no-default-features --features sink
xcargo build --no-default-features --features default,sink
xcargo build --no-default-features --features posix,sink
xcargo build --no-default-features --features save,sink
xcargo build --no-default-features --features default,save,sink
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
xcall ./util/bench-ryan.sh
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
