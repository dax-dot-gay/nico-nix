#![allow(dead_code)]

use std::{
    collections::HashMap, fs, path::{Path, PathBuf}
};

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{cli::InitArgs, context::Context};

#[derive(Serialize, Deserialize, Clone, Debug, Builder)]
pub struct GitRemote {
    #[builder(start_fn, into)]
    pub name: String,

    #[builder(start_fn, into)]
    pub url: String,

    #[builder(default = "main".to_string(), into)]
    pub main_branch: String,

    #[builder(default = "testing-".to_string(), into)]
    pub testing_branch_prefix: String,

    #[builder(default = 60)]
    pub polling_period: u64,

    #[builder(default = 300)]
    pub timeout: u64
}

impl GitRemote {
    pub fn as_nix(&self) -> String {
        handlebars::Handlebars::new().render_template("{
            name = \"{{name}}\";
            url = \"{{url}}\";
            branches.main.name = \"{{main_branch}}\";
            branches.testing.name = \"{{testing_branch_prefix}}${config.services.comin.hostname}\";
            poller.period = {{polling_period}};
            timeout = {{timeout}};
        }", &self).expect("Failed to render remote into nix config.")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitConfig {
    pub description: String,
    pub nix: String,
    pub system: String,
    pub sops_url: String,
    pub comin_url: String
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
    pub remotes: HashMap<String, GitRemote>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    pub init: InitConfig,
    pub resources: Resources,
}

impl Configuration {
    pub fn new(root: PathBuf, init: InitArgs, remotes: Vec<GitRemote>) -> crate::Result<Self> {
        let mut resources = Resources::default();
        resources.remotes = remotes.into_iter().map(|r| (r.name.clone(), r)).collect();
        let new_config = Self {
            init: InitConfig {
                description: init.description.clone(),
                nix: init.nix.clone(),
                system: init.system.clone(),
                sops_url: init.sops_url.clone(),
                comin_url: init.comin_url.clone(),
            },
            resources: resources,
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

    pub fn load_path(path: impl AsRef<Path>) -> crate::Result<Self> {
        let path = path.as_ref();
        let config_path = (if path.is_dir() && path.join("nico.config.json").exists() {
            Ok(path.join("nico.config.json"))
        }
        else if path.is_file() && path.ends_with("nico.config.json") {
            Ok(path.to_path_buf())
        } else {
            Err(crate::Error::ConfigNotFound)
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
