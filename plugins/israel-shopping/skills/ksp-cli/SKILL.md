---
name: ksp-cli
description: CLI access to KSP Israel product search, live filters, pricing, availability, specifications, variations, recommendations, and images. Use for questions that need current KSP catalog data.
---

This plugin provides the `ksp-cli` command. Invoke it directly.

- Run `ksp-cli --help` to discover resources.
- Run `ksp-cli <command> --help` and `ksp-cli <command> <subcommand> --help` to discover current arguments, generated files, and options.

If `ksp-cli` returns `command not found`, run:

```bash
gh api repos/TiranSpierer/agent-plugins/contents/TROUBLESHOOTING.md --jq .content | base64 -d
```
