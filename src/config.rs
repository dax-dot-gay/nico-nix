#![allow(dead_code)]

use std::{
    clone, fs, path::{Path, PathBuf}
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{cli::InitArgs, context::Context};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitConfig {
    pub description: String,
    pub nix: String,
    pub system: String,
    pub sops_url: String,
    pub comin_url: String,
    pub git_remote: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExtraFlake {
    pub ident: String,
    pub url: String,
    pub follows: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Resources {
    pub extra_flakes: Vec<ExtraFlake>,
    pub dev_packages: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum GitRemote {
    Local { name: String, url: String },
    Http { name: String, url: String },
    Ssh { name: String, url: String },
    Git { name: String, url: String }
}

impl GitRemote {
    pub fn parse(name: impl AsRef<str>, uri: impl AsRef<str>) -> crate::Result<Self> {
        let name = name.as_ref().to_string();
        let url = uri.as_ref().to_string();

        if !url.trim_end_matches("/").ends_with(".git") {
            return Err(crate::Error::url(url, "Git URLs should end with .git"));
        }

        Ok(if url.starts_with("https://") || url.starts_with("http://") {
            Self::Http { name, url }
        } else if url.starts_with("ssh://") || url.starts_with("rsync://") || name.contains("@") {
            Self::Ssh { name, url }
        } else if url.starts_with("git://") {
            Self::Git { name, url }
        } else {
            Self::Local { name, url }
        })
    }

    pub fn url(&self) -> String {
        match self {
            GitRemote::Local { url, .. } => url.clone(),
            GitRemote::Http { url, .. } => url.clone(),
            GitRemote::Ssh { url, .. } => url.clone(),
            GitRemote::Git { url, .. } => url.clone(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            GitRemote::Local { name, .. } => name.clone(),
            GitRemote::Http { name, .. } => name.clone(),
            GitRemote::Ssh { name, .. } => name.clone(),
            GitRemote::Git { name, .. } => name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    pub init: InitConfig,
    pub resources: Resources,
}

impl Configuration {
    pub fn new(root: PathBuf, init: InitArgs, remote: String) -> crate::Result<Self> {
        let new_config = Self {
            init: InitConfig {
                description: init.description.clone(),
                nix: init.nix.clone(),
                system: init.system.clone(),
                sops_url: init.sops_url.clone(),
                comin_url: init.comin_url.clone(),
                git_remote: remote
            },
            resources: Resources::default(),
        };
        new_config.save(root)?;
        Ok(new_config)
    }

    pub fn load() -> crate::Result<Self> {
        let mut current = std::env::current_dir()?;
        loop {
            if current.join("nico.config.json").exists() {
                let raw_config = fs::read_to_string(current.join("nico.config.json"))?;
                return Ok(serde_json::from_str::<Self>(&raw_config)?);
            } else if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                return Err(crate::Error::ConfigNotFound);
            }
        }
    }

    pub fn load_path(context: Context, path: impl AsRef<Path>) -> crate::Result<Self> {
        let path = path.as_ref();
        let config_path = (if path.is_dir() && path.join("nico.config.json").exists() {
            Ok(path.join("nico.config.json"))
        }
        else if path.is_file() && path.ends_with("nico.config.json") {
            Ok(path.to_path_buf())
        } else {
            Err(context.error(clap::error::ErrorKind::ValueValidation, format!("The supplied configuration file (must be either a directory containing `nico.config.json` or an existing `nico.config.json` file.")))
        })?.canonicalize()?;
        let deserialized = serde_json::from_str::<Self>(&fs::read_to_string(config_path.clone())?)?;
        Ok(deserialized)
    }

    pub fn save(&self, directory: impl AsRef<Path>) -> crate::Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
        fs::write(directory.as_ref().join("nico.config.json"), serialized)?;
        Ok(())
    }

    pub fn render_flake(&self, context: Context) -> crate::Result<String> {
        let extra_flakes = ""; // TODO: Flake management
        let dev_packages = ""; // TODO: Extra dev pkgs

        let data = json!({
           "init": {
                "description": self.init.description.clone(),
                "nix": self.init.nix.clone(),
                "sops_url": self.init.sops_url.clone(),
                "comin_url": self.init.comin_url.clone(),
                "system": self.init.system.clone()
           },
           "resources": {
                "extra_flakes": extra_flakes,
                "dev_packages": dev_packages
           }
        });

        context.render_template("flake/root.nix", &data)
    }
}
