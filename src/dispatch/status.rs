use colored::Colorize;

use crate::{cli::StatusArgs, config::GitRemote, dispatch::Dispatcher};

pub struct StatusDispatcher;
impl Dispatcher for StatusDispatcher {
    type Args = StatusArgs;
    fn dispatch(context: crate::context::Context, _: Self::Args) -> crate::Result<()> {
        println!("{}\t\t{}", "Project Path:".bright_white().bold(), context.project_root().unwrap().to_str().unwrap());

        let config = context.config().unwrap();
        println!("{}\t\tnixpkgs/nixos-{}", "Nix Branch:".bright_white().bold(), config.init.nix.clone());
        println!("{}\t{}", "Target Architecture:".bright_white().bold(), config.init.system.clone());
        println!("{}", "Remotes:".bright_white().bold());
        for (name, GitRemote {url, ..}) in config.resources.remotes.clone() {
            println!("  - {name}: {}", url.italic());
        }
        Ok(())
    }
}
