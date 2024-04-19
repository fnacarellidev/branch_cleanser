use git2::Repository;
use git2::BranchType;

fn main() {
    let repo = match Repository::open("/home/fnacarelli/42-Projects/42-webserv") {
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
