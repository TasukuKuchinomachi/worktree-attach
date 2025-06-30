#!/usr/bin/env node

import { execSync } from 'child_process';
import inquirer from 'inquirer';

interface Worktree {
  path: string;
  commit: string;
  branch: string;
}

function getWorktrees(): Worktree[] {
  try {
    const output = execSync('git worktree list', { encoding: 'utf-8' });
    const worktrees: Worktree[] = [];
    
    for (const line of output.trim().split('\n')) {
      const parts = line.trim().split(/\s+/);
      if (parts.length >= 3) {
        const path = parts[0];
        const commit = parts[1];
        const branch = parts[2].replace(/[\[\]]/g, '');
        worktrees.push({ path, commit, branch });
      }
    }
    
    return worktrees;
  } catch (error) {
    console.error('Error getting worktrees:', error);
    process.exit(1);
  }
}

async function selectWorktree(worktrees: Worktree[]): Promise<string> {
  if (worktrees.length === 0) {
    console.log('No worktrees found.');
    process.exit(0);
  }

  const choices = worktrees.map(wt => ({
    name: `${wt.branch} - ${wt.path}`,
    value: wt.path
  }));

  const answer = await inquirer.prompt([
    {
      type: 'list',
      name: 'selectedPath',
      message: 'Select a worktree to open in VSCode:',
      choices
    }
  ]);

  return answer.selectedPath;
}

function openInVSCode(path: string): void {
  try {
    execSync(`code "${path}"`, { stdio: 'inherit' });
    console.log(`Opened ${path} in VSCode`);
  } catch (error) {
    console.error('Error opening VSCode:', error);
    process.exit(1);
  }
}

async function main(): Promise<void> {
  const worktrees = getWorktrees();
  const selectedPath = await selectWorktree(worktrees);
  openInVSCode(selectedPath);
}

main().catch(console.error);