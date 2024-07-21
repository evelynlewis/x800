# shellcheck shell=sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Fuzzer environment configuration
# This file should be sourced not run
#
# usage:
# . common.sh
#

plus_line() {
    echo ================================================================================
}

plus_header() {
    plus_line
    echo "$*"
    echo
}

plus_ok() {
    echo "OK"
    plus_line
    echo
}
