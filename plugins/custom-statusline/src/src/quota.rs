use serde_json::Value;

pub const COLOR_GREEN: &str = "\x1b[38;2;92;219;109m";
pub const COLOR_ORANGE: &str = "\x1b[38;2;255;152;0m";
pub const COLOR_RED: &str = "\x1b[38;2;255;85;85m";
pub const RESET: &str = "\x1b[0m";

pub fn format_duration(sec: u64) -> String {
    if sec == 0 {
        return String::new();
    }
    let h = sec / 3600;
    let m = (sec % 3600) / 60;
    let s = sec % 60;
    if h > 0 {
        format!("{}h {}m", h, m)
    } else if m > 0 {
        format!("{}m", m)
    } else {
        format!("{}s", s)
    }
}

pub fn get_quota_color(remaining_pct: f64) -> &'static str {
    if remaining_pct >= 50.0 {
        COLOR_GREEN
    } else if remaining_pct >= 25.0 {
        COLOR_ORANGE
    } else {
        COLOR_RED
    }
}

pub fn extract_quota_info(quota_val: &Value, model_name: &str) -> Option<String> {
    let model_lower = model_name.to_lowercase();
    let mut frac: Option<f64> = None;
    let mut reset_sec: u64 = 0;

    // 1. Direct flat quota
    if let Some(f) = quota_val.get("remaining_fraction").and_then(|v| v.as_f64()) {
        frac = Some(f);
        reset_sec = quota_val
            .get("reset_in_seconds")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
    } else if let Some(pct) = quota_val.get("remaining_percentage").and_then(|v| v.as_f64()) {
        frac = Some(pct / 100.0);
        reset_sec = quota_val
            .get("reset_in_seconds")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
    } else if let Some(map) = quota_val.as_object() {
        // 2. Bucketed by model or provider
        if !map.is_empty() {
            let matched_key = map
                .keys()
                .find(|k| {
                    let lk = k.to_lowercase();
                    if model_lower.contains("claude") && (lk.contains("claude") || lk.contains("3p") || lk.contains("anthropic")) {
                        return true;
                    }
                    if model_lower.contains("gemini") && (lk.contains("gemini") || lk.contains("google")) {
                        return true;
                    }
                    if (model_lower.contains("gpt") || model_lower.contains("o1") || model_lower.contains("o3")) && (lk.contains("gpt") || lk.contains("openai")) {
                        return true;
                    }
                    if model_lower.contains("deepseek") && lk.contains("deepseek") {
                        return true;
                    }
                    false
                })
                .or_else(|| map.keys().next());

            if let Some(key) = matched_key {
                if let Some(bucket) = map.get(key) {
                    frac = bucket
                        .get("remaining_fraction")
                        .and_then(|v| v.as_f64())
                        .or_else(|| {
                            bucket
                                .get("remaining_percentage")
                                .and_then(|v| v.as_f64())
                                .map(|p| p / 100.0)
                        });
                    reset_sec = bucket
                        .get("reset_in_seconds")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                }
            }
        }
    }

    if let Some(f) = frac {
        let remaining_pct = (f * 100.0).clamp(0.0, 100.0);
        let used_pct = (100.0 - remaining_pct).round() as i64;
        let q_color = get_quota_color(remaining_pct);

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

        Some(format!(
            "Usage: {}{}%{}{}",
            q_color, used_pct, RESET, timer_str
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(3600 + 120), "1h 2m");
        assert_eq!(format_duration(300), "5m");
        assert_eq!(format_duration(45), "45s");
        assert_eq!(format_duration(0), "");
    }

    #[test]
    fn test_flat_quota() {
        let q = json!({
            "remaining_fraction": 0.75,
            "reset_in_seconds": 3600
        });
        let res = extract_quota_info(&q, "claude-3-5-sonnet").unwrap();
        assert!(res.contains("25%"));
        assert!(res.contains("1h 0m"));
    }

    #[test]
    fn test_bucketed_quota() {
        let q = json!({
            "claude": {
                "remaining_fraction": 0.40,
                "reset_in_seconds": 1800
            },
            "gemini": {
                "remaining_fraction": 0.90,
                "reset_in_seconds": 7200
            }
        });
        let res_claude = extract_quota_info(&q, "Claude Sonnet 4.6").unwrap();
        assert!(res_claude.contains("60%"));
        assert!(res_claude.contains("30m"));

        let res_gemini = extract_quota_info(&q, "Gemini 3.7 Flash").unwrap();
        assert!(res_gemini.contains("10%"));
        assert!(res_gemini.contains("2h 0m"));
    }
}
