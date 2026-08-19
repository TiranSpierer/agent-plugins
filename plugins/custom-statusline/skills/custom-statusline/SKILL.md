---
name: custom-statusline
description: Custom high-performance statusline written in Rust for Antigravity CLI. Displays active model with brand colors, workspace directory, Git branch, context window percentage, and API quota.
---

# Custom Statusline Plugin

This plugin provides an ultra-fast (<2ms), zero-dependency, native Rust statusline for Antigravity CLI and Claude Code.

## Features
- **Model TrueColor Branding**: Renders exact brand colors for Anthropic Claude (Orange), Google Gemini (Blue), OpenAI GPT/o-series (Teal), DeepSeek (Cyan), Mistral, and Meta Llama.
- **Fast Zero-Subprocess Git Detection**: Directly inspects `.git/HEAD`, worktrees, and submodules without spawning `git.exe`.
- **Dynamic Context Window Color Tiers**: Color-codes remaining context tokens (Blue >=75%, Green >=50%, Yellow >=25%, Red <25%).
- **Universal API Quota Parser**: Supports both flat quota objects and bucketed provider quotas with countdown reset timers.
- **Cross-Platform Native Binary**: Zero runtime dependencies; compiles directly to an optimized native binary on Windows, Linux, and macOS.

## Building from Source
To compile the statusline binary from source:

```bash
cd src
cargo build --release
```

The compiled binary will be placed at `target/release/statusline` (or `statusline.exe` on Windows).

## Activation & Configuration
Configure the statusline command in your `settings.json` (`~/.gemini/antigravity-cli/settings.json` or `~/.gemini/settings.json`):

```json
{
  "statusLine": {
    "type": "command",
    "command": "C:\\Users\\<USER>\\.gemini\\antigravity-cli\\plugins\\custom-statusline\\bin\\statusline.exe",
    "enabled": true
  }
}
```
