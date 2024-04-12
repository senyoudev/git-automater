use std::process::{exit, Command};



pub fn add(excluded_files: Vec<&str>) {
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

    // Exclude files from the staging area
    for file in excluded_files {
        // Create a new command to run /git reset $FILE/
        let reset_command = Command::new("git")
            .arg("reset")
            .arg(file)
            .output()
            .expect("Failed to reset files");

        // Check if the command was successful
        if !reset_command.status.success() {
            eprintln!(
                "Failed to reset files: {}",
                String::from_utf8_lossy(&reset_command.stderr)
            );
            exit(1);
        }
    }

   

    println!("Successfully added files to staging area");
}

pub fn commit(commit_message: &str) {
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

    println!("Successfully committed changes");
}

pub fn push(remote: &str, branch: &str) {
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



pub fn update_commit_push(excluded_files: Vec<&str>,commit_message: &str, remote: &str, branch: &str) {
    
    add(excluded_files);
    commit(commit_message);
    push(remote, branch);

    println!("Successfully pushed changes to remote repository");
}
