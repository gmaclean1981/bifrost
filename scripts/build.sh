#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo +1.95.0 build --locked --release
