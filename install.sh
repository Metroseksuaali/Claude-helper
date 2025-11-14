#!/bin/bash
set -e

# Claude Helper Installation Script

echo "=================================="
echo "Claude Helper Installation"
echo "=================================="
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM=linux;;
    Darwin*)    PLATFORM=macos;;
    MINGW*|MSYS*|CYGWIN*)    PLATFORM=windows;;
    *)          PLATFORM="UNKNOWN"
esac

echo "Detected platform: ${PLATFORM}"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo ""
    echo "⚠️  Rust is not installed."
    echo "Would you like to install Rust now? (y/n)"
    read -r install_rust

    if [ "$install_rust" = "y" ] || [ "$install_rust" = "Y" ]; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo "Please install Rust from https://rustup.rs/ and try again."
        exit 1
    fi
fi

echo ""
echo "Building Claude Helper from source..."
echo "This may take a few minutes..."
echo ""

# Clone repository (if not already in it)
if [ ! -f "Cargo.toml" ]; then
    echo "Cloning repository..."
    git clone https://github.com/Metroseksuaali/Claude-helper.git
    cd Claude-helper
fi

# Build release binary
cargo build --release

# Install binary
if [ "$PLATFORM" = "windows" ]; then
    INSTALL_DIR="$HOME/.cargo/bin"
else
    INSTALL_DIR="/usr/local/bin"
fi

echo ""
echo "Installing to ${INSTALL_DIR}..."

if [ "$PLATFORM" = "windows" ]; then
    cp target/release/claude-helper.exe "${INSTALL_DIR}/"
    echo "✓ Installed to ${INSTALL_DIR}/claude-helper.exe"
else
    if [ -w "${INSTALL_DIR}" ]; then
        cp target/release/claude-helper "${INSTALL_DIR}/"
    else
        echo "Need sudo access to install to ${INSTALL_DIR}"
        sudo cp target/release/claude-helper "${INSTALL_DIR}/"
    fi

    echo "✓ Installed to ${INSTALL_DIR}/claude-helper"
fi

# Create config directory
CONFIG_DIR="$HOME/.config/claude-helper"
mkdir -p "${CONFIG_DIR}"

echo ""
echo "=================================="
echo "✓ Installation Complete!"
echo "=================================="
echo ""
echo "Next steps:"
echo ""
echo "1. Configure authentication:"
echo "   claude-helper config set-api-key"
echo ""
echo "2. Try it out:"
echo "   claude-helper status          # Check token usage"
echo "   claude-helper tui             # Open interactive dashboard"
echo "   claude-helper run \"task\"      # Run with Master Coder"
echo ""
echo "3. Optional: Install status line for Claude Code:"
echo "   claude-helper install-statusline"
echo ""
echo "For help:"
echo "   claude-helper --help"
echo ""
echo "Documentation: https://github.com/Metroseksuaali/Claude-helper"
echo ""
