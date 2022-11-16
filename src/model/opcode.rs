use crate::model::prelude::*;

/// OP code for things like block opcode or monitor opcode
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OpCode<T>(T);
