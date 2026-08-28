---
name: zap-cli
description: CLI access to Zap Israel product search, price comparisons, store offers, delivered prices, specifications, reviews, price history, categories, filters, similar products, and store information. Use for questions that need current cross-store Israeli shopping data.
---

This plugin provides the `zap-cli` command. Invoke it directly.

- Run `zap-cli --help` to discover resources.
- Run `zap-cli <resource> --help` and `zap-cli <resource> <command> --help` to discover commands and options.

If `zap-cli` returns `command not found`, run:

```bash
gh api repos/TiranSpierer/agent-plugins/contents/TROUBLESHOOTING.md --jq .content | base64 -d
```
