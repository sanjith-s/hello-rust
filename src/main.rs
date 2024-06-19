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

    let cfg = repo.config().unwrap();

    // let mut entries =cfg.entries(None).unwrap();
    // while let Some(entry) = entries.next() {
    //     let entry = entry.unwrap();
    //     println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
    // }
    let name = cfg.get_string("user.name").unwrap();

    let email = cfg.get_string("user.email").unwrap();

    let mut index = repo.index().unwrap();
    println!("{:?}", index.len());

    let index_iter = index.iter();
    for entry in index_iter {
        dbg!(entry);
    }
    
    let index_tree_id = index.write_tree().unwrap();
    let index_tree = repo.find_tree(index_tree_id).unwrap();
    let parent = repo.head().unwrap().peel_to_commit().unwrap();
    let commit_message = "My Libgit commit";
    let update_ref = "HEAD";

    let signature = git2::Signature::now(&name, &email).unwrap();
    let _new_commit_id = match repo.commit(Some(update_ref),&signature, &signature, commit_message, &index_tree, &[&parent]) {
        Ok(oid) => oid,
        Err(e) => {
            let json_str = json!(
                {
                    "code": e.raw_code(),
                    "message": e.message()
                }
            ).to_string();
        
            return;
        },
    };


    // return json!({
    //     "code": 0,
    //     "message": "Success",
    // }).to_string();
    // print!("User Name: {:?}. User Email {:?}", , cfg.get_str("user.email").unwrap());
    // repo.set_index(&mut index).unwrap();
    
    // dbg!(new_branch);

    return;
}
