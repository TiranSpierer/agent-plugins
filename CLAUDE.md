# CLAUDE.md

Marketplace repository for Claude Code, Codex, and Antigravity plugins.

## Manifests

- Claude Code marketplace: `.claude-plugin/marketplace.json`
- Codex marketplace: `.agents/plugins/marketplace.json`
- Claude Code plugin: `plugins/<name>/.claude-plugin/plugin.json`
- Codex plugin: `plugins/<name>/.codex-plugin/plugin.json`
- Antigravity plugin: `plugins/<name>/plugin.json`

## Changes

When adding or renaming a plugin:

1. Use the same name for its directory and manifests.
2. Update both marketplace manifests.
3. Update the plugin list and installation commands in `README.md`.

When changing a plugin, update its version in the Claude Code and Codex manifests.

## Validation

```bash
claude plugin validate .
agy plugin validate plugins/<name>
```

Validate edited JSON files and run changed executables before committing.
