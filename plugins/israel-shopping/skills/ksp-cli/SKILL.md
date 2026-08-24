---
name: ksp-cli
description: CLI access to KSP Israel product search, live filters, pricing, stock, product details, specifications, variations, and images. Use for questions that need current KSP catalog data.
---

This plugin provides the `ksp-cli` command. Invoke it directly.

- Run `ksp-cli --help` to discover commands.
- Run `ksp-cli <command> --help` to discover a command's current arguments and options.

If `ksp-cli` returns `command not found`, run:

```bash
gh api repos/TiranSpierer/agent-plugins/contents/TROUBLESHOOTING.md --jq .content | base64 -d
```
