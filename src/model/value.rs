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

impl From<Int> for Number {
    fn from(i: Int) -> Self {
        Number::Int(i)
    }
}

impl From<Float> for Number {
    fn from(i: Float) -> Self {
        Number::Float(i)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl From<Number> for Value {
    fn from(n: Number) -> Self {
        Value::Number(n)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Number(Number::Int(i))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Number(Number::Float(f))
    }
}