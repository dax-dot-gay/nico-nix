pub(crate) mod cli;
pub(crate) mod context;
mod error;
use std::process::Command;

pub(crate) use error::{Error, Result};
pub(crate) mod config;
pub(crate) mod dispatch;
pub(crate) mod repo;

use context::Context;
use log::{debug, info};

fn ensure_dependency(command: impl AsRef<str>, args: impl IntoIterator<Item = impl AsRef<str>>) -> Result<String> {
    let command = command.as_ref().to_string();
    let args: Vec<String> = args.into_iter().map(|v| v.as_ref().to_string()).collect();

    debug!("Confirming dependency installation of {}: Running {}", command.clone(), {
        let mut full = vec![command.clone()];
        full.extend(args.clone());
        full.join(" ")
    });
    let output = Command::new(command.clone()).args(args).output()?;
    if output.status.success() {
        let result = String::from_utf8(output.stdout).expect("Dependency check returned non-UTF8 output!").trim().to_string();
        info!("Confirmed host dependency {command}: {result}");
        Ok(result)
    } else {
        let code = output.status.code().unwrap_or(127);
        if code == 127 {
            Err(Error::dependency(command))
        } else {
            panic!("CRITICAL: Attempting to determine dependency version for {command} failed with code {code}! This indicates the dependency is installed but may be corrupted.")
        }
    }
}

fn main() -> Result<()> {
    let context = Context::new()?;
    
    if let Some(ctx) = context {
        env_logger::Builder::new()
            .filter_level(ctx.verbosity.clone().into())
            .init();

        let _nix_version = ensure_dependency("nix", ["--version"])?;
        let _git_version = ensure_dependency("git", ["--version"])?;
        dispatch::dispatch(ctx)
    } else {
        Ok(())
    }
}
