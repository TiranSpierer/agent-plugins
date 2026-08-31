# agent-plugins

A plugin marketplace for Claude Code, Codex, and Antigravity.

## Setup

<details>
<summary>Claude Code</summary>

Add the marketplace once:

```bash
claude plugin marketplace add https://github.com/TiranSpierer/agent-plugins.git
```

</details>

<details>
<summary>Codex</summary>

Add the marketplace once:

```bash
codex plugin marketplace add https://github.com/TiranSpierer/agent-plugins.git
```

</details>

<details>
<summary>Antigravity</summary>

Antigravity doesn't have a marketplace. Use the command shown under the plugin you want below.

</details>

## Plugins

<details>
<summary><strong>israel-shopping</strong> — KSP and Zap product research</summary>

Search KSP's live catalog and inspect product pricing, availability, variations, specifications, recommendations, and images. Search Zap's cross-store catalog, compare regular and Eilat offers, inspect delivered prices, specifications, reviews, price history, categories, filters, similar products, and stores. Read-only and requires no authentication.

**Claude Code**

```bash
claude plugin install israel-shopping@tirsi --scope user
```

**Codex**

```bash
codex plugin add israel-shopping@tirsi
```

**Antigravity**

```bash
agy plugin install https://github.com/TiranSpierer/agent-plugins/tree/main/plugins/israel-shopping
```

</details>

<details>
<summary><strong>web-platforms</strong> — Reddit and YouTube</summary>

Search Reddit and individual subreddits, browse feeds, and read complete discussions. Search YouTube, inspect video and channel metadata, browse channel and playlist videos, and save transcripts or comments for local analysis. Read-only and works without authentication.

**Claude Code**

```bash
claude plugin install web-platforms@tirsi --scope user
```

**Codex**

```bash
codex plugin add web-platforms@tirsi
```

**Antigravity**

```bash
agy plugin install https://github.com/TiranSpierer/agent-plugins/tree/main/plugins/web-platforms
```

</details>

## Troubleshooting

If a plugin command is unavailable, see [CLI troubleshooting](TROUBLESHOOTING.md).
