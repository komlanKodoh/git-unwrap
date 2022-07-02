use clap::Parser;
use fs_extra::dir::{copy, remove, CopyOptions};
use git2::{BranchType, Error, Repository};
use std::{
    env,
    path::{Path, PathBuf},
};

const ABOUT: &str = "Clone repository creating a folder per remote branch";

#[derive(Debug, Parser)]
#[clap(about= ABOUT)]
pub struct CloneConfig {
    /// URL to repository that you want to copy. ssh is not yet supported 
    pub repo: String,

    /// Destination folder within which all file are created
    #[clap(default_value_t = String::from("."), value_parser)]
    pub folder: String,
}

pub fn clone(config: &mut CloneConfig) {
    let dest_folder = env::current_dir()
        .unwrap()
        .join(&config.folder)
        .join(extra_repo_name(&config.repo));
    let repo_url = &config.repo;

    let repo = match Repository::clone(repo_url, &dest_folder.join("temp")) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    let branches = get_remote_branches(&repo);

    for (branch, _) in branches {
        let name = branch.name().unwrap().unwrap();
        println!("working on branch {}", name);

        match checkout_branch(&repo, name) {
            Ok(_) => (),
            Err(_) => {
                print!(" Failed to checkout branch {}", name);
                continue;
            }
        };

        copy_folder(&dest_folder.join("temp"), &dest_folder.join(&name[7..]))
            .expect("Could not copy file content");
    }

    remove(&dest_folder.join("temp")).unwrap();
}

fn copy_folder(src: &PathBuf, dest: &PathBuf) -> Result<u64, fs_extra::error::Error> {
    copy(
        src,
        dest,
        &CopyOptions {
            overwrite: true,
            skip_exist: true,
            buffer_size: 64000,
            copy_inside: true,
            content_only: true,
            depth: 0,
        },
    )
}

fn checkout_branch(repo: &Repository, branchname: &str) -> Result<(), Error> {
    let (object, reference) = repo.revparse_ext(branchname).expect("Object not found");

    repo.checkout_tree(&object, None)?;

    match reference {
        Some(gref) => repo.set_head(gref.name().unwrap()),
        None => repo.set_head_detached(object.id()),
    }?;

    Ok(())
}

fn get_remote_branches(repo: &Repository) -> Vec<(git2::Branch, BranchType)> {
    repo.branches(Some(BranchType::Remote))
        .expect("Error: Could not fetch branches")
        .filter_map(|x| x.ok())
        .collect()
}

fn extra_repo_name(path: &str) -> &str {
    Path::new(path).file_stem().unwrap().to_str().unwrap()
}
