---
name: antigravity-statusline
description: Custom high-performance statusline written in Rust for Antigravity CLI. Displays active model with brand colors, workspace directory, Git branch, context window percentage, and API quota.
---

# Antigravity Statusline Plugin

This plugin provides an ultra-fast, native Rust statusline for the Antigravity CLI.

## Features
- **Model Branding**: 24-bit Truecolor per model family (Claude orange, Gemini blue, OpenAI teal).
- **Fast Git Branch**: Directly reads `.git/HEAD` without spawning `git.exe`.
- **Adaptive Context Alerts**: Context window usage changes color dynamically (Blue -> Green -> Yellow -> Red).
- **Universal Quota**: Parses flat or bucketed quota objects from CLI stdin with countdown timers.

## Building from Source
```bash
cd src
cargo build --release
```

## Configuration
In `~/.gemini/antigravity-cli/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "C:\\Users\\<USER>\\.gemini\\antigravity-cli\\plugins\\antigravity-statusline\\bin\\statusline.exe",
    "enabled": true
  }
}
```
