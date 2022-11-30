use serde::{Deserialize, de::Visitor};


#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PathSegment {
    String(String),
    Index(usize),
}

impl From<String> for PathSegment {
    fn from(s: String) -> Self {
        PathSegment::String(s)
    }
}

impl From<&str> for PathSegment {
    fn from(s: &str) -> Self {
        PathSegment::String(s.to_string())
    }
}

impl From<usize> for PathSegment {
    fn from(u: usize) -> Self {
        PathSegment::Index(u)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Path(pub Vec<PathSegment>);

impl From<Vec<PathSegment>> for Path {
    fn from(v: Vec<PathSegment>) -> Self {
        Path(v)
    }
}

struct PathVisitor;

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.0.iter().fold(String::new(), |mut s, p| {
            match p {
                PathSegment::String(key) => s.push_str(key),
                PathSegment::Index(idx) => s.push_str(&idx.to_string()),
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
pub struct PathWithPriotiy(pub Path, pub PathPriority);

#[derive(Debug)]
pub enum PathPriority {
    /// Will return error if not found
    Requried,
    /// Will skip if not found
    Optional,
}

struct PathPriorityVisitor;

impl<'de> Visitor<'de> for PathPriorityVisitor {
    type Value = PathPriority;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bool")
    }
    
    fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v {
            Ok(PathPriority::Requried)
        } else {
            Ok(PathPriority::Optional)
        }
    }
}

impl<'de> Deserialize<'de> for PathPriority {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_bool(PathPriorityVisitor)
    }
}

