#!/bin/bash
set -e

echo "Building macOS disk image (.dmg)..."

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
BUNDLE_PATH="macos-screensaver/build/MatrixRainSaver.saver"
DMG_OUTPUT="release/MatrixRainSaver-${VERSION}.dmg"
DMG_STAGING="release/dmg-staging"

# Verify bundle exists
if [ ! -d "$BUNDLE_PATH" ]; then
    echo "Error: Screensaver bundle not found at $BUNDLE_PATH"
    echo "Please run: cd macos-screensaver && ./build-screensaver.sh"
    exit 1
fi

# Create release directory
mkdir -p release

# Clean up any existing staging directory
rm -rf "$DMG_STAGING"
mkdir -p "$DMG_STAGING"

# Copy screensaver bundle to staging
echo "Copying screensaver bundle to staging area..."
cp -R "$BUNDLE_PATH" "$DMG_STAGING/"

# Create a symbolic link to the Screen Savers folder for easy installation
echo "Creating symbolic link to Screen Savers folder..."
ln -s "$HOME/Library/Screen Savers" "$DMG_STAGING/Install Here (Double-Click)"

# Create a README file with installation instructions
cat > "$DMG_STAGING/README.txt" << EOF
Matrix Rain Screensaver v${VERSION}

INSTALLATION:
1. Double-click "MatrixRainSaver.saver"
2. Click "Install" when prompted
3. Choose "Install for this user only" or "Install for all users"
4. Open System Settings > Screen Saver to configure

OR

Drag "MatrixRainSaver.saver" to "Install Here (Double-Click)"

For more information, visit:
https://github.com/stainedhead/matrix-rain-macos-screensaver
EOF

# Remove existing DMG if it exists
if [ -f "$DMG_OUTPUT" ]; then
    rm "$DMG_OUTPUT"
fi

# Create the DMG
echo "Creating disk image..."
hdiutil create \
    -volname "Matrix Rain v${VERSION}" \
    -srcfolder "$DMG_STAGING" \
    -ov \
    -format UDZO \
    "$DMG_OUTPUT"

# Clean up staging directory
rm -rf "$DMG_STAGING"

echo "DMG created: $DMG_OUTPUT"
echo ""
echo "The disk image contains:"
echo "  - MatrixRainSaver.saver (the screensaver bundle)"
echo "  - Install Here (Double-Click) (symbolic link to Screen Savers folder)"
echo "  - README.txt (installation instructions)"
