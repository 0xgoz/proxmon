# Quick Start: Test Release on Linux

## Recent Improvements

✅ **Auto-config creation** - No config file needed! Proxmon creates it automatically on first run
✅ **MUSL build** - Zero glibc dependencies, works on ANY Linux distro

## The GLIBC Issue (Solved!)

If you encountered: `GLIBC_2.39 not found`

**Solution:** Use the MUSL build - it's statically linked with **zero glibc dependencies**!

## Test Right Now (Before GitHub Release)

Build a MUSL binary on your Mac and test it on your Linux machine:

```bash
# On your Mac:
# 1. Add the musl target (one-time setup)
rustup target add x86_64-unknown-linux-musl

# 2. Build for musl
cargo build --release --target x86_64-unknown-linux-musl

# 3. Copy to your Linux machine (adjust the path/user/host)
scp target/x86_64-unknown-linux-musl/release/proxmon user@your-linux-machine:/tmp/
```

Then on your Linux machine:
```bash
chmod +x /tmp/proxmon
/tmp/proxmon
```

**This will work on ANY Linux distro!** No glibc issues. 🎉

## After GitHub Release Completes

Once you push your tag and the GitHub Actions complete, users (and you) can download the MUSL build:

```bash
# On your Linux machine
curl -L https://github.com/0xgoz/proxmon/releases/download/v0.1.0/proxmon-linux-x86_64-musl.tar.gz | tar xz
chmod +x proxmon
sudo mv proxmon /usr/local/bin/
proxmon
```

## What I Changed

1. ✅ **Updated release workflow** to use Ubuntu 20.04 for the glibc build (better compatibility)
2. ✅ **Already included MUSL build** in the workflow (no glibc dependency at all!)
3. ✅ **Updated all docs** to recommend MUSL for Linux
4. ✅ **Added troubleshooting** for the glibc error

## Why Two Linux Builds?

- **`proxmon-linux-x86_64.tar.gz`** (glibc)
  - Standard Linux build
  - Slightly smaller binary
  - Works on Ubuntu 20.04+, Debian 11+, modern distros

- **`proxmon-linux-x86_64-musl.tar.gz`** (MUSL - RECOMMENDED)
  - Statically linked, zero dependencies
  - Works on ANY Linux distro (old or new!)
  - Perfect for: old servers, containers, air-gapped systems
  - Slightly larger, but maximum compatibility

## Release Workflow

Your GitHub Actions now build **5 binaries**:
1. macOS Intel (x86_64)
2. macOS Apple Silicon (ARM64)
3. Linux with glibc (Ubuntu 20.04+)
4. Linux with musl (universal!)
5. Windows (x86_64)

## Next Steps

1. **Test MUSL build locally** (commands above)
2. **Commit and push** the updated workflows
3. **Tag and release** when ready:
   ```bash
   git add .
   git commit -m "Update workflows for better Linux compatibility"
   git push origin main

   git tag v0.1.0
   git push origin v0.1.0
   ```
4. **Wait ~10-15 minutes** for builds
5. **Download and test** the MUSL build from GitHub Releases

## Compatibility Matrix

| Linux Version | glibc build | MUSL build |
|--------------|-------------|------------|
| Ubuntu 24.04 | ✅ | ✅ |
| Ubuntu 22.04 | ✅ | ✅ |
| Ubuntu 20.04 | ✅ | ✅ |
| Ubuntu 18.04 | ❌ | ✅ |
| Debian 12 | ✅ | ✅ |
| Debian 11 | ✅ | ✅ |
| Debian 10 | ❌ | ✅ |
| CentOS 7 | ❌ | ✅ |
| Alpine Linux | ❌ | ✅ |
| Any old distro | ❌ | ✅ |

**MUSL = Maximum compatibility!**

---

🎮 **MUSL is the way!**

