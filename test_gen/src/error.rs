#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeToml(#[from] toml::de::Error),
    #[error("the item you want to make a test to is not iteratable e.g. Array, Object")]
    ItemIsNotIteratable,
    #[error("path you configured to an array does not exist")]
    PathToArrayNotExist,
    #[error("path you configured to the content does not exist")]
    PathToContentNotExist,
    #[error(transparent)]
    IterError(#[from] crate::ContentIterError),
}
