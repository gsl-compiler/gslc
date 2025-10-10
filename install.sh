#!/bin/bash
# GSLC Installation Script
# Usage: curl -sSf https://raw.githubusercontent.com/politikl/gslc/main/install.sh | sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🚀 Installing GSLC - Geometry Shorthand Language Compiler"
echo ""

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        PLATFORM="linux"
        BINARY="gslc"
        ;;
    Darwin*)
        PLATFORM="macos"
        BINARY="gslc"
        if [ "$ARCH" = "arm64" ]; then
            ARCH_SUFFIX="arm64"
        else
            ARCH_SUFFIX="x86_64"
        fi
        ;;
    MINGW*|MSYS*|CYGWIN*)
        PLATFORM="windows"
        BINARY="gslc.exe"
        ARCH_SUFFIX="x86_64"
        ;;
    *)
        echo "${RED}❌ Unsupported operating system: $OS${NC}"
        exit 1
        ;;
esac

if [ "$PLATFORM" = "linux" ]; then
    ARCH_SUFFIX="x86_64"
fi

# Get latest release version
echo "📦 Fetching latest release..."
LATEST_RELEASE=$(curl -s https://api.github.com/repos/politikl/gslc/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "${RED}❌ Failed to fetch latest release${NC}"
    exit 1
fi

echo "   Latest version: ${GREEN}$LATEST_RELEASE${NC}"

# Download URL
if [ "$PLATFORM" = "windows" ]; then
    DOWNLOAD_FILE="gslc-${PLATFORM}-${ARCH_SUFFIX}.zip"
else
    DOWNLOAD_FILE="gslc-${PLATFORM}-${ARCH_SUFFIX}.tar.gz"
fi

DOWNLOAD_URL="https://github.com/politikl/gslc/releases/download/${LATEST_RELEASE}/${DOWNLOAD_FILE}"

echo "⬇️  Downloading GSLC..."
echo "   URL: $DOWNLOAD_URL"

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download
if ! curl -L -o "$DOWNLOAD_FILE" "$DOWNLOAD_URL"; then
    echo "${RED}❌ Download failed${NC}"
    rm -rf "$TMP_DIR"
    exit 1
fi

echo "📂 Extracting..."
if [ "$PLATFORM" = "windows" ]; then
    unzip -q "$DOWNLOAD_FILE"
else
    tar -xzf "$DOWNLOAD_FILE"
fi

# Install
echo "💾 Installing..."
INSTALL_DIR="$HOME/.local/bin"

if [ "$PLATFORM" = "windows" ]; then
    INSTALL_DIR="$HOME/bin"
fi

mkdir -p "$INSTALL_DIR"

if ! mv "$BINARY" "$INSTALL_DIR/"; then
    echo "${YELLOW}⚠️  Failed to move to $INSTALL_DIR, trying sudo...${NC}"
    sudo mv "$BINARY" /usr/local/bin/
    INSTALL_DIR="/usr/local/bin"
fi

chmod +x "$INSTALL_DIR/$BINARY"

# Cleanup
cd -
rm -rf "$TMP_DIR"

echo ""
echo "${GREEN}✅ GSLC installed successfully!${NC}"
echo ""
echo "📍 Installed to: $INSTALL_DIR/$BINARY"
echo ""

# Check if in PATH
if echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "${GREEN}✓${NC} $INSTALL_DIR is in your PATH"
else
    echo "${YELLOW}⚠️  $INSTALL_DIR is not in your PATH${NC}"
    echo ""
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo ""
echo "🎉 Try it out:"
echo "   gslc '\\\\P:A/P:B/S:AB\\\\'"
echo ""
echo "📚 Documentation: https://tinyurl.com/geoshorthand"
