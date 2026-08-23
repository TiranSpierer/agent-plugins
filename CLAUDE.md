# CLAUDE.md

This repository distributes personal agent capabilities for Claude Code, Codex, and Google Antigravity.

## Design

- Plugins expose raw capabilities. Skills explain what a CLI can access and how to discover its commands; they do not prescribe research or decision-making workflows.
- The conversation supplies the goal. The agent decides which installed capabilities to combine.
- CLI command definitions and help are maintained in the tool repository, not copied into skill prose.
- Each substantial CLI or MCP implementation lives in its own repository. This marketplace contains only manifests, thin skills, and launch shims.
- A plugin directory and every platform manifest must use the same lowercase kebab-case name.

## Repository structure

- `.claude-plugin/marketplace.json` — Claude Code marketplace catalog.
- `.agents/plugins/marketplace.json` — Codex marketplace catalog.
- `plugins/<name>/` — plugin bundle shared across supported agents.

Each cross-platform plugin normally contains:

```text
plugins/<name>/
├── .claude-plugin/plugin.json
├── .codex-plugin/plugin.json
├── plugin.json
├── bin/
└── skills/<cli-name>/SKILL.md
```

## Shared skill and CLI conventions

- Skill descriptions must identify the data or service exposed so agents can route relevant requests correctly.
- Keep `SKILL.md` short. State that the CLI is installed, name it, and direct the agent to `<cli> --help` and `<cli> <command> --help` for discovery.
- Do not encode a fixed sequence of commands or a task workflow in a capability skill.
- Shell shims use `exec npx --prefer-offline -y -p "git+https://github.com/TiranSpierer/<repo>.git" <cli> "$@"`.
- Do not place credentials in the marketplace. Document authentication only when a tool requires user configuration.
- Bump all applicable plugin versions together when changing a published plugin.

## Claude Code quirks

- Marketplace entries live in `.claude-plugin/marketplace.json` and use `"source": "./plugins/<name>"`.
- Every local plugin needs `.claude-plugin/plugin.json` at its root.
- Claude automatically discovers `skills/` and exposes executables from `bin/` in the plugin environment.
- Validate repository changes with `claude plugin validate .`.
- User installation is `claude plugin marketplace add <repo>` followed by `claude plugin install <name>@tiranspierer --scope user`.

## Codex quirks

- Marketplace entries live in `.agents/plugins/marketplace.json`.
- Every entry includes a local source path, installation/authentication policy, and category.
- Every plugin needs `.codex-plugin/plugin.json`; declare `"skills": "./skills/"` and valid interface metadata.
- Codex manifests must not declare unsupported fields such as hooks.
- Validate each plugin with the Codex plugin-creator validator.
- User installation is `codex plugin marketplace add <repo>` followed by `codex plugin add <name>@tiranspierer`.

## Antigravity quirks

- Antigravity does not consume the Claude or Codex marketplace catalogs. It installs an individual plugin directory with `agy plugin install <path>`.
- Every plugin needs a root `plugin.json` using the Antigravity schema. Only `$schema`, `name`, and `description` are supported; additional metadata belongs in the other manifests.
- Antigravity discovers the same `skills/` directory and stages plugin files under its global plugin directory.
- The reliable GitHub installation flow is to clone this repository and pass `plugins/<name>` to `agy plugin install`.

## Adding a plugin

1. Create the three manifests, shared skill, and CLI shim under `plugins/<name>/`.
2. Add the plugin to both marketplace catalogs.
3. Add only currently available behavior to README; do not document planned capabilities.
4. Validate JSON, skills, Claude, Codex, and executable shims before opening a PR.
