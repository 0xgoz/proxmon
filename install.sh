#!/bin/bash
# Proxmon Quick Install Script

set -e

echo "🎮 Proxmon Installer - Gotta manage 'em all!"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Please install from https://rustup.rs"
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"
echo ""

# Install Proxmon
echo "📦 Installing Proxmon..."
cargo install --path .

echo ""
echo "✅ Proxmon installed to ~/.cargo/bin/proxmon"
echo ""

# Check if cargo bin is in PATH
if ! echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
    echo "⚠️  Warning: ~/.cargo/bin is not in your PATH"
    echo ""
    echo "Add this to your shell profile (~/.zshrc or ~/.bashrc):"
    echo "    export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    echo ""
    echo "Then run: source ~/.zshrc  (or ~/.bashrc)"
    echo ""
fi

# Create config directory
echo "📁 Creating config directory..."
mkdir -p ~/.config/proxmon

if [ ! -f ~/.config/proxmon/config.yml ]; then
    echo "📄 Copying example config..."
    cp config.example.yml ~/.config/proxmon/config.yml
    echo ""
    echo "✅ Config created at: ~/.config/proxmon/config.yml"
    echo ""
    echo "🎯 Next steps:"
    echo "   1. Run: proxmon"
    echo "   2. Press 'a' to add your first Proxmox host"
    echo "   3. Fill in the details and press Enter"
    echo "   4. Start managing your infrastructure! 🚀"
else
    echo "✅ Config already exists at: ~/.config/proxmon/config.yml"
    echo ""
    echo "🎯 Ready to go! Run: proxmon"
fi

echo ""
echo "📖 Documentation:"
echo "   - README.md  - Full documentation"
echo "   - SETUP.md   - Quick setup guide"
echo "   - INSTALL.md - Installation details"
echo ""
echo "🎮 Enjoy Proxmon!"

