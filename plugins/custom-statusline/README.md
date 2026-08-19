# Custom Statusline Plugin

High-performance native Rust statusline for Antigravity & Claude Code CLI.

## Features
- **Model TrueColor Branding**: 24-bit TrueColor per model provider (Claude, Gemini, GPT, DeepSeek, Mistral, Llama).
- **Instant Git Branch**: Directly reads .git/HEAD and worktrees in <0.2ms without spawning external processes.
- **Adaptive Context Alerts**: Context window usage changes color dynamically (Blue >=75%, Green >=50%, Yellow >=25%, Red <25%).
- **Universal Quota Tracking**: Seamlessly parses flat or bucketed provider quota payloads with countdown timers.
- **Zero Runtime Dependencies**: Compiled native binary with LTO (Link-Time Optimization) and stripped symbols.
