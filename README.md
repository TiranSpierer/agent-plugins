# Antigravity & Claude Code Plugins

A central repository and marketplace for Google Antigravity (`agy`) and Claude Code plugins.

## 📦 Available Plugins

- **[`custom-statusline`](./plugins/custom-statusline)**: High-performance native Rust statusline for Antigravity CLI with model brand colors, context usage, fast git branch, and real-time quota tracking.

## 🚀 How to Install & Use

### In Claude Code
```bash
# 1. Add this marketplace
/plugin marketplace add TiranSpierer/antigravity-plugins

# 2. Install any plugin from it
/plugin install custom-statusline@tiranspierer-antigravity-plugins
```

### In Antigravity CLI
```bash
# Install directly from GitHub
agy plugin install https://github.com/TiranSpierer/antigravity-plugins
```

## ➕ Adding a New Plugin

To add a new plugin (e.g. skills, prompts, tools, or hooks):
1. Create a folder: `plugins/<your-plugin-name>/`
2. Add a `plugin.json` manifest:
   ```json
   {
     "name": "<your-plugin-name>",
     "description": "What this plugin does"
   }
   ```
3. Add your content (`skills/<skill-name>/SKILL.md`, rules, hooks, or tools).
4. Register the plugin in `marketplace.json` and `.claude-plugin/marketplace.json`:
   ```json
   {
     "name": "<your-plugin-name>",
     "source": "./plugins/<your-plugin-name>",
     "description": "What this plugin does"
   }
   ```
