//! Module to deal with Scratch comment

use crate::prelude::*;

/// Adjustable textboxes that can be attached to [`crate::block::Block`]s, or left floating
#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    /// The ID of the block the comment is attached to.
    pub block_id: Option<Uid>,

    /// The x-coordinate of the comment in the code area.
    pub x: Option<Number>,

    /// The y-coordinate of the comment in the code area.
    pub y: Option<Number>,

    /// The width.
    pub width: Number,

    /// The height.
    pub height: Number,

    /// True if the comment is collapsed and false otherwise.
    pub minimized: bool,

    /// The text.
    pub text: Text,
}
