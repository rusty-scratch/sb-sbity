use crate::model::prelude::*;

/// Newtype struct for string that suppose to be a name
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Name(pub String);

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name(s.into())
    }
}
