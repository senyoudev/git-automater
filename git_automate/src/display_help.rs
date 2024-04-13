

pub fn display_help() {
    println!("Supported commands:");
    println!("git_automate pushFromOne <commit_message> [<remote>] [<branch>] - Add, commit, and push changes to a Git repository, with an optional remote and branch");
    println!("git_automate add [<src_of_excluded_files_separated_with_comma>] - Add all files to the staging area, with an optional list of files to exclude");
    println!("git_automate commit [<commit_message>] - Commit all files in the staging area, with an optional commit message");
    println!("git_automate push [<remote>] [<branch>] - Push all committed changes to a remote repository, with an optional remote and branch");
    println!("git_automate clone <url> - Clone a Git repository from a URL");
    println!("git_automate help - Display this help message");

}