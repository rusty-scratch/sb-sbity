use std::ops::Deref;
use crate::model::prelude::*;

#[derive(Debug,Default, Clone, Copy, PartialEq, Eq)]
pub struct ConstBool<const VAL: bool>; 

impl<const VAL: bool> Deref for ConstBool<VAL> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &VAL
    }
}

impl<const VAL: bool> Serialize for ConstBool<VAL> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_bool(VAL)
    }
}

impl<'de, const VAL: bool> Deserialize<'de> for ConstBool<VAL> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_bool(ConstBool)
    }
}

impl<'de, const VAL: bool> Visitor<'de> for ConstBool<VAL> {
    type Value = Self;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bool")
    }
    
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        if v == VAL {
            Ok(Self)
        } else {
            Err(E::custom(&format!("expecting {}", VAL)))
        }
    }
}

pub fn is_false(b: &bool) -> bool { !b }

/// This is only for serializing compile time constant field where the field never change.
/// Type is `&str` and always serialize `"mutation"`
#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub(crate) struct ConstStr_mutation;

impl ConstStr_mutation {
    const VALUE: &str = "mutation";
}

impl Serialize for ConstStr_mutation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_str(Self::VALUE)
    }
}

impl<'de> Deserialize<'de> for ConstStr_mutation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(ConstStr_mutation)
    }
}

impl<'de> Visitor<'de> for ConstStr_mutation {
    type Value = Self;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("mutation")
    }
    
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == "mutation" {
            Ok(ConstStr_mutation)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &"mutation"))
        }
    }
}

impl Default for ConstStr_mutation {
    fn default() -> Self {
        Self
    }
}
