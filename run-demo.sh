#!/bin/bash
# Script to run matrix-rain in a new terminal window

cd "$(dirname "$0")"

# Run for 30 seconds with defaults
./target/release/matrix-rain --duration 30

echo ""
echo "Matrix Rain demo finished!"
echo "Press any key to close this window..."
read -n 1
