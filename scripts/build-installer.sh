#!/bin/bash
set -e

echo "Building macOS installer package (.pkg)..."

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
BUNDLE_PATH="macos-screensaver/build/MatrixRainSaver.saver"
PKG_OUTPUT="release/MatrixRainSaver-${VERSION}.pkg"

# Verify bundle exists
if [ ! -d "$BUNDLE_PATH" ]; then
    echo "Error: Screensaver bundle not found at $BUNDLE_PATH"
    echo "Please run: cd macos-screensaver && ./build-screensaver.sh"
    exit 1
fi

# Create release directory
mkdir -p release

# Build component package
echo "Creating component package..."
pkgbuild \
    --component "$BUNDLE_PATH" \
    --install-location "/Library/Screen Savers" \
    --version "$VERSION" \
    --identifier "com.matrixrain.screensaver" \
    "release/MatrixRainSaver-component-${VERSION}.pkg"

# Create distribution package (for signing and customization)
echo "Creating distribution package..."
productbuild \
    --package "release/MatrixRainSaver-component-${VERSION}.pkg" \
    --version "$VERSION" \
    "$PKG_OUTPUT"

# Clean up component package
rm "release/MatrixRainSaver-component-${VERSION}.pkg"

echo "Installer created: $PKG_OUTPUT"
echo ""
echo "To sign the package (requires Developer ID certificate):"
echo "  productsign --sign 'Developer ID Installer: Your Name' \\"
echo "    '$PKG_OUTPUT' \\"
echo "    'release/MatrixRainSaver-${VERSION}-signed.pkg'"
