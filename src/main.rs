extern crate git2;
extern crate gitutils;

use gitutils::{ get_repo, get_head };
use git2::{StatusOptions};

fn main() {
    let path = "/Users/amukiza/code/learning/rust/gitutils";

    let repo = get_repo(path);

    let mut options = StatusOptions::new();
    options.include_ignored(true);

    let head = get_head(&repo);

    let head = head.as_ref().and_then(|h| h.shorthand());

   println!("# On branch {}", head.unwrap_or("Not currently on any branch"));
}
