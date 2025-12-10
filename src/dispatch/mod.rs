use std::fmt::Debug;

use clap::Args;
use serde::{Serialize, de::DeserializeOwned};

use crate::{cli::Operations, context::Context};

pub trait Dispatcher {
    type Args: Serialize + DeserializeOwned + Clone + Debug + Args;
    fn dispatch(context: Context, args: Self::Args) -> crate::Result<()>;
}

mod completions;

pub fn dispatch(context: Context) -> crate::Result<()> {
    match context.operation.clone() {
        Operations::Completions(args) => {
            completions::CompletionsDispatcher::dispatch(context.clone(), args)
        }
        _ => unimplemented!(),
    }
}
