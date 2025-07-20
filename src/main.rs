use clap::Parser;
use inquire::Select;
use std::process::{exit, Command};
use git2::Repository;

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
    // Open the repository from current directory
    let repo = Repository::open_from_env()?;
    let mut worktrees = Vec::new();

    // Get the main worktree (the repository itself)
    let workdir = repo.workdir().ok_or("Repository has no working directory")?;
    let head_ref = repo.head()?;
    let main_branch = if head_ref.is_branch() {
        head_ref.shorthand().unwrap_or("HEAD").to_string()
    } else {
        "HEAD".to_string()
    };
    
    worktrees.push(Worktree {
        path: workdir.to_string_lossy().to_string(),
        branch: main_branch,
    });

    // Get additional worktrees
    let worktree_list = repo.worktrees()?;
    for name in worktree_list.iter() {
        if let Some(name_str) = name {
            if let Ok(worktree) = repo.find_worktree(name_str) {
                if let Some(path) = worktree.path() {
                    // Try to determine the branch for this worktree
                    let branch_name = if let Ok(wt_repo) = Repository::open(path) {
                        let head = wt_repo.head().ok();
                        if let Some(head_ref) = head {
                            if head_ref.is_branch() {
                                head_ref.shorthand().unwrap_or(name_str).to_string()
                            } else {
                                name_str.to_string()
                            }
                        } else {
                            name_str.to_string()
                        }
                    } else {
                        name_str.to_string()
                    };
                    
                    worktrees.push(Worktree {
                        path: path.to_string_lossy().to_string(),
                        branch: branch_name,
                    });
                }
            }
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
