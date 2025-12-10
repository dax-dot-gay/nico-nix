use std::sync::Arc;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse arguments/other input: {0}")]
    Parsing(Arc<clap::Error>),

    #[error("An unknown error occurred: {0:?}")]
    Unknown(Arc<anyhow::Error>),

    #[error("Unable to find nico config in this or any parent folders.")]
    ConfigNotFound,

    #[error("Encountered an IO error: {0}")]
    Io(Arc<std::io::Error>),

    #[error("JSON error: {0}")]
    Json(Arc<serde_json::Error>),

    #[error(
        "Configuration reported a configuration directory different from the one it's in. The configuration must be manually corrected."
    )]
    ConfigDirectoryMismatch,
}

macro_rules! from {
    ($error:path, $kind:ident) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Self::$kind(Arc::new(value))
            }
        }
    };
}

from!(serde_json::Error, Json);
from!(clap::Error, Parsing);
from!(anyhow::Error, Unknown);
from!(std::io::Error, Io);

pub type Result<T> = std::result::Result<T, Error>;
