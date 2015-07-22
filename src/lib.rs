extern crate git2;

use git2::{Repository, ErrorCode, Reference};

pub fn get_repo(path: &'static str) -> Repository {
    match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("{}", e),
    }
}

pub fn get_head(repo: &Repository) -> Option<Reference> {
   match repo.head() {
       Ok(head) => Some(head),
       Err(ref e) if e.code() == ErrorCode::UnbornBranch ||
           e.code() == ErrorCode::NotFound => None,
       Err(_) => None
   }
}
