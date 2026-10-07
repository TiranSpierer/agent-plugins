---
name: baby-cli
description: CLI access to current products, prices, sales, availability, variants, barcodes, descriptions, and collections from major Israeli baby retailers. Use for baby-product shopping and comparisons across Shilav, Baby Star, Motsesim, Agalease, and My Baby.
---

This plugin provides the `baby-cli` command. Invoke it directly.

- Run `baby-cli --help` to discover resources.
- Run `baby-cli <command> --help` and `baby-cli <command> <subcommand> --help` to discover commands and options.

If `baby-cli` returns `command not found`, run:

```bash
gh api repos/TiranSpierer/agent-plugins/contents/TROUBLESHOOTING.md --jq .content | base64 -d
```
