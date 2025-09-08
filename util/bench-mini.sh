#!/bin/sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Benchmark without draw code
#
# usage:
# ./util/bench-mini.sh
#

set -e
test -d ./util

./util/bench-shell.sh --no-default-features
