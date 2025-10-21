# Proxmon - Quick Setup Guide

🎮 **Gotta manage 'em all!** - A blazingly fast terminal UI for Proxmox

## You're Ready to Go! 🚀

Your Proxmon is fully built and ready to use.

## Running the Application

```bash
# Run the optimized release version
./target/release/proxmon

# Or install system-wide
cargo install --path .
proxmon
```

## Keyboard Controls

**Main View:**
- **↑/↓** or **j/k** - Navigate one at a time
- **PgUp/PgDn** - Jump 10 items
- **Home/End** or **g/G** - Jump to top/bottom
- **1-5** - Sort columns (Name/Type/Status/IP/Node)
- **a** - Add new Proxmox host
- **i** - Edit IP address (interactive!)
- **e** - Export to Ansible hosts format
- **r** - Refresh data from Proxmox
- **q** - Quit

**Setup View (Add Host):**
- **Tab/Shift+Tab** - Navigate fields
- **Space** - Toggle SSL checkbox
- **Enter** - Save host
- **Esc** - Cancel

**Edit IP View:**
- **Type** - Enter IP address
- **Enter** - Save and refresh
- **Esc** - Cancel

**Export View:**
- **c** or **y** - Copy to clipboard (one-click!)
- **Enter/Esc/q** - Close export view

## Configuration

Proxmon looks for config in the following locations (in order):
1. `$PROXMON_CONFIG` environment variable
2. `~/.config/proxmon/config.yml`
3. `./config.yml` (current directory)

### Adding IP Overrides (Interactive!)

If some VMs show "N/A" for their IP address, you can add them **directly in the TUI**:

1. Navigate to the host (↑/↓ or j/k)
2. Press **`i`** to edit the IP
3. Type the IP address (e.g., `10.1.2.100`)
4. Press **Enter** to save

The IP override is automatically saved to your config file!

**Alternative:** Edit config file directly:
```yaml
ip_overrides:
  - name: "dc01"        # Exact VM name from Proxmox
    ip: "10.1.2.100"    # Your static IP
```

## What Proxmon Does

1. **Connects to Proxmox**: Uses API tokens to authenticate
2. **Fetches VMs and LXC containers**: From all configured hosts
3. **Displays information**: Name, type, status, IP address, node
4. **Exports to Ansible**: Generate properly formatted inventory

## Tips

- Make sure your Proxmox hosts are reachable
- The QEMU Guest Agent should be running on VMs for IP detection
- Press 'r' to refresh if you start/stop VMs
- Press 'e' to see the Ansible hosts format
- Press 'a' to add new Proxmox hosts from the app

## Troubleshooting

If you can't connect:
1. Verify network connectivity to your Proxmox hosts
2. Check that API tokens are valid in Proxmox UI
3. Ensure token IDs match the format: `user@realm!tokenid`

## Next Steps

Consider adding:
- Start/stop VM functionality
- SSH integration
- Real-time status updates
- Resource monitoring

Enjoy Proxmon! 🎮
