#![allow(dead_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{cli::InitArgs, context::Context};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitConfig {
    pub description: String,
    pub nix: String,
    pub system: String,
    pub sops_url: String,
    pub comin_url: String,
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
pub struct Configuration {
    root_directory: PathBuf,
    pub init: InitConfig,
    pub resources: Resources,
}

impl Configuration {
    pub fn new(root: PathBuf, init: InitArgs) -> crate::Result<Self> {
        let new_config = Self {
            root_directory: root,
            init: InitConfig {
                description: init.description.clone(),
                nix: init.nix.clone(),
                system: init.system.clone(),
                sops_url: init.sops_url.clone(),
                comin_url: init.comin_url.clone(),
            },
            resources: Resources::default(),
        };
        new_config.save()?;
        Ok(new_config)
    }

    pub fn config_file(&self) -> PathBuf {
        self.root_directory.join("nico.config.json")
    }

    pub fn root_directory(&self) -> PathBuf {
        self.root_directory.clone()
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
        if deserialized.config_file() == config_path {
            Ok(deserialized)
        } else {
            Err(crate::Error::ConfigDirectoryMismatch)
        }
    }

    pub fn save(&self) -> crate::Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
        fs::write(self.config_file(), serialized)?;
        Ok(())
    }
}
