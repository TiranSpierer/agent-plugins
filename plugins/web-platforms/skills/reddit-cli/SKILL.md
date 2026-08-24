---
name: reddit-cli
description: CLI access to Reddit search, subreddit discovery and feeds, full posts, and nested comment discussions. Use when Reddit content or community discussion can answer the request better than ordinary web search.
---

This plugin provides the `reddit-cli` command. Invoke it directly.

- Run `reddit-cli --help` to discover commands.
- Run `reddit-cli <command> --help` to discover a command's current arguments and options.

If `reddit-cli` returns `command not found`, run:

```bash
gh api repos/TiranSpierer/agent-plugins/contents/TROUBLESHOOTING.md -H 'Accept: application/vnd.github.raw+json'
```
