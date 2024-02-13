#!/bin/sh

set -ex

touch /tmp/moves 
hyperfine --prepare 'etc/gen-moves.sh /tmp/moves' --warmup=256 --runs=1024 --input='/tmp/moves' -N 'target/release/x800'

