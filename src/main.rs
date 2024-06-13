use git2::build::CheckoutBuilder;
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
    

    // let branches = repo.branches(None).unwrap();


    // for branch in branches {
    //     let (branch, branch_type) = branch.unwrap();
    //     let branch_name = branch.name().unwrap();
    //     // dbg!(branch_name.unwrap());
        
    // }

    let reference = repo.revparse_single("simple").unwrap();
    let mut checkout_builder: CheckoutBuilder = CheckoutBuilder::new();
    repo.checkout_tree(&reference, Some(&mut checkout_builder)).unwrap();
    repo.set_head("simple").unwrap();
    dbg!(reference);

    return;
}
