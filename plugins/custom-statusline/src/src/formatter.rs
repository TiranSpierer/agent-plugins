use serde_json::Value;
use std::env;
use std::path::{Path, PathBuf};

use crate::git::get_fast_git_branch;
use crate::model::format_model;
use crate::quota::extract_quota_info;

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const GRAY: &str = "\x1b[90m";

// Context window health colors
pub const COLOR_CTX_BLUE: &str = "\x1b[38;2;87;202;255m";   // >= 75% remaining #57CAFF
pub const COLOR_CTX_GREEN: &str = "\x1b[38;2;92;219;109m";  // >= 50% remaining #5CDB6D
pub const COLOR_CTX_YELLOW: &str = "\x1b[38;2;255;212;39m"; // >= 25% remaining #FFD427
pub const COLOR_CTX_RED: &str = "\x1b[38;2;255;85;85m";     // < 25% remaining #FF5555

pub fn get_context_color(remaining_pct: f64) -> &'static str {
    if remaining_pct >= 75.0 {
        COLOR_CTX_BLUE
    } else if remaining_pct >= 50.0 {
        COLOR_CTX_GREEN
    } else if remaining_pct >= 25.0 {
        COLOR_CTX_YELLOW
    } else {
        COLOR_CTX_RED
    }
}

pub fn format_path_display(path: &Path) -> String {
    let raw = path.to_string_lossy().replace('\\', "/");

    // Replace user home directory with ~
    if let Ok(home) = env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
        let home_norm = home.replace('\\', "/");
        if raw.starts_with(&home_norm) {
            return raw.replacen(&home_norm, "~", 1);
        }
    }
    raw
}

pub fn render_statusline(payload: &Value) -> String {
    // 1. Model Component
    let model_name = payload["model"]["display_name"]
        .as_str()
        .or_else(|| payload["model"]["id"].as_str())
        .or_else(|| payload["activeModel"].as_str())
        .or_else(|| payload["model"].as_str())
        .unwrap_or("Antigravity");

    let model_part = format_model(model_name);

    // 2. Workspace & Git Branch Component
    let cwd_raw = payload["cwd"]
        .as_str()
        .or_else(|| payload["workspace"]["current_dir"].as_str())
        .or_else(|| payload["workspace"]["root"].as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let cwd_display = format_path_display(&cwd_raw);
    let branch = get_fast_git_branch(&cwd_raw);
    let branch_str = branch.map(|b| format!(" [{}]", b)).unwrap_or_default();
    let workspace_part = format!("{}{}{}{}", BOLD, cwd_display, branch_str, RESET);

    // 3. Context Window Usage Component
    let (ctx_pct, ctx_rem) = if let Some(used) = payload["context_window"]["used_percentage"].as_f64() {
        let rem = payload["context_window"]["remaining_percentage"]
            .as_f64()
            .unwrap_or(100.0 - used);
        (used, rem)
    } else if let Some(used) = payload["contextWindow"]["used_percentage"].as_f64() {
        let rem = payload["contextWindow"]["remaining_percentage"]
            .as_f64()
            .unwrap_or(100.0 - used);
        (used, rem)
    } else if let Some(pct) = payload["contextWindow"]["percentage"].as_f64() {
        (pct, 100.0 - pct)
    } else {
        (0.0, 100.0)
    };

    let ctx_color = get_context_color(ctx_rem);
    let context_part = format!("Context: {}{:.1}%{}", ctx_color, ctx_pct, RESET);

    // 4. API Quota Component (if available)
    let quota_part = payload.get("quota").and_then(|q| extract_quota_info(q, model_name));

    // 5. Assemble all active segments
    let bullet = format!(" {}•{} ", GRAY, RESET);
    let mut parts = vec![model_part, workspace_part, context_part];
    if let Some(qp) = quota_part {
        parts.push(qp);
    }

    parts.join(&bullet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_render_statusline() {
        let payload = json!({
            "model": { "display_name": "Claude Sonnet 4.6 (Thinking)" },
            "cwd": "C:/git/antigravity-plugins",
            "context_window": {
                "used_percentage": 15.4,
                "remaining_percentage": 84.6
            },
            "quota": {
                "remaining_fraction": 0.80,
                "reset_in_seconds": 3600
            }
        });

        let output = render_statusline(&payload);
        assert!(output.contains("Claude Sonnet 4.6 (Thinking)"));
        assert!(output.contains("Context:"));
        assert!(output.contains("15.4%"));
        assert!(output.contains("Usage:"));
        assert!(output.contains("20%"));
    }
}
