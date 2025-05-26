#!/bin/sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Benchmark without draw code
#
# usage:
# ./util/benchm.sh
#

set -e
test -d ./util

./util/benchs.sh --no-default-features
