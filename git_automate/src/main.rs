use std::process::{Command, exit};

fn update_commit_push(
    commit_message: &str,
    remote: &str,
    branch: &str
) {
    // Create a new command to run /git add ./
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to add files");

    // Check if the command was successful
    if !add_command.status.success() {
        eprintln!("Failed to add files");
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
        eprintln!("Failed to commit files");
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
        eprintln!("Failed to push files");
        exit(1);
    }

    println!("Successfully pushed changes to remote repository");

}

fn main() {
    
}
