# Publishing Proxmon

🎮 **Guide for distributing Proxmon to users**

## Option 1: Install from Git (Available Now!)

Users can install directly from GitHub without cloning:

```bash
cargo install --git https://github.com/0xgoz/proxmon
```

**Requirements:** Just Rust/Cargo installed
**Your setup:** Push your code to GitHub - done!

## Option 2: Publish to crates.io (Recommended)

This makes installation super easy:

```bash
cargo install proxmon
```

### Steps to Publish

1. **Create a crates.io account**
   - Go to https://crates.io
   - Sign in with GitHub

2. **Get your API token**
   ```bash
   cargo login
   ```
   - This will prompt you for your API token from https://crates.io/me

3. **Verify your package**
   ```bash
   cargo package --list
   ```
   This shows what files will be included.

4. **Dry run (test)**
   ```bash
   cargo publish --dry-run
   ```

5. **Publish!**
   ```bash
   cargo publish
   ```

### Updating Proxmon

When you release a new version:

1. Update `version` in `Cargo.toml`:
   ```toml
   version = "0.1.1"  # or 0.2.0, etc.
   ```

2. Commit the changes

3. Tag the release:
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```

4. Publish to crates.io:
   ```bash
   cargo publish
   ```

## Option 3: GitHub Releases (Pre-compiled Binaries)

For users who don't have Rust/Cargo installed.

### Manual Release

1. **Build for your platform:**
   ```bash
   cargo build --release
   ```
   Binary at: `./target/release/proxmon`

2. **Create a GitHub release:**
   - Go to your repo → Releases → "Create a new release"
   - Tag: `v0.1.0`
   - Upload `./target/release/proxmon`
   - Name it: `proxmon-v0.1.0-macos-arm64` (or appropriate platform)

3. **Users can download and run:**
   ```bash
   curl -L https://github.com/0xgoz/proxmon/releases/download/v0.1.0/proxmon-v0.1.0-macos-arm64 -o proxmon
   chmod +x proxmon
   sudo mv proxmon /usr/local/bin/
   ```

### Automated Releases (GitHub Actions)

I can create a GitHub Actions workflow to automatically:
- Build for macOS (Intel & ARM)
- Build for Linux (x86_64 & ARM)
- Build for Windows
- Create GitHub release with all binaries

Would you like me to set this up?

## Option 4: Homebrew (macOS Users)

Once you have GitHub releases, you can create a Homebrew formula:

```ruby
class Proxmon < Formula
  desc "Blazingly fast terminal UI for managing Proxmox VMs"
  homepage "https://github.com/0xgoz/proxmon"
  url "https://github.com/0xgoz/proxmon/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "YOUR_SHA256_HERE"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "proxmon", shell_output("#{bin}/proxmon --version")
  end
end
```

Users install with:
```bash
brew tap 0xgoz/proxmon
brew install proxmon
```

## Comparison

| Method | User Requirements | Your Effort | Update Process |
|--------|------------------|-------------|----------------|
| Git Install | Rust/Cargo | Push to GitHub | Git push |
| crates.io | Rust/Cargo | One-time setup | `cargo publish` |
| GitHub Releases | None | Manual or CI/CD | Upload binaries |
| Homebrew | Homebrew | Create formula | Update formula |

## Recommended Approach

**For Rust developers:** Publish to crates.io
**For everyone else:** GitHub Releases with pre-compiled binaries
**Best of both:** Do both! 🚀

## Current Status

✅ `Cargo.toml` is ready for crates.io
✅ README.md has installation instructions
✅ LICENSE is MIT
⏳ Waiting for first GitHub push
⏳ Waiting for crates.io publish

## Quick Start Publishing

1. **Push to GitHub:**
   ```bash
   git init
   git add .
   git commit -m "Initial commit - Proxmon v0.1.0"
   git remote add origin https://github.com/0xgoz/proxmon.git
   git push -u origin master
   ```

2. **Publish to crates.io:**
   ```bash
   cargo login
   cargo publish
   ```

3. **Done!** Users can now:
   ```bash
   cargo install proxmon
   ```

## Need Help?

- crates.io docs: https://doc.rust-lang.org/cargo/reference/publishing.html
- GitHub releases: https://docs.github.com/en/repositories/releasing-projects-on-github

---

🎮 **Gotta distribute 'em all!**

