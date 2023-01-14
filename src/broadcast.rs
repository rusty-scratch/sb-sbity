//! Module to deal with Scratch broadcast

use crate::prelude::*;

/// Only contains in stage
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Broadcast {
    /// Name of the broadcast
    pub name: Text,
}
