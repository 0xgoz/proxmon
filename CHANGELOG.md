# 🎮 Proxmon - Rebranding Complete!

## What Changed

### Name & Branding
- **Old**: `rust-proxmox` / `proxmox-tui`
- **New**: **Proxmon** - "Gotta manage 'em all!" 🎮

### Binary & Package
- Binary: `proxmox-tui` → **`proxmon`**
- Cargo package: `proxmox-tui` → **`proxmon`**

### Configuration
- **Old location**: `./config.yaml` (current directory only)
- **New locations** (checked in order):
  1. `$PROXMON_CONFIG` environment variable
  2. **`~/.config/proxmon/config.yml`** (recommended)
  3. `./config.yml` (fallback)

- **File format**: `.yaml` → **`.yml`**
- **Env var**: `PROXMOX_TUI_CONFIG` → **`PROXMON_CONFIG`**

### UI Changes
- Title: "Proxmox Host Manager" → **"Proxmon - Gotta manage 'em all!"**
- API token examples: `root@pam!rust-tui` → `root@pam!proxmon`

### Files Updated
✅ `Cargo.toml` - Package name, version, metadata
✅ `src/main.rs` - Config paths, env vars, error messages
✅ `src/ui.rs` - Title text
✅ `README.md` - All references, installation instructions
✅ `SETUP.md` - Complete rewrite with new branding
✅ `.gitignore` - Added `config.yml`
✅ `config.example.yaml` → `config.example.yml`
✅ Created `INSTALL.md` - Comprehensive installation guide

### Installation
```bash
# System-wide installation
cargo install --path .

# Your config is at:
~/.config/proxmon/config.yml

# Run from anywhere:
proxmon
```

### Migration for Existing Users

If you have an existing setup:

```bash
# Create new config directory
mkdir -p ~/.config/proxmon

# Copy your existing config
cp config.yaml ~/.config/proxmon/config.yml

# Install the new version
cargo install --path . --force

# Add to PATH if needed
export PATH="$HOME/.cargo/bin:$PATH"

# Run it!
proxmon
```

## Features Summary

✨ **Interactive Onboarding** - Add Proxmox hosts without editing files
✨ **Live IP Editing** - Update IP addresses directly in the TUI
✨ **Column Sorting** - Sort by any column (Name, Type, Status, IP, Node)
✨ **Fast Navigation** - Page Up/Down, Home/End, Vim keybindings
✨ **Ansible Export** - One-click export to clipboard
✨ **Manual Hosts** - Add non-Proxmox devices (like Raspberry Pis)
✨ **IP Overrides** - Manually set IPs for hosts without guest agents
✨ **Multi-Host Support** - Manage multiple Proxmox clusters

## Quick Reference

### Running Proxmon
```bash
proxmon                          # Use default config locations
PROXMON_CONFIG=~/custom.yml proxmon  # Custom config
```

### Keyboard Shortcuts
- **↑/↓** or **j/k** - Navigate
- **1-5** - Sort columns
- **a** - Add Proxmox host
- **i** - Edit IP address
- **e** - Export to Ansible
- **r** - Refresh from Proxmox
- **q** - Quit

### Config Location
Priority order:
1. `$PROXMON_CONFIG`
2. `~/.config/proxmon/config.yml` ⭐ **Recommended**
3. `./config.yml`

## What's Next?

Future ideas:
- Start/stop VMs from the TUI
- Real-time resource monitoring
- SSH integration
- Backup management
- Snapshot operations

---

**Proxmon v0.1.0** - Built with Rust 🦀 and Ratatui
🎮 **Gotta manage 'em all!**

