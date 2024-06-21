use git2::build::CheckoutBuilder;
use git2::Branch;
use git2::BranchType;
use git2::CredentialHelper;
use git2::ErrorCode;
use git2::Oid;
use git2::Repository;
use git2::StatusOptions;
use std::iter;
use std::path::Path;
use std::collections::HashMap;
use serde_json::json;
use chrono::DateTime;
use git2::RemoteCallbacks;
use git2::Cred;

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

fn create_callbacks(repo: &Repository) -> RemoteCallbacks {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::credential_helper(&repo.config().unwrap(), "https://github.com/sanjith-s/hello-rust.git", None)});

    // &callbacks.credentials(|_str, _str_opt, _cred_type| {
    //     Cred::credential_helper(&repo.config().unwrap(), "https://github.com/sanjith-s/hello-rust.git", None)
    // });
    callbacks
}

fn main() {

    let repo = match Repository::open("C:\\Users\\sanji\\simple-rs\\hello-rust") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    
    
    let repo_remotes = repo.remotes().unwrap();

    let found_origin = repo_remotes.iter().any(|remote| {
        remote.unwrap() == "origin"
    });
    
    if !found_origin {
        println!("No origin remote found");
        return;
    }

    let mut remote_obj = repo.find_remote("origin").unwrap();
    dbg!(remote_obj.url().unwrap());

    let mut local_branch = repo.find_branch("simple", git2::BranchType::Local).unwrap();
    let remote_branch = match local_branch.upstream() {
        Ok(branch) => {},
        Err(e) => {

            if e.code() == ErrorCode::NotFound {
                local_branch.set_upstream(Some("simple")).unwrap();
            } else {
                let json_str = json!({
                    "code": e.code() as i32,
                    "message": "Remote upstream branch not found",
                }).to_string();
                dbg!(json_str);
                return;
            }
           
        }
    };
    

    // Push
    let mut fetch_options = git2::FetchOptions::default();
    let callbacks = create_callbacks(&repo);
    // callbacks.push_update_reference(|reference,error|{
    //     println!("ref = {}, error = {:?}", reference, error);
    //     Ok(())
    // });
    fetch_options.remote_callbacks(callbacks);

    remote_obj.fetch(&["main"], None, None).unwrap();

    let fetch_head = repo.refname_to_id("FETCH_HEAD").unwrap();
    let origin_main = repo.refname_to_id("refs/remotes/origin/main").unwrap();

    dbg!(fetch_head, origin_main);
    
 
    return;
}
