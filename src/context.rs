use std::{fmt::Display, ops::Deref, path::PathBuf, sync::Arc};

use clap::{Command, CommandFactory, Parser, error::ErrorKind};
use handlebars::Handlebars;
use include_directory::{Dir, include_directory};
use log::{debug, trace};
use parking_lot::{Mutex, RwLock};
use serde::Serialize;

use crate::{
    cli::{Cli, Operations},
    config::Configuration,
};

static TEMPLATES: Dir<'_> = include_directory!("$CARGO_MANIFEST_DIR/templates");

#[derive(Clone, Debug)]
pub struct Context {
    input: Cli,
    command: Arc<Mutex<Command>>,
    templater: Arc<RwLock<Handlebars<'static>>>,
    config: Option<Configuration>,
    project_root: Option<PathBuf>,
}

impl Context {
    pub fn new() -> crate::Result<Option<Self>> {
        let parsed = Cli::parse();
        if let Some(project) = parsed.project.clone() {
            if !parsed.ignore_project {
                let project = project.to_str().unwrap().to_string();
                let mut args = vec![
                    "develop".to_string(),
                    project.clone(),
                    "--command".to_string(),
                    std::env::current_exe()?.to_str().unwrap().to_string(),
                    "--ignore-project".to_string(),
                ];
                args.extend(std::env::args().collect::<Vec<String>>()[1..].to_vec());
                std::process::Command::new("nix")
                    .args(args)
                    .env("NICO_OVERRIDE_ENV", project.clone())
                    .current_dir(project.clone())
                    .status()?;
                return Ok(None);
            }
        }

        let mut templater = Handlebars::new();

        debug!("Loading templates...");
        for file in TEMPLATES
            .find("**/*.template")
            .unwrap()
            .filter_map(|v| v.as_file())
        {
            let path = file.path().to_str().unwrap().to_string();
            let key = path.rsplit_once(".").unwrap().0.to_string();

            trace!("Adding template with key {key}.");
            let content = file.contents_utf8().unwrap();
            let cleaned = if key.ends_with(".nix") {
                content.replace("#! ", "")
            } else {
                content.to_string()
            };
            templater
                .register_template_string(&key, &cleaned)
                .expect("Failed to load internal template.");
        }

        let (config, project_root) = match parsed.operation.clone() {
            Operations::Init(_) | Operations::Completions(_) => (None, None),
            #[allow(unused)]
            _ => {
                if let Ok(env_path) = std::env::var("NICO_ENV") {
                    (
                        Some(Configuration::load_path(env_path.clone())?),
                        Some(PathBuf::from(env_path)),
                    )
                } else {
                    return Err(crate::Error::OutsideShell);
                }
            }
        };

        Ok(Some(Self {
            input: parsed,
            command: Arc::new(Mutex::new(Cli::command())),
            templater: Arc::new(RwLock::new(templater)),
            config,
            project_root,
        }))
    }

    pub fn error(&self, kind: ErrorKind, details: impl Display) -> crate::Error {
        let mut cmd = self.command.lock();
        cmd.error(kind, details).into()
    }

    pub fn render_template<T: Serialize>(
        &self,
        name: impl AsRef<str>,
        data: &T,
    ) -> crate::Result<String> {
        Ok(self.templater.read().render(name.as_ref(), data)?)
    }

    pub fn config(&self) -> Option<Configuration> {
        self.config.clone()
    }

    pub fn project_root(&self) -> Option<PathBuf> {
        self.project_root.clone()
    }
}

impl Deref for Context {
    type Target = Cli;
    fn deref(&self) -> &Self::Target {
        &self.input
    }
}
