#!/usr/bin/env bash
set -e

echo -e "\033[36m============================================================\033[0m"
echo -e "\033[32m  Antigravity CLI Custom Statusline - Fast Installer (Unix) \033[0m"
echo -e "\033[36m============================================================\033[0m"

TARGET_DIR="$HOME/.gemini/antigravity-cli/plugins/custom-statusline"
BIN_DIR="$TARGET_DIR/bin"
SKILLS_DIR="$TARGET_DIR/skills/custom-statusline"

mkdir -p "$BIN_DIR" "$SKILLS_DIR"

INSTALLED_BIN="$BIN_DIR/statusline"

# Check if running inside local repo
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

if [ -f "$REPO_ROOT/plugins/custom-statusline/src/Cargo.toml" ] && command -v cargo >/dev/null 2>&1; then
    echo -e "\033[33m[1/4] Building statusline from source with cargo...\033[0m"
    (cd "$REPO_ROOT/plugins/custom-statusline/src" && cargo build --release)
    cp "$REPO_ROOT/plugins/custom-statusline/src/target/release/statusline" "$INSTALLED_BIN"
    cp "$REPO_ROOT/plugins/custom-statusline/plugin.json" "$TARGET_DIR/plugin.json"
    cp "$REPO_ROOT/plugins/custom-statusline/skills/custom-statusline/SKILL.md" "$SKILLS_DIR/SKILL.md"
else
    echo -e "\033[33m[1/4] Downloading latest statusline binary from GitHub Releases...\033[0m"
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    ARCH="$(uname -m)"
    if [ "$ARCH" = "x86_64" ]; then
        RELEASE_NAME="statusline-${OS}-x86_64"
    elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
        RELEASE_NAME="statusline-${OS}-arm64"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
    curl -fsSL "https://github.com/TiranSpierer/antigravity-plugins/releases/latest/download/${RELEASE_NAME}" -o "$INSTALLED_BIN"
fi

chmod +x "$INSTALLED_BIN"

echo -e "\033[33m[2/4] Testing binary execution...\033[0m"
"$INSTALLED_BIN" --demo

echo -e "\033[33m[3/4] Updating configuration...\033[0m"
SETTINGS_FILE="$HOME/.gemini/antigravity-cli/settings.json"
if [ -f "$SETTINGS_FILE" ]; then
    node -e "
        const fs = require('fs');
        const file = process.argv[1];
        const bin = process.argv[2];
        const data = JSON.parse(fs.readFileSync(file, 'utf8'));
        data.statusLine = { type: 'command', command: bin, enabled: true };
        fs.writeFileSync(file, JSON.stringify(data, null, 2));
    " "$SETTINGS_FILE" "$INSTALLED_BIN" 2>/dev/null || true
fi

echo -e "\033[33m[4/4] Validating plugin...\033[0m"
if command -v agy >/dev/null 2>&1; then
    agy plugin validate "$TARGET_DIR" || true
fi

echo -e "\n\033[32mCustom Statusline successfully installed & activated!\033[0m"
