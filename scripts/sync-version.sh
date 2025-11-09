#!/bin/bash
set -e

# Extract version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

if [ -z "$VERSION" ]; then
    echo "Error: Could not extract version from Cargo.toml"
    exit 1
fi

echo "Syncing version $VERSION to all files..."

# Update Info.plist
if [ -f "macos-screensaver/MatrixRainSaver/Info.plist" ]; then
    sed -i.bak "s/<string>[0-9]*\.[0-9]*\.[0-9]*<\/string>/<string>$VERSION<\/string>/g" \
        macos-screensaver/MatrixRainSaver/Info.plist

    # Clean up backup
    rm -f macos-screensaver/MatrixRainSaver/Info.plist.bak

    echo "Updated Info.plist"
fi

echo "Version sync complete: $VERSION"
