use serde::{Deserialize, de::Visitor, Deserializer};
use serde_json::Value as Json;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Key {
    String(String),
    Int(usize),
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Key::String(s)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Key::String(s.to_string())
    }
}

impl From<usize> for Key {
    fn from(u: usize) -> Self {
        Key::Int(u)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Path(pub Vec<Key>);

impl From<Vec<Key>> for Path {
    fn from(v: Vec<Key>) -> Self {
        Path(v)
    }
}

struct PathVisitor;

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.0.iter().fold(String::new(), |mut s, p| {
            match p {
                Key::String(key) => s.push_str(key),
                Key::Int(idx) => s.push_str(&idx.to_string()),
            }
            s.push('.');
            s
        });
        // remove redundant dot
        s.pop();
        f.write_str(&s)
    }
}

#[derive(Debug, Deserialize)]
pub struct PathWithCommands(
    pub Path,
    #[serde(deserialize_with = "deserialize_path_priority")]
    pub PathPriority,
    #[serde(deserialize_with = "deserialize_path_action")]
    pub PathAction
);

#[derive(Debug)]
pub enum PathPriority {
    /// Will return error if not found
    Requried,
    /// Will skip if not found
    Optional,
}

#[derive(Debug)]
pub enum PathAction {
    /// Just return the thing
    None,
    /// Iterate it if possible
    Iterate,
}

fn deserialize_as_bool<'de, D>(de: D) -> Result<bool, D::Error>
where D: Deserializer<'de> {
    struct BoolVisitor;
    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("bool")
        }
        
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }
    
    de.deserialize_bool(BoolVisitor)
}

fn deserialize_path_priority<'de, D>(de: D) -> Result<PathPriority, D::Error>
where D: Deserializer<'de>
{
    let v = deserialize_as_bool(de)?;
    Ok(if v {
        PathPriority::Requried
    } else {
        PathPriority::Optional
    })
}

fn deserialize_path_action<'de, D>(de: D) -> Result<PathAction, D::Error>
where D: Deserializer<'de>
{
    let v = deserialize_as_bool(de)?;
    Ok(if v {
        PathAction::None
    } else {
        PathAction::Iterate
    })
}

