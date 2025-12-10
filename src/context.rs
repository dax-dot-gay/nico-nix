use std::{fmt::Display, ops::Deref, sync::Arc};

use clap::{Command, CommandFactory, Parser, error::ErrorKind};
use parking_lot::Mutex;

use crate::cli::Cli;

#[derive(Clone, Debug)]
pub struct Context {
    input: Cli,
    command: Arc<Mutex<Command>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            input: Cli::parse(),
            command: Arc::new(Mutex::new(Cli::command())),
        }
    }

    #[allow(dead_code)]
    pub fn error(&self, kind: ErrorKind, details: impl Display) -> crate::Error {
        let mut cmd = self.command.lock();
        cmd.error(kind, details).into()
    }
}

impl Deref for Context {
    type Target = Cli;
    fn deref(&self) -> &Self::Target {
        &self.input
    }
}
