use crate::model::prelude::*;

pub type Int = i64;
pub type Float = f64;
pub type Text = String;

pub type Id = String;

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
