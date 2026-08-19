---
name: custom-statusline
description: Custom high-performance statusline written in Rust for Antigravity CLI. Displays active model with brand colors, workspace directory, Git branch, context window percentage, and API quota.
---

# Custom Statusline Plugin

This plugin provides an ultra-fast, native Rust statusline for the Antigravity CLI.

## Location
- Executable: `~/.gemini/antigravity-cli/plugins/custom-statusline/bin/statusline.exe`
- Rust Source: `~/.gemini/antigravity-cli/plugins/custom-statusline/src/`

## Building from Source
To recompile the binary after editing `src/src/main.rs`:

```bash
cd ~/.gemini/antigravity-cli/plugins/custom-statusline/src
cargo build --release
cp target/release/statusline.exe ../bin/statusline.exe
```

## Configuration
In `~/.gemini/antigravity-cli/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "C:\\Users\\tspie\\.gemini\\antigravity-cli\\plugins\\custom-statusline\\bin\\statusline.exe",
    "enabled": true
  }
}
```
