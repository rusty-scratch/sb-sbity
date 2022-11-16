use std::{collections::HashMap, marker::PhantomData, fmt};
use crate::model::prelude::*;
use serde::de::MapAccess;
use serde::ser::SerializeMap;

/// HashMap<String, V>
#[derive(Debug, PartialEq, Clone)]
pub struct StringHashMap<V>(pub HashMap<String, V>);

impl<V> StringHashMap<V> {
    // /// Automatically generate key for the value that does not exist in the hasmap
    // /// gen_func is function to use to generate the random ID
    // pub fn insert<F>(&mut self, gen_func: F, v: T)
    // where
    //     F: Fn() -> ID
    // {
    //     let k = loop {
    //         let k = gen_func();
    //         if self.0.get(&k).is_none() {
    //             break k 
    //         }
    //     };

    //     let r = self.0.insert(k, v);
    //     assert!(r.is_none(), "R should be None!");
    // }
}

struct StringHashMapVisitor<V> {
    marker: PhantomData<fn() -> StringHashMap<V>>
}

impl<V> StringHashMapVisitor<V>
{
    fn new() -> Self {
        StringHashMapVisitor {
            marker: PhantomData
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

impl<V: Serialize> Serialize for StringHashMap<V>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut map_serializer = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map_serializer.serialize_entry(&k.to_string(), v)?;
        }
        Ok(map_serializer.end()?)
    }
}
