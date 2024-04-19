use git2::{Repository, BranchType};
use clap::Parser;

#[derive(Parser)]
#[command(name = "branch_cleanser")]
#[command(version = "1.0")]
#[command(about = "Cleanses your branches", long_about = None)]
struct Cli {

    /// Path to the git repo you'd like to cleanse
    #[arg(short, long)]
    git_repo_path: String,

    /// A comma separated list of branches you'd like to ignore
    #[arg(short, long)]
    ignore_branches: String
}

fn main() {
    let cli = Cli::parse();

    let repo = match Repository::open(cli.git_repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };
    let branches = repo.branches(Some(BranchType::Local)).unwrap();

    for branch_iterator in branches {
        let (mut branch, _) = branch_iterator.unwrap();
        let branch_name = branch.name().unwrap().unwrap();

        if branch_name != "fnacarellidev-cgi" {
            println!("Deleting branch {}", branch_name);
            let _ = branch.delete();
        }
    }
}
