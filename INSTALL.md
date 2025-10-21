# Installing Proxmon

🎮 **Gotta manage 'em all!** - A blazingly fast terminal UI for Proxmox

## Quick Install

```bash
# Clone the repository
git clone <repository-url>
cd proxmon

# Install system-wide
cargo install --path .

# Make sure ~/.cargo/bin is in your PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Add to your shell profile for persistence (~/.zshrc or ~/.bashrc)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
```

## First Run Setup

```bash
# Run proxmon
proxmon

# The app will auto-create config at ~/.config/proxmon/config.yml
# Press 'a' to add your first Proxmox host interactively
```

## Manual Configuration

If you prefer to configure manually:

```bash
# Create config directory
mkdir -p ~/.config/proxmon

# Copy and edit the example config
cp config.example.yml ~/.config/proxmon/config.yml
nano ~/.config/proxmon/config.yml
```

## Custom Config Location

You can override the config location with an environment variable:

```bash
export PROXMON_CONFIG=/path/to/your/config.yml
proxmon
```

## Uninstall

```bash
cargo uninstall proxmon
rm -rf ~/.config/proxmon
```

## Building from Source

```bash
# Build release binary (no install)
cargo build --release

# Binary will be at:
./target/release/proxmon
```

## Updating

```bash
# Pull latest changes
git pull

# Reinstall
cargo install --path . --force
```

## System Requirements

- **OS**: macOS, Linux, or Windows
- **Rust**: 1.70+ (for building)
- **Network**: Access to Proxmox API (typically port 8006)
- **Terminal**: Any modern terminal emulator

## Verifying Installation

```bash
# Check if installed
which proxmon
# Output: /Users/alex/.cargo/bin/proxmon

# View help
proxmon --help

# Run the app
proxmon
```

## Troubleshooting

### "proxmon: command not found"
Make sure `~/.cargo/bin` is in your PATH:
```bash
echo $PATH | grep cargo
```

If not found, add to your shell profile:
```bash
# For zsh (macOS default)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# For bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Config file not found
Proxmon looks for config in this order:
1. `$PROXMON_CONFIG` (if set)
2. `~/.config/proxmon/config.yml`
3. `./config.yml` (current directory)

Create the directory and run setup:
```bash
mkdir -p ~/.config/proxmon
proxmon  # Press 'a' to add hosts interactively
```

### Can't connect to Proxmox
1. Verify network connectivity: `ping your-proxmox-host`
2. Check API tokens in Proxmox web UI
3. Ensure token format is correct: `root@pam!proxmon`
4. Try disabling SSL verification in config (for self-signed certs)

## Next Steps

Once installed, check out:
- [README.md](README.md) - Full documentation
- [SETUP.md](SETUP.md) - Quick setup guide

Enjoy! 🚀

