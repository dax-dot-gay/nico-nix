use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum, builder::PossibleValue};

#[cfg(not(debug_assertions))]
use clap_verbosity_flag::ErrorLevel;

#[cfg(debug_assertions)]
use clap_verbosity_flag::TraceLevel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Parser)]
#[command(
    version,
    about,
    long_about = "Automated scaffolding & templating for multi-host Nix configurations.",
    name = "nico"
)]
pub struct Cli {
    #[cfg(debug_assertions)]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<TraceLevel>,

    #[cfg(not(debug_assertions))]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<ErrorLevel>,

    /// Use a project outside of the current directory
    /// Specify the project's root directory
    #[arg(short, long)]
    pub project: Option<PathBuf>,

    #[arg(hide = true, long)]
    pub ignore_project: bool,

    #[command(subcommand)]
    pub operation: Operations,
}

#[derive(Serialize, Deserialize, Clone, Debug, Args)]
#[group(required = false, multiple = false)]
pub struct InitGitArgs {
    /// Initialize a local repository, mostly just for testing
    /// This isn't really applicable to the end product, but may be useful for initial setup.
    #[arg(long = "git-local")]
    pub local: bool,

    /// Clone an existing git repository and add it as a remote.
    /// This does not currently support any authentication.
    #[arg(long = "git-clone")]
    pub clone: Option<String>,

    /// Use an existing git repository in the target directory.
    /// Any existing remotes will be automatically detected and added.
    /// This is the default behaviour if nothing else is selected.
    #[arg(long = "git-existing")]
    pub existing: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, Args)]
pub struct InitArgs {
    /// Path of directory to initialize in, or the current directory if blank.
    /// If the target directory or its parents don't exist, they will be created.
    pub path: Option<String>,

    /// Flake description
    #[arg(short, long = "desc", default_value_t = String::from("Automatically generated config flake."))]
    pub description: String,

    /// System architecture to build for
    #[arg(long, default_value_t = String::from("x86_64-linux"))]
    pub system: String,

    /// Nix version/tag to use in the flake (formats into "nixpkgs/nixos-{nix}")
    #[arg(long, default_value_t = String::from("unstable"))]
    pub nix: String,

    /// SOPS flake url
    #[arg(long, default_value_t = String::from("github:Mic92/sops-nix"))]
    pub sops_url: String,

    /// Comin flake URL
    #[arg(long, default_value_t = String::from("github:nlewo/comin"))]
    pub comin_url: String,

    #[command(flatten)]
    pub git: InitGitArgs
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Shell {
    #[default]
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::PowerShell,
            Self::Zsh,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Shell::Bash => PossibleValue::new("bash"),
            Shell::Elvish => PossibleValue::new("elvish"),
            Shell::Fish => PossibleValue::new("fish"),
            Shell::PowerShell => PossibleValue::new("powershell"),
            Shell::Zsh => PossibleValue::new("zsh"),
        })
    }
}

impl From<clap_complete::Shell> for Shell {
    fn from(value: clap_complete::Shell) -> Self {
        match value {
            clap_complete::Shell::Bash => Self::Bash,
            clap_complete::Shell::Elvish => Self::Elvish,
            clap_complete::Shell::Fish => Self::Fish,
            clap_complete::Shell::PowerShell => Self::PowerShell,
            clap_complete::Shell::Zsh => Self::Zsh,
            _ => unimplemented!(),
        }
    }
}

impl From<Shell> for clap_complete::Shell {
    fn from(value: Shell) -> Self {
        match value {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Elvish => clap_complete::Shell::Elvish,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::PowerShell => clap_complete::Shell::PowerShell,
            Shell::Zsh => clap_complete::Shell::Zsh,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Args)]
pub struct CompletionArgs {
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Serialize, Deserialize, Clone, Debug, Args)]
pub struct StatusArgs {}

#[derive(Serialize, Deserialize, Clone, Debug, Subcommand)]
pub enum Operations {
    /// Initializes a new configuration directory
    Init(InitArgs),

    /// Generate completions for the specified shell
    Completions(CompletionArgs),

    /// Get project status
    Status(StatusArgs)
}
