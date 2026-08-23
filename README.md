# Tiran Spierer Agent Plugins

Personal agent plugins distributed from one repository for Claude Code, Codex, and Google Antigravity.

## Plugins

| Plugin | Current capabilities |
|---|---|
| [`israel-market`](plugins/israel-market) | Search KSP products and filters; read product prices, availability, variations, specifications, and images. |
| [`web-sources`](plugins/web-sources) | Search Reddit and subreddits; browse feeds; read full posts and nested comment discussions. |

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

Antigravity installs an individual plugin directory:

```bash
agy plugin install https://github.com/TiranSpierer/agent-plugins/tree/main/plugins/<plugin-name>
```

## Repository layout

```text
.claude-plugin/marketplace.json   Claude Code catalog
.agents/plugins/marketplace.json Codex catalog
plugins/                          Installable plugin directories
```
