---
name: reddit-cli
description: CLI access to Reddit search, subreddit discovery and feeds, and complete discussion threads saved for local analysis. Use when Reddit content or community discussion can answer the request better than ordinary web search.
---

This plugin provides the `reddit-cli` command. Invoke it directly.

- Run `reddit-cli --help` to discover resources.
- Run `reddit-cli <command> --help` and `reddit-cli <command> <subcommand> --help` to discover current arguments, options, and generated files.
- Thread bodies and comments, plus subreddit sidebars and rules, are saved under the OS temporary directory; use the returned paths for detailed analysis.

If `reddit-cli` returns `command not found`, run:

```bash
gh api repos/TiranSpierer/agent-plugins/contents/TROUBLESHOOTING.md --jq .content | base64 -d
```
