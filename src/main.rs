extern crate git2;

use git2::{Repository, StatusOptions, ErrorCode, Reference};

fn main() {
    let path = "/Users/amukiza/code/learning/rust/git-move";

    let repo = get_repo(path);

    let mut options = StatusOptions::new();
    options.include_ignored(true);

    let head = get_head(&repo);

    let head = head.as_ref().and_then(|h| h.shorthand());

   println!("# On branch {}", head.unwrap_or("Not currently on any branch"));
}

fn get_repo(path: &'static str) -> Repository {
    match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("{}", e),
    }
}

fn get_head(repo: &Repository) -> Option<Reference> {
   match repo.head() {
       Ok(head) => Some(head),
       Err(ref e) if e.code() == ErrorCode::UnbornBranch ||
           e.code() == ErrorCode::NotFound => None,
       Err(_) => None
   }
}
