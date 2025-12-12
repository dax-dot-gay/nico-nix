use crate::{
    cli::InitArgs,
    config::{Configuration, GitRemote},
    context::Context,
    dispatch::Dispatcher, repo::RepoExt,
};
use clap::error::ErrorKind;
use git2::{
    FetchOptions, Repository,
    build::{CheckoutBuilder, RepoBuilder},
};
use log::*;
use std::{fs, path::PathBuf};

fn directory_setup(context: Context, args: InitArgs) -> crate::Result<(PathBuf, Vec<GitRemote>, Repository)> {
    let target_folder = args
        .clone()
        .path
        .and_then(|v| Some(PathBuf::from(v)))
        .unwrap_or(std::env::current_dir()?);

    info!("Initializing a project at {target_folder:?}");

    if target_folder.exists() && !target_folder.is_dir() {
        return Err(context.error(
            clap::error::ErrorKind::ValueValidation,
            "Initialization path must be a directory.",
        ));
    }

    if !target_folder.exists() {
        fs::create_dir_all(target_folder.clone())?;
    }

    let target_folder = target_folder.canonicalize()?;

    if args.git.local {
        if target_folder.join(".git").exists() {
            Err(context.error(ErrorKind::ValueValidation, "Attempting to create a new local git repository, but the target directory already contains one."))?;
        }

        let repo = Repository::init(target_folder.clone())?;
        repo.create_initial_commit()?;

        Ok((
            target_folder.clone(),
            vec![GitRemote::builder("local", target_folder.to_str().unwrap().to_string()).build()],
            repo
        ))
    } else if let Some(git_clone) = args.git.clone.clone() {
        if target_folder.join(".git").exists() {
            Err(context.error(ErrorKind::ValueValidation, "Attempting to clone a git repository, but the target directory already contains one."))?;
        }

        let mut checkout = CheckoutBuilder::new();
        checkout.target_dir(target_folder.as_path());
        let mut fetch = FetchOptions::new();
        fetch.depth(0);

        let repo = RepoBuilder::new()
            .with_checkout(checkout)
            .fetch_options(fetch)
            .clone(&git_clone, target_folder.as_path())?;
        Ok((
            target_folder,
            vec![GitRemote::builder("origin", git_clone).build()],
            repo
        ))
    } else {
        if !target_folder.join(".git").exists() {
            Err(context.error(
                ErrorKind::ValueValidation,
                "Target folder does not contain an existing git repository.",
            ))?;
        }

        let repo = Repository::open(&target_folder)?;
        let mut remotes: Vec<GitRemote> = vec![];
        for remote_name in repo
            .remotes()?
            .into_iter()
            .filter_map(|v| v.and_then(|s| Some(s.to_string())))
        {
            if let Ok(remote) = repo.find_remote(&remote_name) {
                if let Some(name) = remote.name() {
                    if let Some(url) = remote.url() {
                        if let Ok(branch) = remote.default_branch() {
                            remotes.push(
                                GitRemote::builder(name, url)
                                    .main_branch(branch.as_str().unwrap_or("main"))
                                    .build(),
                            );
                        }
                    }
                }
            }
        }

        Ok((target_folder, remotes, repo))
    }
}

pub struct InitDispatcher;
impl Dispatcher for InitDispatcher {
    type Args = InitArgs;
    fn dispatch(context: Context, args: Self::Args) -> crate::Result<()> {
        let (target_folder, remotes, repo) = directory_setup(context.clone(), args.clone())?;

        debug!(
            "Writing configuration to {:?}",
            target_folder.join("nico.config.json")
        );
        let config = Configuration::new(target_folder.clone(), args, remotes)?;
        trace!("Config data: {config:?}");
        debug!("Writing flake.nix.");

        let rendered = config.render_flake(context.clone())?;
        fs::write(target_folder.join("flake.nix"), rendered)?;

        repo.add_files(["."])?;
        repo.create_commit("Nico initialization")?;

        Ok(())
    }
}
