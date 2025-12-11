pub(crate) mod cli;
pub(crate) mod context;
mod error;
use std::process::Command;

pub(crate) use error::{Error, Result};
pub(crate) mod config;
pub(crate) mod dispatch;

use context::Context;
use log::{debug, info};

fn main() -> Result<()> {
    let context = Context::new();
    env_logger::Builder::new()
        .filter_level(context.verbosity.clone().into())
        .init();

    debug!("Confirming local nix installation...");
    let output = Command::new("nix").arg("--version").output()?;
    if output.status.success() {
        info!(
            "Local Nix installation: {}",
            String::from_utf8(output.stdout).expect("nix --version returned non-UTF8 output!")
        );
        dispatch::dispatch(context)
    } else {
        let code = output.status.code().unwrap_or(127);
        if code == 127 {
            Err(Error::dependency("nix"))
        } else {
            panic!(
                "CRITICAL: Nix (or a command registered as `nix`) exists, but returns code {code} when executed. Your installation is corrupt."
            );
        }
    }
}
