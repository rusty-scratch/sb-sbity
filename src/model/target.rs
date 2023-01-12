use crate::model::asset::{Costume, Sound};
use crate::model::prelude::*;
use crate::model::script_object::{block::Block, Broadcast, Comment, List, Variable};
use crate::model::string_hashmap::StringHashMap;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub meta: Meta,
    pub extensions: Json,
    pub monitors: Json,
    pub targets: Vec<SpriteOrStage>,
}

/// About the project's author and the Scratch version used.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Meta {
    /// Always `3.0.0`.
    pub semver: String,

    /// The version of the Scratch VM that the project was created with.
    pub vm: String,

    /// The user agent of the last person to edit the project from the editor.
    pub agent: String,
}

/// A target is the stage or a sprite.
#[derive(Debug, PartialEq, Clone, Getters, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    /// The name of the sprite. Always "Stage" for the stage.
    /// If not provided, the target will not be loaded.
    pub name: Text,

    /// An object associating IDs with arrays representing variables.
    /// The first element of the array is the variable name,
    /// the second is the value and the third is `true` if the variable is a cloud variable,
    /// or otherwise not present.
    variables: StringHashMap<Variable>,

    /// An object associating IDs with arrays representing lists.
    /// The first element of the array is the list name and the second is the list as an array.
    lists: StringHashMap<List>,

    /// An object associating IDs with broadcast names.
    /// Normally only present in the stage.
    broadcasts: StringHashMap<Broadcast>,

    /// An object associating IDs with blocks.
    blocks: StringHashMap<Block>,

    /// An object associating IDs with comments.
    comments: StringHashMap<Comment>,

    /// The costume number.
    pub current_costume: Int,

    /// An array of costumes.
    pub costumes: Vec<Costume>,

    /// An array of sounds.
    pub sounds: Vec<Sound>,

    /// The layer number.
    pub layer_order: Int,

    /// The volume
    pub volume: Number,
}

/// Scratch's Stage.
/// Costume is considered backdrop.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Stage {
    /// See [`Target`]
    #[serde(flatten)]
    pub target: Target,

    /// The tempo in BPM.
    pub tempo: Number,

    /// See [`VideoState`]
    pub video_state: VideoState,

    /// The video transparency.
    /// Defaults to 50. Has no effect if video_stage is "off"
    /// or if the project does not use an extension with video input.
    pub video_transparency: Number,

    /// The language of the Text to Speech extension. Defaults to the editor language.
    // TODO: Create TextToSpeechLangage struct
    pub text_to_speech_language: Option<Json>,

    is_stage: utils::ConstBool<true>,
}

/// Scratch Sprite
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    /// See [`Target`]
    #[serde(flatten)]
    pub target: Target,

    /// True if the sprite is visible and false otherwise. Defaults to true.
    pub visible: bool,

    /// The x-coordinate. Defaults to 0.
    pub x: Number,

    /// The y-coordinate. Defaults to 0.
    pub y: Number,

    /// The sprite's size as a percentage. Defaults to 100.
    pub size: Number,

    /// The sprite's direction in degrees clockwise from up. Defaults to 90.
    pub direction: Number,

    /// True if the sprite is draggable and false otherwise. Defaults to false.
    pub draggable: bool,

    /// See [`RotationStyle`]
    pub rotation_style: RotationStyle,

    is_stage: utils::ConstBool<false>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SpriteOrStage {
    Stage(Stage),
    Sprite(Sprite),
}

/// Determines if video is visible on the stage and if it is flipped.
/// Has no effect if the project does not use an extension with video input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoState {
    /// Video is on
    #[serde(rename = "on")]
    On,

    /// Video is off
    #[serde(rename = "off")]
    Off,

    /// Video is on and flipped
    #[serde(rename = "on-flipped")]
    OnFlipped,
}

/// A [`Sprite`]'s rotation style controls which directions a sprite can face in.
/// These directions are all in accordance with the analogous rotation system used by Scratch.
/// Depending on the rotation style of a sprite, the sprite may appear to be facing a different way than the direction it has been set to.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum RotationStyle {
    /// Visually points the sprite in the direction it is facing.
    /// However, this will make the sprite appear upside-down if it is facing left.
    #[serde(rename = "all around")]
    AllAround,

    /// Flips the sprite right or left.
    /// If the sprite's direction is between 0° and 180°, the costume will not appear rotated.
    /// If the sprite's direction is between 0° and -180°, the costume will be mirrored around the y axis.
    #[serde(rename = "left right")]
    LeftRight,

    /// Don't rotate
    ///
    /// Note: The sprite's visual direction will not change,
    ///       but the direction it moves with the Move () Steps block can still be modified.
    #[serde(rename = "don't rotate")]
    DontRotate,
}
