use serde::{
    de::{Deserialize, DeserializeOwned, Deserializer, Unexpected},
    ser::{Serialize, Serializer},
};
use serde_json::Value as Json;

pub fn is_false(v: &bool) -> bool {
    !v
}

pub fn json_to_unexpected(json: &Json) -> Unexpected<'_> {
    match json {
        Json::Null => Unexpected::Unit,
        Json::Bool(b) => Unexpected::Bool(*b),
        Json::Number(n) => {
            if let Some(n) = n.as_i64() {
                Unexpected::Signed(n)
            } else if let Some(n) = n.as_u64() {
                Unexpected::Unsigned(n)
            } else if let Some(n) = n.as_f64() {
                Unexpected::Float(n)
            } else {
                unreachable!()
            }
        }
        Json::String(s) => Unexpected::Str(s),
        Json::Array(_) => Unexpected::Seq,
        Json::Object(_) => Unexpected::Map,
    }
}

pub fn deserialize_json_str<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    use serde::de::Error;

    let v = Json::deserialize(de)?;
    let s = v.as_str().ok_or_else(|| {
        D::Error::invalid_value(
            serde::de::Unexpected::Other(&v.to_string()),
            &"A str of json",
        )
    })?;
    let v = serde_json::from_str::<T>(s).map_err(|e| D::Error::custom(e))?;

    Ok(v)
}

pub fn serialize_json_str<S, T>(s: &T, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    ser.serialize_str(&serde_json::to_string(s).unwrap())
}
