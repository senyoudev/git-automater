# Git Automate Project

## Overview

This project is a simple automation script that helps to automate the process of interacting  with a git repository. This script is written in rust and it is a command-line tool.

## Features

- Until now, the script only supports the following commands:
  - `git add .`
  - `git commit -m `${message}`
  - `git push ${remote} ${branch}`

## How to use

- Clone the repository
- Run the following command to build the project:
  - `cargo build --release`
- Run the following command to add the project to the path:
    - `cargo install --path .`

## Usage
 
In the terminal, if you run the following command:
- `git_automate` you will be able to add, commit and push the changes to the repository.

## Future Features

- Add the help command to the script to show the available commands.
- Add the ability to add the remote and branch name to the configuration file.
- Add the possibility of creating a new repository
- Add the possibility of cloning a repository
- Add the possibility of creating a new branch
- Add the possibility of switching branches
