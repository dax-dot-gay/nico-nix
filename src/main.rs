pub(crate) mod cli;
pub(crate) mod context;
mod error;
pub(crate) use error::{Error, Result};
pub(crate) mod config;
pub(crate) mod dispatch;

use context::Context;

fn main() -> Result<()> {
    let context = Context::new();
    env_logger::Builder::new().filter_level(context.verbosity.clone().into()).init();

    dispatch::dispatch(context)
}
