#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

echo "Downloading data files..."
curl -sSL "https://where.durableobjects.live/api/v3/data.json" | jq '.' > where.durableobjects.live.json
curl -sSL "https://www.cloudflarestatus.com/api/v2/components.json" | jq '.' > components.json

echo "Generating Rust code..."
python3 codegen.py

echo "Building and testing..."
cargo build
cargo test

echo "Done!"
