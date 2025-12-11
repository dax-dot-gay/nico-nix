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

    #[error("Failed to render template: {0}")]
    TemplateRendering(Arc<handlebars::RenderError>)
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
from!(handlebars::RenderError, TemplateRendering);

pub type Result<T> = std::result::Result<T, Error>;
