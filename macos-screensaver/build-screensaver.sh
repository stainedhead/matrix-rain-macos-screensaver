#!/bin/bash

# Build script for Matrix Rain macOS Screensaver
# This script compiles the screensaver bundle using swiftc

set -e

echo "Building Matrix Rain Screensaver..."

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Directories
MACOS_DIR="$SCRIPT_DIR"
SAVER_DIR="$MACOS_DIR/MatrixRainSaver"
FRAMEWORKS_DIR="$MACOS_DIR/Frameworks"
BUILD_DIR="$MACOS_DIR/build"
BUNDLE_DIR="$BUILD_DIR/MatrixRainSaver.saver"
CONTENTS_DIR="$BUNDLE_DIR/Contents"
MACOS_OUT_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
FRAMEWORKS_OUT_DIR="$CONTENTS_DIR/Frameworks"

# Clean previous build
echo "Cleaning previous build..."
rm -rf "$BUILD_DIR"

# Create bundle structure
echo "Creating bundle structure..."
mkdir -p "$MACOS_OUT_DIR"
mkdir -p "$RESOURCES_DIR"
mkdir -p "$FRAMEWORKS_OUT_DIR"

# Copy Info.plist
echo "Copying Info.plist..."
cp "$SAVER_DIR/Info.plist" "$CONTENTS_DIR/"

# Copy Rust library
echo "Copying Rust library..."
cp "$FRAMEWORKS_DIR/libmatrix_rain_core.dylib" "$FRAMEWORKS_OUT_DIR/"

# Update library install name
echo "Updating library install name..."
install_name_tool -id "@rpath/libmatrix_rain_core.dylib" "$FRAMEWORKS_OUT_DIR/libmatrix_rain_core.dylib"

# Compile Swift files
echo "Compiling Swift files..."
swiftc \
    -target x86_64-apple-macos11.0 \
    -emit-executable \
    -o "$MACOS_OUT_DIR/MatrixRainSaver" \
    -module-name MatrixRainSaver \
    -import-objc-header "$SAVER_DIR/BridgingHeader.h" \
    -L "$FRAMEWORKS_OUT_DIR" \
    -l matrix_rain_core \
    -Xlinker -rpath -Xlinker @loader_path/../Frameworks \
    -framework ScreenSaver \
    -framework AppKit \
    -framework SwiftUI \
    "$SAVER_DIR/Preferences.swift" \
    "$SAVER_DIR/ConfigurationView.swift" \
    "$SAVER_DIR/MatrixRainView.swift"

echo "Build complete! Screensaver bundle created at:"
echo "$BUNDLE_DIR"
echo ""
echo "To install, copy to:"
echo "  ~/Library/Screen Savers/"
echo ""
echo "Or run:"
echo "  cp -r '$BUNDLE_DIR' ~/Library/Screen\\ Savers/"
