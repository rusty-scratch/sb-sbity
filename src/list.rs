//! Module to deal with Scratch list

use crate::prelude::*;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// List of values
#[derive(Debug, Default, PartialEq, Clone, Deserialize_tuple, Serialize_tuple)]
pub struct List {
    /// List name.
    pub name: Text,

    /// Vec of the variable
    pub values: Vec<Value>,
}
