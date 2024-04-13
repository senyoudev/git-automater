mod display_help;
mod update_commit_push;
mod clone;
mod config_repo;

use display_help::display_help;
use std::env::args;
use std::process::exit;
use update_commit_push::{add, commit, push, update_commit_push};
use clone::clone;
use config_repo::read_config;

fn main() {
    // params should be passed in the cli as arguments and we get them using std::env::args() or passed to the config file
    let config = read_config();
    let args: Vec<String> = args().collect();
    let default_remote = &config.remote.unwrap_or_else(|| String::from("origin"));
    let default_branch = &config.branch.unwrap_or_else(|| String::from("main"));
    let default_commit_message = String::from("Default commit message");

    if args.len() < 2 || args[1] == "help" {
        display_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "clone" => {
            let url = args.get(2).expect("URL is required");
            // check if the URL is valid
            if !url.starts_with("https://") && !url.starts_with("git@") {
                eprintln!("Invalid URL");
                exit(1);
            }

            clone(url);

        },
        "help" => {
            display_help();
            return;
        }
        "add" => {
            let excluded_files = if let Some(files) = args.get(2) {
                files.split(",").collect()
            } else {
                vec![]
            };
            add(excluded_files);
            return;
        }
        "commit" => {
            let commit_message = args
                .get(2)
                .unwrap_or(&default_commit_message);
            commit(commit_message);
        }
        "push" => {
            let remote = args.get(2).unwrap_or(&default_remote);
            let branch = args.get(3).unwrap_or(&default_branch);
            // check if the remote or branch name contain invalid characters
            if !check_for_invalid_characters(remote) {
                eprintln!("Remote name contains invalid characters");
                exit(1);
            }

            if !check_for_invalid_characters(branch) {
                eprintln!("Branch name contains invalid characters");
                exit(1);
            }
            push(remote, branch);
        }
        "pushFromOne" => {
             let excluded_files = if let Some(files) = args.get(3) {
                files.split(",").collect()
            } else {
                vec![]
            };
            let commit_message = args
                .get(2)
                .unwrap_or(&default_commit_message);
            let remote = args.get(4).unwrap_or(&default_remote);
            let branch = args.get(5).unwrap_or(&default_branch);
            // check if the remote or branch name contain invalid characters
            if !check_for_invalid_characters(remote) {
                eprintln!("Remote name contains invalid characters");
                exit(1);
            }

            if !check_for_invalid_characters(branch) {
                eprintln!("Branch name contains invalid characters");
                exit(1);
            }
            update_commit_push(excluded_files, commit_message, remote, branch);
        }
        _ => {}
    }
}

fn check_for_invalid_characters(name: &str) -> bool {
    if name.contains(|c: char| !(c.is_alphanumeric() || c == '-' || c == '_')) {
        return false;
    }
    true
}
