use thiserror::Error;
use tokio_tungstenite::tungstenite;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Tungstenite(#[from] tungstenite::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
