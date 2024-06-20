use git2::build::CheckoutBuilder;
use git2::Branch;
use git2::BranchType;
use git2::Oid;
use git2::Repository;
use git2::StatusOptions;
use std::iter;
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
    

    let config  = repo.config().unwrap();

    let mut entries = config.entries(None).unwrap();
    while let Some(entry) = entries.next() {
        let entry = entry.unwrap();
        println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
    }    
    
    // let repo_remotes = repo.remotes().unwrap();

    // let found_origin = repo_remotes.iter().any(|remote| {
    //     remote.unwrap() == "origin"
    // });
    
    // if !found_origin {
    //     println!("No origin remote found");
    //     return;
    // }

    // let mut remote_obj = repo.find_remote("origin").unwrap();
    // dbg!(remote_obj.url().unwrap());

    // match remote_obj.connect_auth(git2::Direction::Push, None, None) {
    //     Ok(_) => println!("Connected to remote"),
    //     Err(e) => {
    //         println!("Failed to connect to remote: {:?}", e.code());
    //         return;
    //     }
    
    // };
    
    // if remote_obj.connected() {

    //     let ref_specs = remote_obj.refspecs();

    //     for spec in ref_specs {
    //         println!("Ref spec: {:?}", spec.str());
    //     }
    //     println!("Connected to remote");
    // } else {
    //     println!("Not connected to remote");
    // }

    return;
}
