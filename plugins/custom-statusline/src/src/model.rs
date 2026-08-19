pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const GRAY: &str = "\x1b[90m";

// Provider TrueColor palette
pub const COLOR_CLAUDE: &str = "\x1b[38;2;221;80;19m";  // Anthropic Warm Orange #DD5013
pub const COLOR_GEMINI: &str = "\x1b[38;2;71;150;227m";  // Google Blue #4796E3
pub const COLOR_OPENAI: &str = "\x1b[38;2;116;170;156m"; // OpenAI Teal #74AA9C
pub const COLOR_DEEPSEEK: &str = "\x1b[38;2;77;159;255m"; // DeepSeek Cyan #4D9FFF
pub const COLOR_MISTRAL: &str = "\x1b[38;2;255;112;0m";  // Mistral Orange #FF7000
pub const COLOR_META: &str = "\x1b[38;2;4;104;215m";     // Meta Blue #0468D7

pub const MODEL_RULES: &[(&[&str], &str)] = &[
    (&["claude", "anthropic", "sonnet", "opus", "haiku"], COLOR_CLAUDE),
    (&["gemini", "google", "flash", "pro"], COLOR_GEMINI),
    (&["gpt", "openai", "o1", "o3", "chatgpt"], COLOR_OPENAI),
    (&["deepseek", "r1", "v3"], COLOR_DEEPSEEK),
    (&["mistral", "codestral", "pixtral"], COLOR_MISTRAL),
    (&["llama", "meta"], COLOR_META),
];

pub fn get_model_color(model_name: &str) -> &'static str {
    let lower = model_name.to_lowercase();
    for (keywords, color) in MODEL_RULES {
        if keywords.iter().any(|&kw| lower.contains(kw)) {
            return color;
        }
    }
    GRAY
}

pub fn format_model(model_name: &str) -> String {
    let color = get_model_color(model_name);
    format!("{}{}{}{}", color, BOLD, model_name, RESET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_colors() {
        assert_eq!(get_model_color("Claude Sonnet 4.6 (Thinking)"), COLOR_CLAUDE);
        assert_eq!(get_model_color("Gemini 3.7 Flash (High)"), COLOR_GEMINI);
        assert_eq!(get_model_color("GPT-4o"), COLOR_OPENAI);
        assert_eq!(get_model_color("o1-preview"), COLOR_OPENAI);
        assert_eq!(get_model_color("DeepSeek-R1"), COLOR_DEEPSEEK);
        assert_eq!(get_model_color("Unknown-Model"), GRAY);
    }
}
