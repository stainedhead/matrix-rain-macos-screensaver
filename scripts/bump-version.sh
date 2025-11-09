#!/bin/bash
set -e

BUMP_TYPE="${1:-patch}"

if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found"
    exit 1
fi

# Install cargo-bump if not present
if ! command -v cargo-bump &> /dev/null; then
    echo "Installing cargo-bump..."
    cargo install cargo-bump
fi

# Bump version in Cargo.toml
echo "Bumping $BUMP_TYPE version..."
cargo bump $BUMP_TYPE

# Sync to other files
./scripts/sync-version.sh

# Get new version
NEW_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

echo "Version bumped to: $NEW_VERSION"
echo "NEW_VERSION=$NEW_VERSION" >> $GITHUB_OUTPUT 2>/dev/null || true
