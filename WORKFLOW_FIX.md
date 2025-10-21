# GitHub Actions Workflow Fix

## Problem

The workflow was showing 9 deprecation warnings:
```
The `set-output` command is deprecated and will be disabled soon.
Please upgrade to using Environment Files.
```

## Root Cause

The old workflow used deprecated GitHub Actions:
- `actions/create-release@v1` (deprecated, unmaintained)
- `actions/upload-release-asset@v1` (deprecated, unmaintained)

These actions internally used the old `set-output` syntax that GitHub is phasing out.

## Solution

**Replaced with modern action:** `softprops/action-gh-release@v2`

This is the current, actively maintained action for creating releases. It:
- ✅ Uses modern GitHub Actions syntax (no deprecation warnings)
- ✅ Actively maintained and updated
- ✅ Simpler workflow (single job instead of two)
- ✅ Better error handling
- ✅ Uploads all assets in parallel

## What Changed

### Before (Old Workflow)
```yaml
jobs:
  create-release:  # Job 1: Create the release
    - uses: actions/create-release@v1  # DEPRECATED

  build-release:   # Job 2: Build and upload assets
    needs: create-release
    - uses: actions/upload-release-asset@v1  # DEPRECATED
```

**Issues:**
- 2 separate jobs (slower)
- Used deprecated actions
- 9 deprecation warnings
- More complex

### After (New Workflow)
```yaml
jobs:
  build-release:   # Single job: Build and release
    - uses: softprops/action-gh-release@v2  # MODERN, MAINTAINED
      with:
        files: |
          ${{ matrix.name }}
          ${{ matrix.name }}.sha256
```

**Benefits:**
- ✅ Single job (simpler, faster)
- ✅ Modern action (no warnings)
- ✅ Actively maintained
- ✅ Cleaner code

## Testing

After pushing this fix:

```bash
# Commit the fix
git add .github/workflows/release.yml
git commit -m "Fix: Update to modern GitHub Actions (remove deprecation warnings)"
git push origin main

# Test with a new tag
git tag v0.2.1
git push origin v0.2.1

# Check the Actions tab - no more warnings! ✨
```

## No More Warnings!

The workflow will now run cleanly without any deprecation warnings. GitHub Actions logs will be much cleaner and easier to read.

## Additional Improvements

While fixing the deprecation issue, I also improved the release notes:
- ✨ Mentions auto-config creation feature
- 🎯 Emphasizes MUSL build for Linux
- 📝 Clearer installation instructions
- 🚀 Added "First Run" section

## References

- [softprops/action-gh-release](https://github.com/softprops/action-gh-release) - The modern action we now use
- [GitHub: Deprecating set-output](https://github.blog/changelog/2022-10-11-github-actions-deprecating-save-state-and-set-output-commands/)
- [GitHub Actions: Environment Files](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#environment-files)

---

🎉 **No more ugly warnings!**

