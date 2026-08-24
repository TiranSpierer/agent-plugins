---
name: youtube-cli
description: CLI access to YouTube video search, video and channel metadata, channel and playlist videos, transcripts, and comments. Use when the request needs information from YouTube content.
---

This plugin provides the `youtube-cli` command. Invoke it directly.

- Run `youtube-cli --help` to discover resources.
- Run `youtube-cli <resource> --help` and `youtube-cli <resource> <command> --help` to discover commands and options.

If `youtube-cli` returns `command not found`, run:

```bash
gh api repos/TiranSpierer/agent-plugins/contents/TROUBLESHOOTING.md --jq .content | base64 -d
```
