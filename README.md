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
codex plugin add israel-shopping@tiranspierer
```

**Antigravity**

```bash
agy plugin install https://github.com/TiranSpierer/agent-plugins/tree/main/plugins/israel-shopping
```

</details>

<details>
<summary><strong>web-platforms</strong> — Reddit search and full discussions</summary>

Search Reddit and individual subreddits, discover communities, browse feeds, and read complete posts with nested comment discussions. Read-only and works without authentication.

**Claude Code**

```bash
claude plugin install web-platforms@tiranspierer --scope user
```

**Codex**

```bash
codex plugin add web-platforms@tiranspierer
```

**Antigravity**

```bash
agy plugin install https://github.com/TiranSpierer/agent-plugins/tree/main/plugins/web-platforms
```

</details>
