//! For to use within the crate

pub use serde::{
    de::Unexpected,
    de::{DeserializeOwned, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
pub use serde_json::Value as Json;
pub use serde_repr::{Deserialize_repr, Serialize_repr};

pub(crate) use crate::utils;
pub use crate::{
    string_hashmap::StringHashMap,
    value::{Float, Int, Name, Number, OpCode, Text, Uid, Value, ValueWithBool},
};
