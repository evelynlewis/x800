#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Multi-process fuzzing
#
# usage:
# ./util/fuzza.sh
#
set -e

test -d ./util

command -v "./util/fuzzm.sh"
command -v "./util/fuzzr.sh"

./util/fuzzm.sh
./util/fuzzr.sh
