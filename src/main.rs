use clap::Parser;
use dialoguer::Confirm;
use git2::{BranchType, Branches, Repository};

#[derive(Parser)]
#[command(name = "branch_cleanser")]
#[command(version = "1.0")]
#[command(about = "Cleanses your branches", long_about = None)]
struct Cli {

    /// Path to the git repo you'd like to cleanse
    #[arg(short = 'p', long = "path", required = true, value_name = "/path/to/git/repo")]
    git_repo_path: String,

    /// Branch you'd like to ignore
    #[arg(short, long = "ignore", value_name = "branch to ignore")]
    ignore_branches: Vec<String>
}

fn proceed_with_deletion() -> bool {
    let confirmation = Confirm::new()
        .with_prompt("Are you sure you wish to continue? Make sure you don't have any work you'd lost while deleting the branches.")
        .default(false)
        .interact()
        .unwrap();

    confirmation
}

fn cleanse_branches(branches: Branches, ignore_branches: Vec<String> ) {
    for branch_iterator in branches {
        let (mut branch, _) = branch_iterator.unwrap();
        let branch_name = &branch.name().unwrap().unwrap().to_owned();

        if !ignore_branches.contains(branch_name) {
            println!("Cleansing branch {}", branch_name);
            let _ = branch.delete();
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let repo = match Repository::open(cli.git_repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };
    let branches = repo.branches(Some(BranchType::Local)).unwrap();

    if proceed_with_deletion() {
        cleanse_branches(branches, cli.ignore_branches);
    }

}
