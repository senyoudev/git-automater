use std::env::args;
use std::process::{exit, Command};

fn update_commit_push(commit_message: &str, remote: &str, branch: &str) {
    // Create a new command to run /git add ./
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to add files");

    // Check if the command was successful
    if !add_command.status.success() {
        eprintln!(
            "Failed to add files : {}",
            String::from_utf8_lossy(&add_command.stderr) // This will converts the error to a string and print it
        );
        exit(1);
    }

    // Create a new command to run /git commit -m $COMMIT_MESSAGE/
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("Failed to commit files");

    // Check if the command was successful
    if !commit_command.status.success() {
        eprintln!(
            "Failed to commit files: {}",
            String::from_utf8_lossy(&commit_command.stderr)
        );
        exit(1);
    }

    // Create a new command to run /git push $REMOTE $BRANCH/
    let push_command = Command::new("git")
        .arg("push")
        .arg(remote)
        .arg(branch)
        .output()
        .expect("Failed to push files");

    // Check if the command was successful
    if !push_command.status.success() {
        eprintln!(
            "Failed to push files: {}",
            String::from_utf8_lossy(&push_command.stderr)
        );
        exit(1);
    }

    println!("Successfully pushed changes to remote repository");
}

fn main() {
    // params should be passed in the cli as arguments and we get them using std::env::args()
    let args: Vec<String> = args().collect();
    let default_remote = String::from("origin");
    let default_branch = String::from("main");

 
    let commit_message = &args[1];
    let remote = args.get(2).unwrap_or(&default_remote);
    let branch = args.get(3).unwrap_or(&default_branch);

    // Do the validation of the arguments here
    if commit_message.is_empty() {
        eprintln!("Commit message cannot be empty");
        exit(1);
    }

    // check if the remote or branch name contain invalid characters
    if !check_for_invalid_characters(remote) {
        eprintln!("Remote name contains invalid characters");
        exit(1);
    }

    if !check_for_invalid_characters(branch) {
        eprintln!("Branch name contains invalid characters");
        exit(1);
    }

    update_commit_push(commit_message, remote, branch)
}


fn check_for_invalid_characters(name:&str) -> bool {
    if name.contains(|c:char|  !(c.is_alphanumeric() || c == '-' || c == '_')) {
        return false
    }
    true
}