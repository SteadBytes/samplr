#! /usr/bin/env bash
# Quick and dirty test for --seed flag. Will be replaced asap.

set -euo pipefail

SEED=12345

diff <(seq 0 100 | cargo run -- --seed "$SEED") <(seq 0 100 | cargo run -- --seed "$SEED")
