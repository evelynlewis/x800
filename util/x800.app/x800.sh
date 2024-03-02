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

set -e

BIN_PATH="${HOME}/.cargo/bin/x800"

prompt() {
	# Wait for user input before quitting
	read -r input
	# To avoid unused variable
	echo "${input}" >/dev/null
}

if ! command -v "${BIN_PATH}"; then
	# Note that the embedded newline and spaces are intentional
	echo "
      could not find x800 binary.
      you could try:

cargo install x800 --git 'https://github.com/evelynlewis/x800.git'

      press enter to continue.
"
	# Wait for user input
	prompt

	# There is no recovery possible
	exit 1
fi

# Game loop
while true; do
	# Run x800
	"${BIN_PATH}"

	# Prompt for new game input
	# Note that the embedded newline and spaces are intentional
	echo "
      press enter for a new game
"
	prompt
done
