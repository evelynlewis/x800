# shellcheck shell=sh

# MIT License
#
# Copyright (c) 2024 Evelyn Lewis
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

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
        exit 1
    fi
}
