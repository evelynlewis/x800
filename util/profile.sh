#!/bin/sh
set -e

echo "Browser:"
if command -v "${BROWSER}"; then
    export BROWSER
elif command -v firefox; then
    export BROWSER=firefox
elif command -v /Applications/Firefox.app/Contents/MacOS/firefox; then
    export BROWSER=/Applications/Firefox.app/Contents/MacOS/firefox
else
    echo "\$BROWSER unset and failed to find Firefox"
    exit 1
fi
echo

set -x
cargo build --profile profile --no-default-features --features sink
samply record ./target/profile/ryan fuzz/corpus/roger/gold 10000
