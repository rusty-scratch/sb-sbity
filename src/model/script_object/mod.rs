use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use crate::model::prelude::*;

pub mod block;

/// Variable
#[derive(Debug, PartialEq, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct Variable {
    /// Variable name.
    pub name: Text,

    /// Value of the variable.
    pub value: Value,

    /// Cloud variable. Variable that is stored in the server.
    /// Can only store numbers.
    #[serde(skip_serializing_if = "utils::is_false", default)]
    pub is_cloud_variable: bool
}

/// List of values
#[derive(Debug, PartialEq, Clone, Getters, Deserialize_tuple, Serialize_tuple)]
pub struct List {
    /// List name.
    pub name: Text,

    /// Vec of the variable
    pub values: Vec<Value>,
}

/// Adjustable textboxes that can be attached to [`block::Block`]s, or left floating
#[derive(Debug, PartialEq, Clone, Getters, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    /// The ID of the block the comment is attached to.
    #[getset(get = "pub")]
    block_id: Option<Id>,

    /// The x-coordinate of the comment in the code area.
    pub x: Number,

    /// The y-coordinate of the comment in the code area.
    pub y: Number,

    /// The width.
    pub width: Number,

    /// The height.
    pub height: Number,

    /// True if the comment is collapsed and false otherwise.
    pub minimized: bool,

    /// The text.
    pub text: Text,
}

/// Only contains in stage
#[derive(Debug, Clone, Getters, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Broadcast{
    /// Name of the broadcast
    pub name: Text,
}
