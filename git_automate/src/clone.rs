use std::process::{exit, Command};


pub fn clone(
   url: &str
) {
    // create a new command to run /git clone $URL/
    let clone_command = Command::new("git")
        .arg("clone")
        .arg(url)
        .output()
        .expect("Failed to clone repository");

    // check if the command was successful
    if !clone_command.status.success() {
        eprintln!(
            "Failed to clone repository: {}",
            String::from_utf8_lossy(&clone_command.stderr)
        );
        exit(1);
    }
}