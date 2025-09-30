#!/usr/bin/env bash

set -e

# Usage: ./download_lib3mf.sh [version] [platform]
# Example: ./download_lib3mf.sh 2.4.1 Linux

VERSION="${1:-2.4.1}"
PLATFORM="${2:-Linux}"

REPO="3MFConsortium/lib3mf"
ZIP_URL="https://github.com/$REPO/releases/download/v$VERSION/lib3mf-$VERSION-$PLATFORM.zip"
ZIP_FILE="lib3mf.zip"
EXTRACT_DIR="lib3mf_release"

echo "Downloading lib3mf $VERSION for $PLATFORM..."
curl -L "$ZIP_URL" -o "$ZIP_FILE"

# Ensure the includes directory exists
mkdir -p includes

# Ensure the lib3mf directory exists
mkdir -p lib3mf

echo "Extracting archive..."
unzip -o "$ZIP_FILE" -d "$EXTRACT_DIR"

echo "Copying headers to includes/..."
cp -r "$EXTRACT_DIR/lib3mf-$VERSION-$PLATFORM/include/Bindings/C/"* includes/

echo "Copying Lib to lib3mf/..."
cp "$EXTRACT_DIR/lib3mf-$VERSION-$PLATFORM/lib/lib3mf."* lib3mf/ 2>/dev/null || true

echo "Copying binaries to lib3mf/..."
cp "$EXTRACT_DIR/lib3mf-$VERSION-$PLATFORM/bin/"* lib3mf/ 2>/dev/null || true

echo "Cleaning up..."
rm -rf "$ZIP_FILE" "$EXTRACT_DIR"

echo "Done!"