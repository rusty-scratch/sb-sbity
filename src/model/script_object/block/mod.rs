//! Typing the Scratch's block is so big. I have to put it in a another module.

use getset::Getters;
use serde::{
    Deserialize,
    Serialize,
    de::{Visitor, DeserializeOwned},
    Deserializer,
    Serializer,
};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_json::Value as SerdeValue;

use crate::utils::ConstStr_mutation;

use super::super::{
    id::ID,
    name::Name,
    opcode::OPCode,
    field_hash_map::FieldHashMap,
    value::Value,
    number::Number,
};

/// Scratch scripting block
#[derive(Debug, PartialEq, Clone, Getters, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[get = "pub"]
pub struct Block {
    /// A string naming the block.
    opcode: OPCode<String>,

    /// The ID of the next block or null.
    next: Option<ID>,

    /// If the block is a stack block and is preceded, this is the ID of the preceding block.
    /// If the block is the first stack block in a C mouth, this is the ID of the C block.
    /// If the block is an input to another block, this is the ID of that other block.
    /// Otherwise it is none.
    parent: Option<ID>,

    /// See [`BlockInput`]
    inputs: FieldHashMap<BlockInput>,

    /// See [`BlockField`]
    fields: FieldHashMap<BlockField>,

    /// True if this is a shadow block and false otherwise.
    shadow: bool,

    /// False if the block has a parent and true otherwise.
    top_level: bool,

    /// Mutations are present some blocks that has a certain opcode.
    /// See [`BlockMutationEnum`] for availiable mutations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mutation: Option<BlockMutation>,

    /// X Position of the top level block.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default
    )]
    x: Option<Number>,

    /// Y Position of the top level block.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default
    )]
    y: Option<Number>,
}

/// A struct representing inputs into which other blocks may be dropped, including C mouths.
/// 
/// I'm still figuring this out
#[derive(Debug, Clone, PartialEq, Getters)]
#[get = "pub"]
pub struct BlockInput {
    /// See [`ShadowInputType`]
    shadow: ShadowInputType,

    /// Inputs
    inputs: Vec<Option<IDOrValue>>
}

impl BlockInput {
    /// Use for serializing
    fn size_hint(&self) -> usize {
        1 + self.inputs.len()
    }
}

/// Used for [`BlockInput`]
/// When the input could be either [`ID`] or [`BlockInputValue`]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IDOrValue {
    /// When it's [`ID`]
    ID(ID),
    /// When it's [`BlockInputValue`]
    Value(BlockInputValue),
}

struct BlockInputVisitor;

impl<'de> Visitor<'de> for BlockInputVisitor {
    type Value = BlockInput;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("list that is a block input")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>
    {
        use serde::de::Error;
        
        let shadow = seq.next_element::<ShadowInputType>()?
            .ok_or_else(|| A::Error::invalid_length(0,
                &"Expected 2 or more elements for block input"
            ))?;

        let mut inputs = vec![];
        while let Some(v) = seq.next_element::<Option<IDOrValue>>()? {
            inputs.push(v) 
        }

        Ok(BlockInput {
            shadow,
            inputs,
        })
    }
}

impl<'de> Deserialize<'de> for BlockInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(BlockInputVisitor)
    }
}

impl Serialize for BlockInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        use serde::ser::SerializeSeq;
        let mut s = serializer.serialize_seq(Some(self.size_hint()))?;
        s.serialize_element(&self.shadow)?;
        for v in self.inputs() {
            s.serialize_element(&v)?;
        }
        s.end()
    }
}

/// Shadow enum for [`BlockInput`]
/// 
/// Shadow is area inside block input/arg/param or whatever you wanted to call it.
/// It's consisting of:
///  - raw input field where you just type stuff in and optionally can put a reporter in
///  - menu that you can choose but cannot put a reporter in
///  - menu that you can chose and optionally can put a reporter in
///  - or others I might not catch while developing this
/// 
/// This documentation might not be completed or is completed, idk.
/// Scratch wiki didn't tell anything about this.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json",
    derive(Deserialize_repr, Serialize_repr),
    repr(u8)
)]
pub enum ShadowInputType {
    /// There is a shadow
    Shadow = 1,

    /// There is no shadow
    NoShadow = 2,

    /// There is a shadow but obscured by the input.
    /// The shadow is obscured when reporter is inserted.
    ShadowObscured = 3,
}

/// Input of the BlockInput
#[derive(Debug, Clone, PartialEq)]
pub enum BlockInputValue {
    /// Number input
    Number {
        /// The value
        value: Value,
    } ,

    /// Postive number input
    PositiveNumber {
        /// The value
        value: Value,
    },

    /// Postive integer input
    PositiveInteger { 
        /// The value
        value: Value,
    },

    /// Integer input
    Integer {
        /// The value
        value: Value,
    },

    /// Angle input
    Angle {
        /// The value
        value: Value,
    },

    /// Color input
    Color {
        /// Value, a `#` followed by a hexadecimal numeral representing the color
        value: Value,
    },

    /// String input
    String {
        /// The value
        value: Value,
    },

    /// Broadcast input
    Broadcast {
        /// Name of the broadcast
        name: Name,

        /// ID of the broadcast
        id: ID,
    },

    /// Variable input
    Variable {
        /// Name of the variable
        name: Name,
        /// ID of the variable
        id: ID,
        /// Position X of the variable if top_level
        x: Option<Number>,
        /// Position y of the variable if top_level
        y: Option<Number>,
    },

    /// List input
    List {
        /// Name of the list
        name: Name,
        /// ID of the list
        id: ID,
        /// Position X of the variable if top_level
        x: Option<Number>,
        /// Position y of the variable if top_level
        y: Option<Number>,
    }
}

impl BlockInputValue {
    fn get_id(&self) -> u8 {
        use BlockInputValue::*;

        match self {
            Number { value: _ } => 4,
            PositiveNumber { value: _ } => 5,
            PositiveInteger { value: _ } => 6,
            Integer { value: _ } => 7,
            Angle { value: _ } => 8,
            Color { value: _ } => 9,
            String { value: _ } => 10,
            Broadcast { name: _, id: _ } => 11,
            Variable { name: _, id: _, x: _, y: _ } => 12,
            List { name: _, id: _, x: _, y: _ } => 13,
        }
    }

    fn hint_size(&self) -> usize {
        use BlockInputValue::*;

        match self {
            Number { value: _ } => 1,
            PositiveNumber { value: _ } => 1,
            PositiveInteger { value: _ } => 1,
            Integer { value: _ } => 1,
            Angle { value: _ } => 1,
            Color { value: _ } => 1,
            String { value: _ } => 1,
            Broadcast { name: _, id: _ } => 2,
            Variable { name: _, id: _, x, y } => {
                let mut n = 2;
                if x.is_some() {
                    n += 1
                }
                if y.is_some() {
                    n += 1
                }
                n
            },
            List { name: _, id: _, x, y } => {
                let mut n = 2;
                if x.is_some() {
                    n += 1
                }
                if y.is_some() {
                    n += 1
                }
                n
            },
        }
    }
}

struct BlockInputValueVisitor;

impl<'de> Visitor<'de> for BlockInputValueVisitor {
    type Value = BlockInputValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("list that is a block input value")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>
    {
        use serde::de::Error;
        use BlockInputValue::{
            Number as BlockInputNumber,
            PositiveNumber,
            PositiveInteger,
            Integer,
            Angle,
            Color,
            String,
            Broadcast,
            Variable,
            List,
        };

        fn seq_next_element_error<'de, T, A>(seq: &mut A, len: usize, error: &str) -> Result<T, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
            T: Deserialize<'de>
        {
            seq.next_element::<T>()?
                .ok_or_else(|| A::Error::invalid_length(len, &error))
        }

        let vtype: u8 = seq_next_element_error(
            &mut seq, 0,
            "Expecting 2 or more elements for block input value with any ID"
        )?;
        
        let value = seq_next_element_error(
            &mut seq, 1,
            "Expecting 2 or more elements for block input value with any ID"
        )?;
        
        let res = match vtype {
            4 => BlockInputNumber { value },
            5 => PositiveNumber { value },
            6 => PositiveInteger { value },
            7 => Integer { value },
            8 => Angle { value },
            9 => Color { value },
            10 => String { value },
            11 => {
                let id = seq_next_element_error(
                    &mut seq, 3,
                    "Expecting 3 or more elements for block input value with ID 11"
                )?;

                let name = match value {
                    Value::String(s) => s,
                    Value::Number(_) => return Err(
                        A::Error::invalid_value(
                            serde::de::Unexpected::Other("number"),
                            &"a string"
                        )
                    ),
                };

                Broadcast { name: Name::new(name), id }
            },
            12 => {
                let id = seq_next_element_error(
                    &mut seq, 3,
                    "Expecting 3 or 5 or more elements for block input value with ID 12 - 13 inclusive"
                )?;
                let x = seq.next_element::<Number>()?;
                let y = seq.next_element::<Number>()?;
                let name = match value {
                    Value::String(s) => s,
                    Value::Number(_) => return Err(
                        A::Error::invalid_value(
                            serde::de::Unexpected::Other("number"),
                            &"a string"
                        )
                    ),
                };
                Variable { name: Name::new(name), id, x, y }
            },
            13 => {
                let id = seq_next_element_error(
                    &mut seq, 3,
                    "Expecting 3 or 5 or more elements for block input value with ID 12 - 13 inclusive"
                )?;
                let x = seq.next_element::<Number>()?;
                let y = seq.next_element::<Number>()?;
                let name = match value {
                    Value::String(s) => s,
                    Value::Number(_) => return Err(
                        A::Error::invalid_value(
                            serde::de::Unexpected::Other("number"),
                            &"a string"
                        )
                    ),
                };
                List { name: Name::new(name), id, x, y }
            },
            v => return Err(A::Error::invalid_value(
                serde::de::Unexpected::Unsigned(v.into()),
                &"Expecting a type id between 4 - 13 inclusive"
            ))
        };

        Ok(res)
    }
}

impl<'de> Deserialize<'de> for BlockInputValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(BlockInputValueVisitor)
    }
}

impl Serialize for BlockInputValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        use BlockInputValue::*;
        use serde::ser::SerializeSeq;

        let mut s = serializer.serialize_seq(Some(self.hint_size()))?;
        s.serialize_element(&self.get_id())?;
        match self {
            Number { value } |
            PositiveNumber { value } |
            PositiveInteger { value } |
            Integer { value } |
            Angle { value } |
            Color { value } |
            String { value } => {
                s.serialize_element(value)?;
            },
            Broadcast { name, id } => {
                s.serialize_element(name)?;
                s.serialize_element(id)?;
            },
            Variable { name, id, x, y } |
            List { name, id, x, y } => {
                s.serialize_element(name)?;
                s.serialize_element(id)?;
                if let Some(x) = x {
                    s.serialize_element(x)?;
                }
                if let Some(y) = y {
                    s.serialize_element(y)?;
                }
            },
        }
        s.end()
    }
}

/// Field of the block
#[derive(Debug, Clone, PartialEq)]
pub enum BlockField {
    /// Field when ID are sometimes needed
    WithID {
        /// Value of the field
        value: Value,

        /// For certain fields,
        /// such as variable and broadcast dropdown menus,
        /// there is also a second element, which is the ID of the field's value.
        id: Option<ID>,
    },
    /// Field with no ID needed
    NoID {
        /// Value of the field
        value: Value,
    }
}

impl BlockField {
    /// Value of the field
    #[inline(always)]
    pub fn value(&self) -> &Value {
        match self {
            BlockField::WithID { value, id: _ } => value,
            BlockField::NoID { value } => value,
        }
    }

    /// For certain fields,
    /// such as variable and broadcast dropdown menus,
    /// there is also a second element, which is the ID of the field's value.
    #[inline(always)]
    pub fn id(&self) -> Option<&ID> {
        match self {
            BlockField::WithID { value: _, id } => id.as_ref(),
            BlockField::NoID { value: _ } => None,
        }
    }
}

struct BlockFieldVisitor;

impl<'de> Visitor<'de> for BlockFieldVisitor {
    type Value = BlockField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("sequence of values that is a blockfield")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>
    {
        use serde::de::Error;

        let value = seq.next_element::<Value>()?
            .ok_or_else(|| A::Error::invalid_length(1,
                &"length 1 or 2 for BlockField"
            ))?;
        let id = seq.next_element::<Option<ID>>()?;

        Ok(match id {
            Some(id) => BlockField::WithID { value, id },
            None => BlockField::NoID { value },
        })
    }
}

impl<'de> Deserialize<'de> for BlockField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(BlockFieldVisitor)
    }
}

impl Serialize for BlockField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        use serde::ser::SerializeSeq;
        
        match self {
            BlockField::WithID { value, id } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(value)?;
                seq.serialize_element(id)?;
                seq.end()
            },
            BlockField::NoID { value } => {
                let mut seq = serializer.serialize_seq(Some(1))?;
                seq.serialize_element(value)?;
                seq.end()
            },
        }
    }
}

/// Mutation for procedural block (custom block) or stop block
#[derive(Debug, Clone, PartialEq, Getters)]
#[cfg_attr(
    feature = "json",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase")
)]
#[get = "pub"]
pub struct BlockMutation {
    /// Always equal to "mutation".
    #[cfg_attr(
        feature = "json",
        serde(skip_deserializing)
    )]
    #[getset(skip)]
    tag_name: ConstStr_mutation,

    /// Seems to always be an empty array.
    #[cfg_attr(
        feature = "json",
        serde(skip_deserializing)
    )]
    #[getset(skip)]
    children: [(); 0],

    /// See [`BlockMutationEnum`]
    #[cfg_attr(
        feature = "json",
        serde(flatten)
    )]
    mutation_enum: BlockMutationEnum,
}

/// Different mutation has different properties.
/// This enum define them.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockMutationEnum {
    /// opcode is `"procedures_prototype"` mutations have the following additional properties
    ProceduresPrototype {
        /// The name of the custom block, including inputs: %s for string/number inputs and %b for boolean inputs.
        proccode: String,

        /// An array of the ids of the arguments; these can also be found in the input property of the main block.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str",
        )]
        argumentids: Vec<ID>,

        /// An array of the names of the arguments.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str",
        )]
        argumentnames: Vec<Name>,

        /// An array of the defaults of the arguments.
        ///  - String default is an empty string
        ///  - bool default is `false`
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str",
        )]
        argumentdefaults: Vec<Value>,
        
        /// Whether to run the block without screen refresh or not.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str",
        )]
        warp: Option<bool>,
    },

    /// opcode is `"procedures_call"` mutations have the following additional properties
    ProceduresCall {
        /// The name of the custom block, including inputs: %s for string/number inputs and %b for boolean inputs.
        proccode: String,

        /// An array of the ids of the arguments; these can also be found in the input property of the main block.
        #[cfg_attr(
            feature = "json", 
            serde(
                deserialize_with = "deserialize_json_str",
                serialize_with = "serialize_json_str",
            )
        )]
        argumentids: Vec<ID>,
        
        /// Whether to run the block without screen refresh or not.
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str",
        )]
        warp: Option<bool>,
    },

    /// opcode is `"control_stop"` mutations have the following additional property
    ControlStop {
        /// Whether the block has a block following it or not
        ///  - false for stop all and stop all in sprite
        ///  - true for stop other scripts in sprite)
        #[serde(
            deserialize_with = "deserialize_json_str",
            serialize_with = "serialize_json_str",
        )]
        hasnext: bool,
    }
}

fn deserialize_json_str<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned
{
    use serde::de::Error;

    let v = SerdeValue::deserialize(de)?;
    let s = v.as_str()
        .ok_or_else(|| D::Error::invalid_value(
            serde::de::Unexpected::Other(&v.to_string()),
            &"A str of json"
        ))?;
    let v = serde_json::from_str::<T>(s)
        .map_err(|e| D::Error::custom(e))?;
    
    Ok(v)
}

fn serialize_json_str<S, T>(s: &T, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize
{
    ser.serialize_str(&serde_json::to_string(s).unwrap())
}
