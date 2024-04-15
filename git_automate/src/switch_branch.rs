use std::process::{exit, Command};

pub fn switch_branch(
    branch_name:&str
) {
    let switch_branch_command = Command::new("git")
        .arg("checkout")
        .arg(branch_name)
        .output()
        .expect("Failed to switch branch");

    if !switch_branch_command.status.success() {
        eprintln!(
            "Failed to switch branch: {}",
            String::from_utf8_lossy(&switch_branch_command.stderr)
        );
        exit(1);
    }

    println!("Successfully switched to branch: {}", branch_name);
}