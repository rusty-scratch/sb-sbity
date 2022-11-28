use serde::{Deserialize, de::Visitor};
use crate::PathWithPriotiy;
#[derive(Debug, Deserialize)]
pub struct Cfg {
    /// Function name prefix that will be generate.
    pub func_prefix: String,
    /// Path to array/object recursively where its content will be generate for test.
    /// (it's hard to explain)
    pub path_to_content: Vec<PathWithPriotiy>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum JsonPathSeg {
    String(String),
    Index(usize),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct JsonPath(pub Vec<JsonPathSeg>);

struct JsonPathVisitor;

impl std::fmt::Display for JsonPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.0.iter().fold(String::new(), |mut s, p| {
            match p {
                JsonPathSeg::String(key) => s.push_str(key),
                JsonPathSeg::Index(idx) => s.push_str(&idx.to_string()),
            }
            s.push('.');
            s
        });
        // remove redundant dot
        s.pop();
        f.write_str(&s)
    }
}
