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

# x800 gapless gameplay script
#
# This is a convenience script for x800 gameplay, as it provides the ability
# to play several games in a loop.
#
# usage:
# x800.sh
#
set -e

BIN="x800"
BIN_PATH=$(command -v ${BIN})

prompt() {
	# Wait for user input before quitting
	read -r INPUT
	# To avoid unused variable
	echo "${INPUT}" >/dev/null
}

if ! ${BIN_PATH}; then
	# Note that the embedded newline and spaces are intentional
	echo "     can't find the x800 program"
	echo "     maybe try this command:"
	echo
	echo "cargo install x800 --git 'https://github.com/evelynlewis/x800.git'"
	echo
	echo "     press enter to continue"
	# Wait for user input
	prompt
	# There is no recovery possible
	exit 1
fi

# Game loop
while true; do
	# Run x800
	${BIN_PATH}

	# Prompt for new game input
	# Note that the embedded newline and spaces are intentional
	echo
	echo "      press enter for a new game"
	echo
	# Wait for user input
	prompt
done
