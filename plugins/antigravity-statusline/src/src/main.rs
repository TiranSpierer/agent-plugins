use serde_json::Value;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

// ANSI color codes
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GRAY: &str = "\x1b[90m";
const BLUE: &str = "\x1b[38;2;87;202;255m";
const GREEN: &str = "\x1b[38;2;92;219;109m";
const YELLOW: &str = "\x1b[38;2;255;212;39m";
const ORANGE: &str = "\x1b[38;2;255;152;0m";
const RED: &str = "\x1b[38;2;255;85;85m";

const MODEL_COLORS: &[(&str, &str)] = &[
    ("claude", "\x1b[38;2;221;80;19m"),
    ("gemini", "\x1b[38;2;71;150;227m"),
    ("gpt", "\x1b[38;2;116;170;156m"),
    ("o1", "\x1b[38;2;116;170;156m"),
    ("o3", "\x1b[38;2;116;170;156m"),
];

fn get_tier_color(val: f64, tiers: &[(f64, &'static str)]) -> &'static str {
    for (min, color) in tiers {
        if val >= *min {
            return *color;
        }
    }
    RED
}

fn get_fast_git_branch(start_dir: &Path) -> Option<String> {
    let mut curr = PathBuf::from(start_dir);
    for _ in 0..8 {
        let head_path = curr.join(".git").join("HEAD");
        if head_path.is_file() {
            if let Ok(content) = fs::read_to_string(&head_path) {
                let trimmed = content.trim();
                if let Some(branch) = trimmed.strip_prefix("ref: refs/heads/") {
                    return Some(branch.to_string());
                } else if trimmed.len() >= 7 {
                    return Some(trimmed[..7].to_string());
                }
            }
        }
        if !curr.pop() {
            break;
        }
    }
    None
}

fn format_duration(sec: u64) -> String {
    if sec == 0 {
        return String::new();
    }
    let h = sec / 3600;
    let m = (sec % 3600) / 60;
    if h > 0 {
        format!("{}h {}m", h, m)
    } else {
        format!("{}m", m)
    }
}

fn main() {
    let mut stdin_str = String::new();
    let _ = io::stdin().read_to_string(&mut stdin_str);

    let trimmed = stdin_str.trim();
    if trimmed.is_empty() {
        println!("Antigravity CLI");
        return;
    }

    let d: Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => {
            println!("Antigravity CLI");
            return;
        }
    };

    // 1. Model Name
    let model_name = d["model"]["display_name"]
        .as_str()
        .or_else(|| d["model"]["id"].as_str())
        .or_else(|| d["activeModel"].as_str())
        .unwrap_or("Antigravity");

    let model_lower = model_name.to_lowercase();
    let m_color = MODEL_COLORS
        .iter()
        .find(|(k, _)| model_lower.contains(k))
        .map(|(_, c)| *c)
        .unwrap_or(GRAY);

    let model_part = format!("{}{}{}{}", m_color, BOLD, model_name, RESET);

    // 2. Workspace & Git Branch
    let cwd_raw = d["cwd"]
        .as_str()
        .or_else(|| d["workspace"]["current_dir"].as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let cwd_display = cwd_raw.to_string_lossy().replace('\\', "/");
    let branch = get_fast_git_branch(&cwd_raw);
    let branch_str = branch.map(|b| format!(" [{}]", b)).unwrap_or_default();
    let workspace_part = format!("{}{}{}{}", BOLD, cwd_display, branch_str, RESET);

    // 3. Context Window Usage
    let ctx_pct = d["context_window"]["used_percentage"]
        .as_f64()
        .or_else(|| d["contextWindow"]["used_percentage"].as_f64())
        .or_else(|| d["contextWindow"]["percentage"].as_f64())
        .unwrap_or(0.0);

    let ctx_color = get_tier_color(
        100.0 - ctx_pct,
        &[(75.0, BLUE), (50.0, GREEN), (25.0, YELLOW)],
    );
    let context_part = format!("Context: {}{:.1}%{}", ctx_color, ctx_pct, RESET);

    // 4. API Quota
    let mut quota_part = None;
    if let Some(quota_obj) = d.get("quota") {
        let mut frac = None;
        let mut reset_sec = 0u64;

        if let Some(f) = quota_obj.get("remaining_fraction").and_then(|v| v.as_f64()) {
            frac = Some(f);
            reset_sec = quota_obj
                .get("reset_in_seconds")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
        } else if let Some(map) = quota_obj.as_object() {
            if !map.is_empty() {
                let matched_key = map
                    .keys()
                    .find(|k| {
                        let lk = k.to_lowercase();
                        if model_lower.contains("claude")
                            && (lk.contains("claude") || lk.contains("3p"))
                        {
                            return true;
                        }
                        if model_lower.contains("gemini")
                            && (lk.contains("gemini") || lk.contains("google"))
                        {
                            return true;
                        }
                        false
                    })
                    .or_else(|| map.keys().next());

                if let Some(key) = matched_key {
                    if let Some(bucket) = map.get(key) {
                        frac = bucket.get("remaining_fraction").and_then(|v| v.as_f64());
                        reset_sec = bucket
                            .get("reset_in_seconds")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0);
                    }
                }
            }
        }

        if let Some(f) = frac {
            let remain_pct = (f * 100.0).round() as i64;
            let used_pct = 100 - remain_pct;
            let q_color = get_tier_color(
                f * 100.0,
                &[(50.0, GREEN), (25.0, ORANGE)],
            );
            let timer_str = if reset_sec > 0 {
                let formatted = format_duration(reset_sec);
                if !formatted.is_empty() {
                    format!(" ({})", formatted)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            quota_part = Some(format!(
                "Usage: {}{}%{}{}",
                q_color, used_pct, RESET, timer_str
            ));
        }
    }

    // 5. Assemble status line
    let bullet = format!(" {}•{} ", GRAY, RESET);
    let mut parts = vec![model_part, workspace_part, context_part];
    if let Some(qp) = quota_part {
        parts.push(qp);
    }

    println!("{}", parts.join(&bullet));
}
