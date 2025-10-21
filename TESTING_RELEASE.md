# Testing GitHub Release Workflow

This guide will help you test the GitHub Actions release workflow and deploy binaries to your Linux machine.

## What Was Created

✅ `.github/workflows/release.yml` - Automatic binary releases on tag push
✅ `.github/workflows/ci.yml` - Continuous integration testing

## Supported Platforms

The release workflow builds for:

- **macOS**
  - `x86_64-apple-darwin` (Intel Macs)
  - `aarch64-apple-darwin` (Apple Silicon M1/M2/M3)

- **Linux**
  - `x86_64-unknown-linux-gnu` (Standard Linux with glibc)
  - `x86_64-unknown-linux-musl` (Static binary, works everywhere)

- **Windows**
  - `x86_64-pc-windows-msvc` (64-bit Windows)

## How to Trigger a Release

### 1. Ensure code is pushed to GitHub

```bash
# If this is your first push
git add .
git commit -m "Add GitHub Actions workflows"
git push origin main

# Or if already pushed
git push
```

### 2. Create and push a tag

```bash
# Create a tag (use your actual version)
git tag v0.1.0

# Push the tag to GitHub
git push origin v0.1.0
```

**That's it!** GitHub Actions will automatically:
1. Create a new GitHub Release
2. Build binaries for all 5 platforms
3. Upload binaries with checksums
4. Takes ~10-15 minutes total

## How to Test on Your Linux Machine

### Option 1: Download from GitHub Release (After workflow completes)

```bash
# Download the binary
curl -L https://github.com/0xgoz/proxmon/releases/download/v0.1.0/proxmon-linux-x86_64.tar.gz -o proxmon.tar.gz

# Verify checksum (optional but recommended)
curl -L https://github.com/0xgoz/proxmon/releases/download/v0.1.0/proxmon-linux-x86_64.tar.gz.sha256 -o proxmon.tar.gz.sha256
sha256sum -c proxmon.tar.gz.sha256

# Extract
tar xzf proxmon.tar.gz

# Make executable
chmod +x proxmon

# Move to PATH (optional)
sudo mv proxmon /usr/local/bin/

# Test it!
proxmon
```

### Option 2: Use the MUSL version (works on any Linux)

The MUSL build is statically linked and works on ANY Linux distribution:

```bash
curl -L https://github.com/0xgoz/proxmon/releases/download/v0.1.0/proxmon-linux-x86_64-musl.tar.gz | tar xz
chmod +x proxmon
./proxmon
```

### Option 3: Build directly on Linux machine

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/0xgoz/proxmon
cd proxmon
cargo build --release

# Binary is at:
./target/release/proxmon
```

## Watching the Workflow

1. Go to: `https://github.com/0xgoz/proxmon/actions`
2. You'll see the "Release" workflow running
3. Click on it to see build progress for each platform
4. When complete, check: `https://github.com/0xgoz/proxmon/releases`

## Workflow Details

### Release Workflow (`release.yml`)
- **Triggers on:** Git tags matching `v*.*.*` (e.g., v0.1.0, v1.2.3)
- **Creates:** GitHub Release with binaries and checksums
- **Platforms:** macOS (Intel + ARM), Linux (glibc + musl), Windows
- **Duration:** ~10-15 minutes

### CI Workflow (`ci.yml`)
- **Triggers on:** Push to `main`/`develop` branches, or pull requests
- **Runs:** Tests, formatting check, clippy lints, documentation build
- **Platforms:** Ubuntu, macOS, Windows
- **Rust versions:** Stable and Beta
- **Duration:** ~5-8 minutes

## Quick Test Flow

Here's the fastest way to test everything:

```bash
# 1. Make sure everything is committed
git status
git add .
git commit -m "Ready for release"
git push origin main

# 2. Create and push tag
git tag v0.1.0
git push origin v0.1.0

# 3. Watch it build (opens browser)
open https://github.com/0xgoz/proxmon/actions

# 4. Wait ~10-15 minutes, then check releases
open https://github.com/0xgoz/proxmon/releases

# 5. On your Linux machine, download and test
ssh your-linux-machine
curl -L https://github.com/0xgoz/proxmon/releases/download/v0.1.0/proxmon-linux-x86_64.tar.gz | tar xz
chmod +x proxmon
./proxmon
```

## Troubleshooting

### "Actions are not enabled for this repository"
1. Go to repository Settings → Actions → General
2. Enable "Allow all actions and reusable workflows"

### Build fails on a specific platform
- Check the Actions logs for that platform
- Common issues:
  - Dependency problems (rare with Rust)
  - Platform-specific code issues
  - Network timeout (just re-run the job)

### Tag already exists
```bash
# Delete local tag
git tag -d v0.1.0

# Delete remote tag
git push origin :refs/tags/v0.1.0

# Create new tag
git tag v0.1.0
git push origin v0.1.0
```

### Want to test without creating a release?
The CI workflow runs on every push to `main`, so you can test builds without creating a release:

```bash
git push origin main
# Check: https://github.com/0xgoz/proxmon/actions
```

## Future Releases

For subsequent releases:

```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md
# 3. Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to v0.1.1"
git push origin main

# 4. Tag and release
git tag v0.1.1
git push origin v0.1.1

# Done! Workflow handles the rest.
```

## Binary Size

Approximate binary sizes:
- **Linux (glibc):** ~8-10 MB
- **Linux (musl):** ~10-12 MB (statically linked)
- **macOS:** ~6-8 MB
- **Windows:** ~7-9 MB

All binaries are stripped to reduce size.

## Verifying Downloads

Always verify checksums for security:

```bash
# Linux/macOS
curl -L https://github.com/0xgoz/proxmon/releases/download/v0.1.0/proxmon-linux-x86_64.tar.gz.sha256 | sha256sum -c

# Or manually
sha256sum proxmon-linux-x86_64.tar.gz
# Compare with .sha256 file
```

## Distribution Checklist

Once release is successful:

- [ ] Binaries available on GitHub Releases
- [ ] Tested on Linux machine
- [ ] Tested on macOS (if available)
- [ ] Publish to crates.io: `cargo publish`
- [ ] Announce on relevant channels
- [ ] Update README with installation instructions

---

🎮 **Happy releasing!**

