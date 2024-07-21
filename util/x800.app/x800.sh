#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

# x800 gapless gameplay script
#
# This is a convenience script for x800 gameplay, as it provides the ability
# to play several games in a loop.
#
# usage:
# x800.sh
#
set -e

BIN='x800'
prompt() {
	# Wait for user input before quitting
	read -r _
}

if ! command -v "${BIN}"; then
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
	${BIN}

	# Prompt for new game input
	# Note that the embedded newline and spaces are intentional
	echo
	echo "     press enter for a new game"
	echo
	# Wait for user input
	prompt
done
