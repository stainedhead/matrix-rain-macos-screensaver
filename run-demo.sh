#!/bin/bash
# Script to build and run the Matrix Rain test window

set -e

cd "$(dirname "$0")"

echo "Building Matrix Rain test window..."
cargo build --release --features windowed --bin matrix-rain-test

echo ""
echo "Starting Matrix Rain test window..."
echo "Press Cmd+Q or close the window to exit"
echo ""

./target/release/matrix-rain-test
