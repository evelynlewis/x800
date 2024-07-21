#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Start roger fuzzer in mini mode`
#
# usage:
# ./util/min.sh
#

set -e
test -d ./fuzz
test -d ./util

# Call fuzzer in mini mode
./util/fuzzr.sh --features mini
