//! Module to deal with Scratch value

use crate::prelude::*;

pub type Int = i64;
pub type Float = f64;
pub type Text = String;
pub type Id = String;
pub type Name = String;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
    Int(Int),
    Float(Float),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(Number),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueWithBool {
    Number(Number),
    Text(Text),
    Bool(bool),
}

/// OP code for things like block opcode or monitor opcode
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OpCode<T>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for OpCode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
