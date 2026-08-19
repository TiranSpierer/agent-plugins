mod formatter;
mod git;
mod model;
mod quota;

use serde_json::Value;
use std::env;
use std::io::{self, Read};

use formatter::render_statusline;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("Antigravity Custom Statusline v{}", VERSION);
    println!("High-performance native Rust statusline for Antigravity & Claude Code CLI.");
    println!();
    println!("USAGE:");
    println!("  statusline               # Reads JSON payload from stdin (CLI mode)");
    println!("  statusline --test        # Runs a self-test with sample payload");
    println!("  statusline --version     # Prints version information");
    println!("  statusline --help        # Prints this help message");
}

fn print_test_demo() {
    let demo_payload = serde_json::json!({
        "model": {
            "id": "gemini-3.7-flash-high",
            "display_name": "Gemini 3.7 Flash (High)"
        },
        "cwd": env::current_dir().unwrap_or_default().to_string_lossy().to_string(),
        "context_window": {
            "total_input_tokens": 128450,
            "context_window_size": 1048576,
            "used_percentage": 12.25,
            "remaining_percentage": 87.75
        },
        "quota": {
            "remaining_fraction": 0.85,
            "reset_in_seconds": 14200
        }
    });

    println!("{}", render_statusline(&demo_payload));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--version" | "-v" => {
                println!("statusline {}", VERSION);
                return;
            }
            "--test" | "--demo" => {
                print_test_demo();
                return;
            }
            _ => {}
        }
    }

    let mut stdin_str = String::new();
    let _ = io::stdin().read_to_string(&mut stdin_str);

    let trimmed = stdin_str.trim();
    if trimmed.is_empty() {
        println!("Antigravity CLI");
        return;
    }

    let payload: Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => {
            println!("Antigravity CLI");
            return;
        }
    };

    println!("{}", render_statusline(&payload));
}
