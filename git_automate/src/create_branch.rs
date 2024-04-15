use std::process::{exit, Command};


pub fn createBranch(branch_name:&str) {
    let create_branch_command = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output()
        .expect("Failed to create branch");

    if !create_branch_command.status.success() {
        eprintln!(
            "Failed to create branch: {}",
            String::from_utf8_lossy(&create_branch_command.stderr)
        );
        exit(1);
    }

    println!("Successfully created branch: {}", branch_name);
}