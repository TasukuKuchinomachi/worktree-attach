use clap::Parser;
use inquire::Select;
use std::process::{exit, Command};

#[derive(Parser)]
#[command(name = "worktree-attach")]
#[command(about = "A CLI tool to select and open git worktrees in VSCode")]
struct Cli {}

#[derive(Debug, Clone)]
struct Worktree {
    path: String,
    branch: String,
}

impl std::fmt::Display for Worktree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.branch, self.path)
    }
}

fn get_worktrees() -> Result<Vec<Worktree>, Box<dyn std::error::Error>> {
    let output = Command::new("git").args(["worktree", "list"]).output()?;

    if !output.status.success() {
        eprintln!(
            "Error getting worktrees: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut worktrees = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // NOTE: `git worktree list`` output like below:
        // /path/to/worktree 1234567 [branch-name]
        if parts.len() >= 3 {
            let path = parts[0].to_string();
            // let commit = parts[1].to_string(); // now unused
            let branch = parts[2].trim_matches(['[', ']']).to_string();
            worktrees.push(Worktree { path, branch });
        }
    }

    Ok(worktrees)
}

fn select_worktree(worktrees: Vec<Worktree>) -> Result<String, Box<dyn std::error::Error>> {
    if worktrees.is_empty() {
        println!("No worktrees found.");
        exit(0);
    }

    let selection = Select::new("Select a worktree to open in VSCode:", worktrees).prompt()?;

    Ok(selection.path)
}

fn open_in_vscode(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("code").arg(path).output()?;

    if !output.status.success() {
        eprintln!(
            "Error opening VSCode: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    println!("Opened {} in VSCode", path);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _cli = Cli::parse();

    let worktrees = get_worktrees()?;
    let selected_path = select_worktree(worktrees)?;
    open_in_vscode(&selected_path)?;

    Ok(())
}
