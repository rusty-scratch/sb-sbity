//! Module to deal with Scratch variable

use crate::prelude::*;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// Variable
#[derive(Debug, Default, PartialEq, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct Variable {
    /// Variable name.
    pub name: Text,

    /// Value of the variable.
    pub value: Value,

    /// Cloud variable. Variable that is stored in the server.
    /// Can only store numbers.
    #[serde(skip_serializing_if = "utils::is_false", default)]
    pub is_cloud_variable: bool,
}
