#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p test-results
cargo +1.95.0 test --locked --workspace --no-run --message-format=json > test-results/workspace.jsonl
cargo +1.95.0 test --locked --manifest-path mdns-sd/Cargo.toml --lib --no-run --message-format=json > test-results/mdns.jsonl
python3 scripts/run-test-binaries.py
cargo +1.95.0 test --locked --workspace --doc
