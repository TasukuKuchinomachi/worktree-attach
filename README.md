# worktree-attach

A CLI tool to select and open git worktrees in VSCode.

## Overview

`worktree-attach` is a Rust-based command-line tool that allows you to easily select from available git worktrees and open them in Visual Studio Code. It provides an interactive menu to choose from your git worktrees and automatically opens the selected one in VSCode.

## Features

- Lists all available git worktrees in the current repository
- Interactive selection menu using arrow keys
- Automatically opens the selected worktree in VSCode
- Clean, user-friendly output with branch names and paths

## Prerequisites

- Git with worktree support
- Visual Studio Code (`code` command available in PATH)
- Rust and Cargo (for building from source)

## Installation

### From Source

1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd worktree-attach
   ```

2. Build and install:
   ```bash
   cargo build --release
   cargo install --path .
   ```

## Usage

Navigate to a git repository with worktrees and run:

```bash
worktree-attach
```

The tool will:
1. Scan for available git worktrees
2. Present an interactive menu showing branch names and paths
3. Open the selected worktree in VSCode

### Example Output

```
? Select a worktree to open in VSCode:
❯ main - /path/to/main/worktree
  feature-branch - /path/to/feature/worktree  
  bugfix-branch - /path/to/bugfix/worktree
```

## Error Handling

The tool handles common error scenarios:
- No git worktrees found
- Git command failures
- VSCode launch failures
