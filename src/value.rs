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

/// OP code for things like block opcode or monitor opcode
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OpCode<T>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for OpCode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
