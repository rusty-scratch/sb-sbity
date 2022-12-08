use serde::Deserialize;
use crate::PathWithCommands;

#[derive(Debug, Deserialize)]
pub struct Cfg {
    /// Function name prefix that will be generate.
    pub func_prefix: String,
    /// Path to array/object recursively where its content will be generate for test.
    /// (it's hard to explain)
    pub path_to_content: Vec<PathWithCommands>,
}
