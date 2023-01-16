//! Module to deal with Scratch value

use crate::prelude::*;

pub type Int = i64;
pub type Float = f64;
pub type Text = String;
pub type Uid = String;
pub type Name = String;
pub type OpCode = String;

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

macro_rules! enum_from {
    ($enum:ident {$($ty:ident)*}) => {
        $(
            impl From<$ty> for $enum {
                fn from(v: $ty) -> Self {
                    $enum::$ty(v)
                }
            }
        )*
    }
}

enum_from! {
    Number {
        Int Float
    }
}

enum_from! {
    Value {
        Number Text
    }
}

impl From<Int> for Value {
    fn from(v: Int) -> Self {
        Value::Number(Number::Int(v))
    }
}

impl From<Float> for Value {
    fn from(v: Float) -> Self {
        Value::Number(Number::Float(v))
    }
}

impl From<Value> for ValueWithBool {
    fn from(v: Value) -> Self {
        match v {
            Value::Number(n) => ValueWithBool::Number(n),
            Value::Text(t) => ValueWithBool::Text(t),
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Number::Int(0)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Number(Default::default())
    }
}

impl Default for ValueWithBool {
    fn default() -> Self {
        ValueWithBool::Number(Default::default())
    }
}
