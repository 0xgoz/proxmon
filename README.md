# Proxmon

> 🎮 **Gotta manage 'em all!** - A blazingly fast terminal UI for Proxmox

A terminal user interface (TUI) application for managing and viewing Proxmox VMs and LXC containers, built with Rust and [Ratatui](https://ratatui.rs/).

## Features

- 🖥️  View all VMs and LXC containers across multiple Proxmox hosts
- 📊 Display host information including name, type, status, IP address, and node
- 🔐 Secure API token authentication
- 🎯 **Interactive onboarding** - Add Proxmox hosts directly from the TUI
- ✏️  **Inline IP editing** - Set IP overrides without editing config files
- 🔄 **Column sorting** - Sort by any column (name, type, status, IP, node)
- 📋 **One-click export** - Copy Ansible inventory to clipboard
- ➕ Support for manual hosts (e.g., Raspberry Pis, physical servers)
- ⚡ Fast and lightweight terminal interface

## Prerequisites

- Rust (1.70 or later)
- Access to Proxmox VE API
- API tokens configured on your Proxmox hosts

## Setting Up Proxmox API Tokens

Before using this tool, you need to create API tokens on your Proxmox hosts:

1. Log in to your Proxmox web interface
2. Navigate to **Datacenter** → **Permissions** → **API Tokens**
3. Click **Add** to create a new token
4. Set the following:
   - **User**: `root@pam` (or your preferred user)
   - **Token ID**: `proxmon` (or your preferred name)
   - **Privilege Separation**: Uncheck if you want full permissions
5. Click **Add** and **save the secret** - you won't be able to see it again!
6. The token format will be: `root@pam!proxmon`

Repeat this process for each Proxmox host you want to manage.

## Installation

### Option 1: Install from crates.io (Recommended)

If you have Rust/Cargo installed:

```bash
cargo install proxmon
```

### Option 2: Install from GitHub

```bash
cargo install --git https://github.com/0xgoz/proxmon
```

### Option 3: Download Pre-compiled Binary

Download the latest release for your platform from [GitHub Releases](https://github.com/0xgoz/proxmon/releases):

**macOS/Linux:**
```bash
# Download and extract (replace URL with latest release)
curl -L https://github.com/0xgoz/proxmon/releases/latest/download/proxmon-<PLATFORM>.tar.gz | tar xz

# Make executable and move to PATH
chmod +x proxmon
sudo mv proxmon /usr/local/bin/
```

**Windows:**
Download the `.zip`, extract, and add `proxmon.exe` to your PATH.

### Option 4: Build from Source

```bash
git clone https://github.com/0xgoz/proxmon
cd proxmon
cargo build --release

# Binary will be at: ./target/release/proxmon
./target/release/proxmon
```

### Adding Your First Proxmox Host

You have two options:

**Option 1: Interactive Setup (Recommended)**
1. Run `proxmon`
2. Press **`a`** to open the setup wizard
3. Fill in your Proxmox details:
   - Name (e.g., "vs01")
   - Host/IP (e.g., "10.1.2.1")
   - Port (default: 8006)
   - API Token ID (e.g., "root@pam!proxmon")
   - API Token Secret
   - Verify SSL (usually unchecked for self-signed certs)
4. Press **Enter** to save

Config is automatically saved to `~/.config/proxmon/config.yml`

**Option 2: Manual Configuration**
1. Create config directory: `mkdir -p ~/.config/proxmon`
2. Copy example: `cp config.example.yml ~/.config/proxmon/config.yml`
3. Edit with your details
4. Run `proxmon`

## Usage

### Keyboard Controls

**Main View:**
- **↑/↓** or **j/k**: Navigate one item at a time
- **Page Up/Down**: Jump 10 items at a time
- **Home** or **g**: Jump to top of list
- **End** or **G**: Jump to bottom of list
- **1-5**: Sort by column (1=Name, 2=Type, 3=Status, 4=IP, 5=Node)
  - Press same number again to reverse sort order
  - Visual indicator (↑/↓) shows active sort column
- **a**: Add new Proxmox host (opens setup wizard)
- **i**: Edit IP address for selected host (interactive!)
- **e**: Export current hosts to Ansible format
- **r**: Refresh data from Proxmox hosts
- **q**: Quit the application

**Setup View (Add Proxmox Host):**
- **Tab/Shift+Tab**: Navigate between fields
- **Type**: Enter values in text fields
- **Space**: Toggle SSL verification checkbox
- **Enter**: Save host (when on last field)
- **Esc**: Cancel and return to main view

**Edit IP View:**
- **Type**: Enter IP address
- **Enter**: Save IP override to config.yml
- **Esc**: Cancel without saving

**Export View:**
- **c** or **y**: Copy to clipboard
- **Enter/Esc/q**: Close export view

### Main View

The main view displays a table with all your hosts:

```
┌────────────────────────────────────────────────┐
│              Proxmox Host Manager              │
└────────────────────────────────────────────────┘
┌────────────────────────────────────────────────┐
│ Name        │ Type │ Status  │ IP         │... │
│ vm1         │ VM   │ running │ 10.1.2.10  │... │
│ lxc1        │ LXC  │ running │ 10.1.2.20  │... │
│ pi1         │ Phys │ unknown │ 10.1.2.50  │... │
└────────────────────────────────────────────────┘
```

### Export View

Press **e** to view the Ansible hosts file format. You can copy this output directly to your `/etc/ansible/hosts` file:

```
[Proxmox_VM]
vm1 ansible_host=10.1.2.10 ansible_user=gozy

[Proxmox_LXC]
lxc1 ansible_host=10.1.2.20 ansible_user=gozy

[Physical]
pi1 ansible_host=10.1.2.50 ansible_user=gozy

[all:vars]
ansible_python_interpreter=/usr/bin/python3
ansible_become=yes
ansible_become_method=sudo
```

## Configuration

Proxmon looks for config in the following locations (in order):
1. `$PROXMON_CONFIG` environment variable
2. `~/.config/proxmon/config.yml`
3. `./config.yml` (current directory)

The `config.yml` file has three main sections:

### 1. Proxmox Hosts

```yaml
proxmox_hosts:
  - name: vs01                              # Display name
    host: 10.1.2.1                          # IP or hostname
    port: 8006                              # Proxmox API port (default: 8006)
    api_token_id: "root@pam!rust-tui"       # API token ID
    api_token_secret: "your-secret"         # API token secret
    verify_ssl: false                       # SSL verification (use true in production)
```

### 2. Manual Hosts

Add any hosts that aren't in Proxmox:

```yaml
manual_hosts:
  - name: "pi1"
    ip: "10.1.2.50"
    type: "physical"
    ansible_user: "gozy"
```

### 3. IP Overrides (Interactive!)

For VMs/LXCs that don't report their IP addresses (no guest agent, stopped, etc.), you can set them **interactively in the TUI** or in the config file:

**Interactive Method (Recommended):**
1. Run the app and navigate to a host showing "N/A" for IP
2. Press **`i`** to enter edit mode
3. Type the IP address
4. Press **Enter** to save (automatically updates `config.yaml`)
5. Press **`r`** to refresh and see the new IP

**Config File Method:**
```yaml
ip_overrides:
  - name: "dc01"           # Exact name as shown in Proxmox
    ip: "10.1.2.100"
  - name: "truenas"
    ip: "10.1.2.15"
```

**When to use IP overrides:**
- VM doesn't have QEMU Guest Agent installed
- Container's IP isn't being detected correctly
- You want to use a specific IP (e.g., static IP) instead of auto-detected one
- VM is stopped but you still need the IP in your inventory

### 4. Ansible Defaults

```yaml
ansible_defaults:
  python_interpreter: "/usr/bin/python3"
  become: true
  become_method: "sudo"
```

## Troubleshooting

### "Failed to connect to Proxmox host"

- Verify the host IP and port are correct
- Check that the API token has the necessary permissions
- Ensure the Proxmox API is accessible from your network
- If using self-signed certificates, set `verify_ssl: false`

### "No IP addresses shown"

- Make sure QEMU Guest Agent is installed and running on your VMs
- For LXC containers, ensure they have network configured
- Some VMs may take a moment to report their IP after starting

### Building fails

- Ensure you have Rust 1.70 or later: `rustc --version`
- Try `cargo clean` and rebuild

## Future Roadmap

- [ ] Interactive VM/LXC management (start/stop/restart)
- [ ] SSH directly into hosts from the TUI
- [ ] Filter and search functionality
- [ ] Real-time status updates
- [ ] Resource usage monitoring (CPU, RAM, disk)
- [ ] Multiple Ansible inventory output formats
- [ ] Configuration validation on startup

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT License - See LICENSE file for details

## Acknowledgments

- Built with [Ratatui](https://ratatui.rs/) - A Rust library for building rich terminal user interfaces
- Uses [Crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal manipulation

