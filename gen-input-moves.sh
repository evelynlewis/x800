#!/bin/sh

# Check to see if we need a modifed variant of the command
if command -v gbase32 && command -v tr && command -v dd; then
    gbase32 /dev/urandom | tr '[:upper:]' '[:lower:]' | tr -dC asdw | dd of="$OUT" bs=8M count=1
    exit 0
elif command -v base32 && command -v tr && command -v dd; then
    base32 /dev/urandom | tr '[:upper:]' '[:lower:]' | tr -dC asdw | dd of="$OUT" bs=8M count=1
    exit 0
fi

exit 1
