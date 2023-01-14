//! Module to deal with Scratch monitor

use crate::prelude::*;
use serde::ser::SerializeMap;

/// A Stage monitor, sometimes called a watcher, is a display on the Stage that shows the value of a variable, boolean, or a list.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    /// The Id.
    pub id: Id,

    /// See [`Mode`]
    pub mode: Mode,

    /// See [`MonitorOpCode`]
    pub opcode: OpCode<MonitorOpCode>,

    /// See [`Parameter`]
    #[serde(rename = "params")]
    pub params: Parameter,

    /// The name of the target the monitor belongs to, if any.
    /// [`None`] if is global var.
    pub sprite_name: Option<Name>,

    /// The value appearing on the monitor.
    pub value: ListOrValue,

    /// The width.
    pub width: u64,

    /// The height.
    pub height: u64,

    /// The x-coordinate.
    pub x: i64,

    /// The y-coordinate.
    pub y: i64,

    /// True if the monitor is visible and false otherwise.
    pub visible: bool,

    /// See [`Slider`]
    /// [`None`] if [`Mode`] is [`Mode::List`]
    #[serde(flatten)]
    pub slider: Option<Slider>,
}

/// OP code for type of data this monitor is showing
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum MonitorOpCode {
    /// User defined variable
    #[serde(rename = "data_variable")]
    DataVariable,

    /// User defiend list
    #[serde(rename = "data_listcontents")]
    DataListContents,

    /// Sprite's variable x position
    #[serde(rename = "motion_xposition")]
    MotionXPostion,

    /// Sprite's variable y position
    #[serde(rename = "motion_yposition")]
    MotionYPosition,

    /// Sprite's variable direction
    #[serde(rename = "motion_direction")]
    MotionDirection,

    /// Sprite's variable costume
    #[serde(rename = "looks_costumenumbername")]
    LooksCostume,

    /// Sprite's variable backdrop
    #[serde(rename = "looks_backdropnumbername")]
    LooksBackdrop,

    /// Sprite's variable size
    #[serde(rename = "looks_size")]
    LooksSize,

    /// Sense loudness
    #[serde(rename = "sensing_loudness")]
    SensingLoudness,

    /// Variable timer
    #[serde(rename = "sensing_timer")]
    SensingTimer,

    /// Variable username
    #[serde(rename = "sensing_username")]
    SensingUsername,
}

/// Monitor's Mode
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Mode {
    /// Show value in small display
    Default,

    /// Show value in large display
    Large,

    /// Have slider to the monitor
    Slider,

    /// Display as list
    List,
}

/// Name of list or variable that this monitor refering to
#[derive(Debug, PartialEq, Clone)]
pub enum Parameter {
    /// No parameter
    None,

    /// Name of a user defined variable
    Variable(Name),

    /// Name of a user defined list
    List(Name),

    /// See [`NumberName`]
    NumberName(NumberName),
}

/// Enum for monitor value.
/// The field could be either value or list.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ListOrValue {
    /// When the field is not a list
    Value(Value),
    /// When the field is a list
    List(Vec<Value>),
}

/// Show number or name of a chosen variable
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NumberName {
    /// Show variable as number
    Number,

    /// Show variable as name
    Name,
}

/// Monitors that do not belong to lists also have these properties
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Slider {
    /// The minimum value of the monitor's slider.
    slider_min: i64,

    /// The maximum value of the monitor's slider.
    slider_max: i64,

    /// True if the monitor's slider allows only integer values and false otherwise.
    is_discrete: bool,
}

// Serde impl ==================================================================

struct ParameterVisitor;

impl<'de> Visitor<'de> for ParameterVisitor {
    type Value = Parameter;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("object that is a parameter")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;
        if let Some((k, v)) = map.next_entry::<&str, &str>()? {
            Ok(match (k, v) {
                ("VARIABLE", v) => Parameter::Variable(v.to_owned()),
                ("LIST", v) => Parameter::List(v.into()),
                ("NUMBER_NAME", "name") => Parameter::NumberName(NumberName::Name),
                ("NUMBER_NAME", "number") => Parameter::NumberName(NumberName::Number),
                (k, _) => {
                    return Err(A::Error::invalid_value(
                        serde::de::Unexpected::Str(k),
                        &"Expected either VARIABLE, LIST, or NUMBER_NAME",
                    ))
                }
            })
        } else {
            Ok(Parameter::None)
        }
    }
}

impl<'de> Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ParameterVisitor)
    }
}

impl Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Parameter::None => serializer.serialize_map(Some(0))?.end(),
            r => {
                let mut s = serializer.serialize_map(Some(1))?;
                match r {
                    Parameter::Variable(n) => s.serialize_entry("VARIABLE", n)?,
                    Parameter::List(n) => s.serialize_entry("LIST", n)?,
                    Parameter::NumberName(n) => s.serialize_entry("NUMBER_NAME", n)?,
                    Parameter::None => unreachable!("There cannot be none in here"),
                };
                s.end()
            }
        }
    }
}
