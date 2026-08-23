# CLAUDE.md

This repository is a marketplace of CLI-backed plugins for Claude Code, Codex, and Antigravity.

## Structure

- `.claude-plugin/marketplace.json` lists the Claude Code plugins.
- `.agents/plugins/marketplace.json` lists the Codex plugins.
- `plugins/<name>/` contains an installable plugin.

A plugin can provide:

- `.claude-plugin/plugin.json` for Claude Code.
- `.codex-plugin/plugin.json` for Codex.
- `plugin.json` for Antigravity.
- `skills/` for agent skills.
- `bin/` for CLI launchers.

## Changes

- Keep plugin names consistent between the directory and its manifests.
- Register new plugins in both marketplace manifests.
- Keep skills focused on capability discovery. CLI commands and options belong in the CLI's `--help` output.
- Update README installation commands and plugin descriptions when their user-facing behavior changes.
- Bump the plugin version in its Claude and Codex manifests when changing a plugin.

## Validation

```bash
claude plugin validate .
agy plugin validate plugins/<name>
```

Also validate edited JSON files and run each changed launcher locally.
