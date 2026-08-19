# AI Agent Plugins (Antigravity & Claude Code)

A centralized, general-purpose repository for AI agent plugins, skills, and extensions supporting **Google Antigravity (`agy`)** and **Claude Code** (with extensible layout for other agent harnesses like Codex).

## 📦 Plugins Catalog

| Plugin | Target Agent | Description | Directory |
| :--- | :---: | :--- | :--- |
| **`antigravity-statusline`** | Antigravity CLI (`agy`) | Ultra-fast native Rust statusline with model brand colors, real-time quota countdown, fast Git branch detection, and color-coded context usage. | [`plugins/antigravity-statusline`](./plugins/antigravity-statusline) |

## 🚀 Installation

### In Antigravity CLI
```bash
agy plugin install https://github.com/TiranSpierer/antigravity-plugins
```

### In Claude Code
```bash
/plugin marketplace add TiranSpierer/antigravity-plugins
/plugin install <plugin-name>@tiranspierer-ai-plugins
```

## ➕ Adding New Plugins

To add any new plugin (skill, rules, tool, or hook):
1. Create a folder: `plugins/<your-plugin-name>/`
2. Add `plugin.json`:
   ```json
   {
     "name": "<your-plugin-name>",
     "description": "Description of plugin"
   }
   ```
3. Add your plugin contents (`skills/<name>/SKILL.md`, rules, hooks, etc.).
4. Add an entry to `marketplace.json` and `.claude-plugin/marketplace.json`:
   ```json
   {
     "name": "<your-plugin-name>",
     "source": "./plugins/<your-plugin-name>",
     "description": "Description of plugin"
   }
   ```
