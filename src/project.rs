//! Module to deal with Scratch project

use crate::monitor::Monitor;
use crate::prelude::*;
use crate::target::SpriteOrStage;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub meta: Meta,
    pub extensions: Json,
    pub monitors: Vec<Monitor>,
    pub targets: Vec<SpriteOrStage>,
}

/// About the project's author and the Scratch version used.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// Always `3.0.0`.
    pub semver: String,

    /// The version of the Scratch VM that the project was created with.
    pub vm: String,

    /// The user agent of the last person to edit the project from the editor.
    pub agent: String,
}
