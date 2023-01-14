//! Module to deal with Serde map

use crate::prelude::*;
use serde::de::MapAccess;
use serde::ser::SerializeMap;
use std::{collections::HashMap, fmt, marker::PhantomData};

/// HashMap<String, V>
#[derive(Debug, Default, PartialEq, Clone, Eq)]
pub struct StringHashMap<V>(pub HashMap<String, V>);

// Serde impl ==================================================================

struct StringHashMapVisitor<V> {
    marker: PhantomData<fn() -> StringHashMap<V>>,
}

impl<V> StringHashMapVisitor<V> {
    fn new() -> Self {
        StringHashMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, V> Visitor<'de> for StringHashMapVisitor<V>
where
    V: Deserialize<'de>,
{
    type Value = StringHashMap<V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let map = HashMap::with_capacity(access.size_hint().unwrap_or(0));
        let mut map = StringHashMap(map);

        while let Some((key, value)) = access.next_entry::<String, _>()? {
            map.0.insert(key, value);
        }

        Ok(map)
    }
}

impl<'de, V> Deserialize<'de> for StringHashMap<V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StringHashMapVisitor::new())
    }
}

impl<V: Serialize> Serialize for StringHashMap<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map_serializer = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map_serializer.serialize_entry(&k.to_string(), v)?;
        }
        Ok(map_serializer.end()?)
    }
}
