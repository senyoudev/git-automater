use std::process::{Command, exit};
use std::env::args;
use std::io::Write;


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
        eprintln!("Failed to add files : {}",
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
        eprintln!("Failed to commit files: {}", String::from_utf8_lossy(&commit_command.stderr));
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
        eprintln!("Failed to push files: {}", String::from_utf8_lossy(&push_command.stderr));
        exit(1);
    }

    println!("Successfully pushed changes to remote repository");

}

fn main() {
    // params should be passed in the cli as arguments and we get them using std::env::args()
    let args: Vec<String> = args().collect();

    // Check if the number of arguments is correct
    if args.len() != 4 {
        eprintln!("Usage: git-auto-push <commit_message> <remote> <branch>");
        exit(1);
    }

    let commit_message = &args[1];
    let remote = &args[2];
    let branch = &args[3];

    update_commit_push(commit_message, remote, branch)
    
}
