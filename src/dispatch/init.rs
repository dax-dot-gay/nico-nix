use crate::{cli::InitArgs, config::Configuration, context::Context, dispatch::Dispatcher};
use git2::{Repository, build::RepoBuilder};
use log::*;
use std::{fs, path::PathBuf};

fn directory_setup(context: Context, args: InitArgs) -> crate::Result<(PathBuf, String)> {
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
    if let Some(remote) = args.git_remote.clone() {
        
    }
}

pub struct InitDispatcher;
impl Dispatcher for InitDispatcher {
    type Args = InitArgs;
    fn dispatch(context: Context, args: Self::Args) -> crate::Result<()> {
        let target_folder = target_folder.canonicalize()?;

        debug!("Ensuring git setup...");
        let git_remote = if let Some(git_remote) = args.git_remote.clone() {
            let repo = RepoBuilder::new().
        } else {

        };

        debug!(
            "Writing configuration to {:?}",
            target_folder.join("nico.config.json")
        );
        let config = Configuration::new(target_folder.clone(), args)?;
        trace!("Config data: {config:?}");
        debug!("Writing flake.nix.");

        let rendered = config.render_flake(context.clone())?;
        fs::write(target_folder.join("flake.nix"), rendered)?;

        Ok(())
    }
}
