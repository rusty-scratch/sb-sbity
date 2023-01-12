use crate::model::prelude::*;

/// OP code for things like block opcode or monitor opcode
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OpCode<T>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for OpCode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
