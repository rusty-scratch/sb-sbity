use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Cfg {
    /// Function name prefix that will be generate.
    pub func_prefix: String,
    /// Path to array where its content will be generate for test.
    pub path_to_array: JsonPath,
    /// After the array is found, this path will be use to go to part of the item that will be use to generate.
    pub path_to_content: JsonPath,
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
