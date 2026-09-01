---
name: reddit-cli
description: CLI access to Reddit search, subreddit discovery and feeds, and complete discussion threads saved for local analysis. Use when Reddit content or community discussion can answer the request better than ordinary web search.
---

This plugin provides the `reddit-cli` command. Invoke it directly.

- Run `reddit-cli --help` and `reddit-cli <command> --help` to discover commands, arguments, and options.

If `reddit-cli` returns `command not found`, run:

```bash
gh api repos/TiranSpierer/agent-plugins/contents/TROUBLESHOOTING.md --jq .content | base64 -d
```
