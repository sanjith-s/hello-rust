use git2::build::CheckoutBuilder;
use git2::Branch;
use git2::BranchType;
use git2::CredentialHelper;
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
        Cred::credential_helper(&repo.config().unwrap(), "https://github.com/sanjith-s/hello-rust.git", Some("sanjith-s"))});

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
    
    // let cred = git2::Cred::credential_helper(&repo.config().unwrap(), "https://github.com/sanjith-s/hello-rust.git", Some("sanjith-s")).unwrap();
    // dbg!(cred.credtype());

    // let mut cred_helper = git2::CredentialHelper::new("https://github.com/sanjith-s/hello-rust.git");
    // // cred_helper.username(Some("sanjith-s")); 
    // let result = cred_helper.execute().unwrap();
    // // if result.is_some() {
    //     dbg!(result);
    // }
    
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

    // remote_obj.fetch(&["main"], None, None);

    // remote_obj.push(&[&"main"], None).unwrap();
    
    // match remote_obj.connect_auth(git2::Direction::Push, Some(create_callbacks(&repo)), None) {
    //     Ok(_) => println!("Connected to remote"),
    //     Err(e) => {
    //         println!("Failed to connect to remote: {:?}", e.code());
    //         return;
    //     }
    
    // };
    let mut push_options = git2::PushOptions::default();
    let mut callbacks = create_callbacks(&repo);
    callbacks.push_update_reference(|reference,error|{
        println!("ref = {}, error = {:?}", reference, error);
        Ok(())
    });
    push_options.remote_callbacks(callbacks);

    remote_obj.push(&["refs/heads/main:refs/heads/main"], Some(&mut push_options)).unwrap();
    // dbg!(remote_obj.connected());
    
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
