#!/bin/sh

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

#
# x800 valid move generator script
# Writes moves to stdout or a provided file
#
# usage:
# benchgen.sh [OUTPUT_FILE]
#

BS="8K"
COUNT="1"

# Use an output file if provided
if [ -z "$1" ]; then
	OUT="/dev/stdout"
else
	OUT="$1"
fi

# shellcheck source=compat.sh
. ./util/compat.sh
# Setup tools
BASE32=$(compat base32)
TR=$(compat tr)
DD=$(compat dd)

${BASE32} </dev/urandom |
	${TR} -dC 'ASDW' |
	${DD} of="${OUT}" bs="${BS}" count="${COUNT}" 2>/dev/null &&
	exit 0

exit 1
