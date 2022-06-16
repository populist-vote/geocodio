#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Var(#[from] std::env::VarError),

    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error("Failed to deserialize address from geocodio response")]
    BadAddress(#[from] serde_json::Error),

    #[error("Failed to fetch {0} from Geocodio API")]
    Api(String),

    #[error("Missing required API key")]
    MissingAPIKey,

    #[error("Address cannot be empty")]
    EmptyAddress,
}
