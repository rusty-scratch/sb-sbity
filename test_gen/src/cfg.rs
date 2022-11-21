use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Cfg {
    /// Function name prefix that will be generate.
    pub func_prefix: String,
    /// Path to array/object recursively where its content will be generate for test.
    /// (it's hard to explain)
    pub path_to_cotent: Vec<JsonPath>,
    /// What to do when the content is not found in path to content
    pub content_not_found_action: ContentNotFoundAction,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JsonPathSeg {
    String(String),
    Index(usize),
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct JsonPath(pub Vec<JsonPathSeg>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentNotFoundAction {
    Skip,
    ErrorOut,
}
