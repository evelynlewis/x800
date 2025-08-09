# shellcheck shell=sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

# This file should be sourced not run
#
# usage:
# . compat.sh
#

# Check without argument whether
# the first argument given is executable
exists() {
    command -v "$1" >/dev/null
}

# Validated a provided executable name, with preference for GNU coreutils.
# A GNU version of a tool 'TOOL' is often named 'gTOOL'. Prefer any such explicitly GNU tool.
# The fallback is any valid executable of the provided name.
compat() {
    if exists "g$1"; then
        echo "g$1"
    elif exists "$1"; then
        echo "$1"
    else
        echo "Tool not found: $1" > /dev/stderr
        exit 1
    fi
}
