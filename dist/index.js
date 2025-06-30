#!/usr/bin/env node
"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const child_process_1 = require("child_process");
const inquirer_1 = __importDefault(require("inquirer"));
function getWorktrees() {
    try {
        const output = (0, child_process_1.execSync)('git worktree list', { encoding: 'utf-8' });
        const worktrees = [];
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
    }
    catch (error) {
        console.error('Error getting worktrees:', error);
        process.exit(1);
    }
}
async function selectWorktree(worktrees) {
    if (worktrees.length === 0) {
        console.log('No worktrees found.');
        process.exit(0);
    }
    const choices = worktrees.map(wt => ({
        name: `${wt.branch} - ${wt.path}`,
        value: wt.path
    }));
    const answer = await inquirer_1.default.prompt([
        {
            type: 'list',
            name: 'selectedPath',
            message: 'Select a worktree to open in VSCode:',
            choices
        }
    ]);
    return answer.selectedPath;
}
function openInVSCode(path) {
    try {
        (0, child_process_1.execSync)(`code "${path}"`, { stdio: 'inherit' });
        console.log(`Opened ${path} in VSCode`);
    }
    catch (error) {
        console.error('Error opening VSCode:', error);
        process.exit(1);
    }
}
async function main() {
    const worktrees = getWorktrees();
    const selectedPath = await selectWorktree(worktrees);
    openInVSCode(selectedPath);
}
main().catch(console.error);
