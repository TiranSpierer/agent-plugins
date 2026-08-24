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

Codex loads the plugin skills but does not add the bundled `bin/` launchers to `PATH`. Install each plugin's CLIs separately using the commands shown below.

</details>

<details>
<summary>Antigravity</summary>

Antigravity installs each plugin directly from its GitHub directory. Use the command shown under the plugin you want below.

</details>

## Plugins

<details>
<summary><strong>israel-shopping</strong> — KSP product search and details</summary>

Search KSP's live catalog and filters, then inspect product pricing, availability, variations, specifications, and images. Read-only and requires no authentication.

**Claude Code**

```bash
claude plugin install israel-shopping@tiranspierer --scope user
```

**Codex**

```bash
npm install -g git+https://github.com/TiranSpierer/ksp-mcp.git
codex plugin add israel-shopping@tiranspierer
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
claude plugin install web-platforms@tiranspierer --scope user
```

**Codex**

```bash
npm install -g git+https://github.com/TiranSpierer/reddit-mcp.git
uv tool install git+https://github.com/TiranSpierer/youtube-cli.git
codex plugin add web-platforms@tiranspierer
```

**Antigravity**

```bash
agy plugin install https://github.com/TiranSpierer/agent-plugins/tree/main/plugins/web-platforms
```

</details>
