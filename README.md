# agent-plugins

Marketplace of CLI-backed capabilities for coding agents. Install a plugin to give your agent direct access to sources that ordinary web search cannot fully query or inspect.

Plugins expose the underlying CLIs without prescribing a workflow, leaving the agent free to combine them based on the conversation.

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
<summary><strong>israel-market</strong> — KSP product search and details</summary>

Search KSP's live catalog and filters, then inspect product pricing, availability, variations, specifications, and images. Read-only and requires no authentication.

**Claude Code**

```bash
claude plugin install israel-market@tiranspierer --scope user
```

**Codex**

```bash
codex plugin add israel-market@tiranspierer
```

**Antigravity**

```bash
agy plugin install https://github.com/TiranSpierer/agent-plugins/tree/main/plugins/israel-market
```

</details>

<details>
<summary><strong>web-sources</strong> — Reddit search and full discussions</summary>

Search Reddit and individual subreddits, discover communities, browse feeds, and read complete posts with nested comment discussions. Read-only and works without authentication.

**Claude Code**

```bash
claude plugin install web-sources@tiranspierer --scope user
```

**Codex**

```bash
codex plugin add web-sources@tiranspierer
```

**Antigravity**

```bash
agy plugin install https://github.com/TiranSpierer/agent-plugins/tree/main/plugins/web-sources
```

</details>
