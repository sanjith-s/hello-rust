use git2::Branch;
use git2::BranchType;
use git2::Oid;
use git2::Repository;
use git2::StatusOptions;
use std::path::Path;
use std::collections::HashMap;
use serde_json::json;
use chrono::DateTime;

fn is_git_repo(proj_path: String) -> bool {
    let path = Path::new(&proj_path);
    let parent = path.parent();
    let mut ceiling_dir = vec![];
    if parent != None {
        ceiling_dir.push(parent.unwrap());
    }

    let repo_path = match Repository::discover_path(path, ceiling_dir) {
        Ok(repo_path) => repo_path,
        Err(_err) => {
            return false;
        }
    };

    println!("Repo  path: {:?}", repo_path);
    return true;
}


fn main() {

    let repo = match Repository::open("C:\\Users\\sanji\\simple-rs\\hello-rust") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    

    // print!("{:?}", repo.merge_base( Oid::from_str("502d8092d1124c272a91d9771dc03f6d3311b416").unwrap(), Oid::from_str("719ec3594b3c6a0070dae4df687bd40bf30e7c6a").unwrap()).unwrap());
    let mut rev_walk = repo.revwalk().unwrap();
    rev_walk.push(Oid::from_str("a7a0251b416e56ba021dc3822ac7013c6a17695a").unwrap()).unwrap();
    rev_walk.push_head().unwrap();
    rev_walk.push_range("a7a0251b416e56ba021dc3822ac7013c6a17695a..HEAD").unwrap();
    rev_walk.simplify_first_parent().unwrap();
    rev_walk.set_sorting(git2::Sort::TOPOLOGICAL).unwrap();

    for commit in rev_walk {
        let commit = commit.unwrap();
        let commit = repo.find_commit(commit).unwrap();
        let commit_id = commit.id();
        let commit_id = commit_id.to_string();
        let commit_message = commit.message().unwrap();
        let commit_message = commit_message.trim();
        let commit_author = commit.author();
        let commit_author = commit_author.name().unwrap();
        let commit_time = commit.time();
        let commit_time = commit_time.seconds();
        let commit_time = DateTime::from_timestamp(commit_time, 0).unwrap();
        println!("commit_id: {}, commit_message: {}, commit_author: {}, commit_time: {}", commit_id, commit_message, commit_author, commit_time);
    }

    return;
}
