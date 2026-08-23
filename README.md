# Tiran Spierer Agent Plugins

Personal agent plugins distributed from one repository for Claude Code, Codex, and Google Antigravity.

## Plugins

No plugins are published on `main` yet.

## Install the marketplace

### Claude Code

```bash
claude plugin marketplace add https://github.com/TiranSpierer/agent-plugins.git
```

Install a listed plugin with:

```bash
claude plugin install <plugin-name>@tiranspierer-ai-plugins --scope user
```

### Codex

```bash
codex plugin marketplace add https://github.com/TiranSpierer/agent-plugins.git
```

Install a listed plugin with:

```bash
codex plugin add <plugin-name>@tiranspierer
```

### Antigravity

Antigravity installs an individual plugin directory rather than a marketplace manifest:

```bash
git clone https://github.com/TiranSpierer/agent-plugins.git
agy plugin install ./agent-plugins/plugins/<plugin-name>
```

## Repository layout

```text
.claude-plugin/marketplace.json   Claude Code catalog
.agents/plugins/marketplace.json Codex catalog
plugins/                          Installable plugin directories
```
