#!/bin/sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Start roger fuzzer engine non-interactively
#
# usage:
# ./util/fuzzm.sh
#

set -e
test -d ./fuzz
test -d ./util

# Start roger fuzzer engine non-interactively
./util/fuzzr.sh --no-default-features
