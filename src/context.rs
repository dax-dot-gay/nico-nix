use std::{fmt::Display, ops::Deref, sync::Arc};

use clap::{Command, CommandFactory, Parser, error::ErrorKind};
use handlebars::Handlebars;
use include_directory::{Dir, include_directory};
use log::{debug, trace};
use parking_lot::{Mutex, RwLock};
use serde::Serialize;

use crate::cli::Cli;

static TEMPLATES: Dir<'_> = include_directory!("$CARGO_MANIFEST_DIR/templates");

#[derive(Clone, Debug)]
pub struct Context {
    input: Cli,
    command: Arc<Mutex<Command>>,
    templater: Arc<RwLock<Handlebars<'static>>>
}

impl Context {
    pub fn new() -> Self {
        let mut templater = Handlebars::new();

        debug!("Loading templates...");
        for file in TEMPLATES.find("**/*.template").unwrap().filter_map(|v| v.as_file()) {
            let path = file.path().to_str().unwrap().to_string();
            let key = path.rsplit_once(".").unwrap().0.to_string();

            trace!("Adding template with key {key}.");
            let content = file.contents_utf8().unwrap();
            let cleaned = if key.ends_with(".nix") {
                content.replace("#! ", "")
            } else {content.to_string()};
            templater.register_template_string(&key, &cleaned).expect("Failed to load internal template.");
        }

        Self {
            input: Cli::parse(),
            command: Arc::new(Mutex::new(Cli::command())),
            templater: Arc::new(RwLock::new(templater))
        }
    }

    pub fn error(&self, kind: ErrorKind, details: impl Display) -> crate::Error {
        let mut cmd = self.command.lock();
        cmd.error(kind, details).into()
    }

    pub fn render_template<T: Serialize>(&self, name: impl AsRef<str>, data: &T) -> crate::Result<String> {
        Ok(self.templater.read().render(name.as_ref(), data)?)
    }
}

impl Deref for Context {
    type Target = Cli;
    fn deref(&self) -> &Self::Target {
        &self.input
    }
}
