use std::fs;
use std::path::{Path, PathBuf};

/// Quickly extracts the active git branch or short commit SHA without spawning child processes.
/// Handles standard repositories, git worktrees, and submodules.
pub fn get_fast_git_branch(start_dir: &Path) -> Option<String> {
    let mut curr = PathBuf::from(start_dir);

    for _ in 0..10 {
        let git_entry = curr.join(".git");

        if git_entry.is_dir() {
            let head_path = git_entry.join("HEAD");
            if let Some(branch) = parse_head_file(&head_path) {
                return Some(branch);
            }
        } else if git_entry.is_file() {
            // Worktree or submodule (.git contains "gitdir: <path>")
            if let Ok(content) = fs::read_to_string(&git_entry) {
                let trimmed = content.trim();
                if let Some(gitdir_rel) = trimmed.strip_prefix("gitdir:") {
                    let gitdir_path = gitdir_rel.trim();
                    let target_path = if Path::new(gitdir_path).is_absolute() {
                        PathBuf::from(gitdir_path)
                    } else {
                        curr.join(gitdir_path)
                    };
                    let head_path = target_path.join("HEAD");
                    if let Some(branch) = parse_head_file(&head_path) {
                        return Some(branch);
                    }
                }
            }
        }

        if !curr.pop() {
            break;
        }
    }
    None
}

fn parse_head_file(head_path: &Path) -> Option<String> {
    if !head_path.is_file() {
        return None;
    }
    let content = fs::read_to_string(head_path).ok()?;
    let trimmed = content.trim();

    if let Some(branch) = trimmed.strip_prefix("ref: refs/heads/") {
        Some(branch.to_string())
    } else if let Some(tag) = trimmed.strip_prefix("ref: refs/tags/") {
        Some(format!("tag:{}", tag))
    } else if trimmed.len() >= 7 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        // Detached HEAD at commit SHA
        Some(trimmed[..7].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_head_branch() {
        let ref_content = "ref: refs/heads/main\n";
        assert_eq!(
            ref_content.trim().strip_prefix("ref: refs/heads/"),
            Some("main")
        );
    }

    #[test]
    fn test_parse_head_detached_sha() {
        let sha_content = "4b825dc642cb6eb9a060e54bf8d69288fbee4904\n";
        let trimmed = sha_content.trim();
        assert!(trimmed.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(&trimmed[..7], "4b825dc");
    }
}
