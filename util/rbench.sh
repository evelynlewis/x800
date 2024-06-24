#!/bin/sh
set -e

test -d ./fuzz/corpus/roger
test -d ./util
# shellcheck source=common.sh
. ./util/common.sh

LOOPS=10000

test_ryan() {
    time ./target/release/ryan ./fuzz/corpus/roger/gold "${LOOPS}"
}

plus_header "Building the ryan driver in default mode"
cargo build --release --bin ryan
test_ryan > /dev/null
plus_ok

plus_header "Building the ryan driver in sink mode"
cargo build --release --features=sink --bin ryan
test_ryan
plus_ok

plus_header "Building the ryan driver in mini mode"
cargo build --release --features=mini --bin ryan
test_ryan
plus_ok
