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

if [ -z "$1" ]; then
	OUT="/dev/stdout"
else
	OUT="${1}"
fi

BS="4K"
COUNT="1"

# Check to see if we need a modifed variant of the command
if command -v base32; then
	BASE32="base32"
elif command -v gbase32; then
	BASE32="gbase32"
fi

if command -v "${BASE32}" && command -v tr && command -v dd; then
	"${BASE32}" /dev/urandom |
		tr '[:upper:]' '[:lower:]' |
		tr -dC asdw |
		dd of="${OUT}" bs="${BS}" count="${COUNT}" &&
		exit 0
fi

exit 1
