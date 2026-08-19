# Antigravity Statusline Plugin

A lightweight, zero-dependency, compiled Rust binary that renders an ultra-fast terminal statusline with near-zero latency (< 2ms) specifically for the Antigravity CLI (`agy`).

## Features
- **Model Branding**: 24-bit Truecolor per model family (Claude orange, Gemini blue, OpenAI teal).
- **Fast Git Branch**: Directly reads `.git/HEAD` without spawning `git.exe`.
- **Adaptive Context Alerts**: Context window usage changes color dynamically (Blue -> Green -> Yellow -> Red).
- **Universal Quota**: Parses flat or bucketed quota objects from CLI stdin with countdown timers.

## Structure
```
antigravity-statusline/
├── plugin.json
├── README.md
├── bin/
│   └── statusline.exe
├── skills/
│   └── antigravity-statusline/
│       └── SKILL.md
└── src/
    ├── Cargo.toml
    └── src/
        └── main.rs
```
