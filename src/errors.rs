use thiserror::Error;

pub type Result<T> = std::result::Result<T, OrandaError>;

#[derive(Debug, Error)]
pub enum OrandaError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error(transparent)]
    ParseError(#[from] url::ParseError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Syntect(#[from] syntect::Error),

    #[error(transparent)]
    AxoAsset(#[from] axoasset::AxoassetError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    FSExtra(#[from] fs_extra::error::Error),

    #[error("failed to read {filedesc} at {path}")]
    FileNotFound { filedesc: String, path: String },

    #[error("Could not find a build in {dist_dir}. Did you remember to run `oranda build`?")]
    BuildNotFound { dist_dir: String },

    #[error("Encountered an error (status: {status_code}) while fetching your cargo-dist release manifest at {url}. This often occurs when you haven't yet published a GitHub release for the version set in your Cargo.toml. Consider publishing a release or promoting a draft release for the current version, or updating your Cargo.toml to use an already released version.")]
    CargoDistManifestFetchError {
        url: String,
        status_code: reqwest::StatusCode,
    },

    #[error("Encountered an error parsing your cargo-dist manifest at {url}. Details: {details}")]
    CargoDistManifestParseError { url: String, details: String },

    #[error("{0}")]
    Other(String),
}
