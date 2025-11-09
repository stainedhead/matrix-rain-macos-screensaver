#!/bin/bash
set -e

echo "Building universal binary for macOS..."

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Build for both architectures
echo "Building for x86_64-apple-darwin..."
cargo build --release --features ffi --target x86_64-apple-darwin

echo "Building for aarch64-apple-darwin..."
cargo build --release --features ffi --target aarch64-apple-darwin

# Create universal dylib
echo "Creating universal binary..."
mkdir -p target/universal-apple-darwin/release
lipo -create \
    target/x86_64-apple-darwin/release/libmatrix_rain_core.dylib \
    target/aarch64-apple-darwin/release/libmatrix_rain_core.dylib \
    -output target/universal-apple-darwin/release/libmatrix_rain_core.dylib

echo "Universal binary created at: target/universal-apple-darwin/release/libmatrix_rain_core.dylib"

# Verify
echo "Architectures in universal binary:"
lipo -info target/universal-apple-darwin/release/libmatrix_rain_core.dylib
