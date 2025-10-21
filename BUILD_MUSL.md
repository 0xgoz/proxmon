# Building MUSL Binary for Linux

This guide shows you how to build a universal Linux binary on your Mac that will work on **any** Linux distribution.

## Why MUSL?

- ✅ **Statically linked** - No dependencies required
- ✅ **Universal** - Works on any Linux distro (Ubuntu, Debian, CentOS, Alpine, etc.)
- ✅ **Any age** - Works on old and new systems alike
- ✅ **Containers** - Perfect for Docker/Podman
- ❌ **Slightly larger** - But worth it for compatibility

## Quick Build (On Your Mac)

```bash
# 1. Add the MUSL target (one-time setup)
rustup target add x86_64-unknown-linux-musl

# 2. Build for MUSL
cd /Users/alex/projects/personal/rust-proxmox
cargo build --release --target x86_64-unknown-linux-musl

# 3. Binary is ready!
ls -lh target/x86_64-unknown-linux-musl/release/proxmon
```

## Transfer to Linux Machine

```bash
# Copy to your Linux machine
scp target/x86_64-unknown-linux-musl/release/proxmon user@your-linux-machine:/tmp/

# SSH to Linux machine and test
ssh user@your-linux-machine
chmod +x /tmp/proxmon
/tmp/proxmon

# App will auto-create config at ~/.config/proxmon/config.yml
# Press 'a' to add your first Proxmox host!
```

## Install on Linux Machine

```bash
# Move to system location
sudo mv /tmp/proxmon /usr/local/bin/

# Now you can run from anywhere
proxmon
```

## Troubleshooting

### "rustup: command not found"

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### "error: can't find crate for `core`"

Add the MUSL target:
```bash
rustup target add x86_64-unknown-linux-musl
```

### Build fails on macOS

Make sure you have Xcode command line tools:
```bash
xcode-select --install
```

## Verify It Works

On your Linux machine:

```bash
# Check the binary type (should show "statically linked")
file /tmp/proxmon
# Output: proxmon: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, BuildID[sha1]=..., stripped

# Check dependencies (should show "not a dynamic executable")
ldd /tmp/proxmon
# Output: not a dynamic executable

# Run it!
/tmp/proxmon
# Should show: "✨ Created new config at: /home/user/.config/proxmon/config.yml"
```

## GitHub Actions Build

When you push a tag, GitHub Actions automatically builds the MUSL version for you:

```bash
git tag v0.1.0
git push origin v0.1.0

# Wait ~10-15 minutes, then download:
curl -L https://github.com/0xgoz/proxmon/releases/download/v0.1.0/proxmon-linux-x86_64-musl.tar.gz | tar xz
chmod +x proxmon
./proxmon
```

## Comparison: glibc vs MUSL

| Feature | glibc build | MUSL build |
|---------|-------------|------------|
| Works on old Linux | ❌ No | ✅ Yes |
| Works on new Linux | ✅ Yes | ✅ Yes |
| Works in containers | ⚠️ Maybe | ✅ Always |
| Binary size | Smaller | Slightly larger |
| Dependencies | glibc 2.31+ | None! |
| Recommended | For modern distros | **For everyone** |

## When to Use Each

**Use glibc build (`proxmon-linux-x86_64.tar.gz`) if:**
- You're on Ubuntu 20.04 or newer
- You're on Debian 11 or newer
- You want a slightly smaller binary
- You know your glibc version is recent enough

**Use MUSL build (`proxmon-linux-x86_64-musl.tar.gz`) if:**
- ✅ You want maximum compatibility (recommended!)
- ✅ You're on an older Linux distro
- ✅ You're deploying to containers
- ✅ You're not sure what Linux version you have
- ✅ You encounter "GLIBC version not found" errors

**TL;DR: Use MUSL for everything Linux. It just works!** 🎯

---

🐧 **MUSL = Maximum Universal Static Linux!**

