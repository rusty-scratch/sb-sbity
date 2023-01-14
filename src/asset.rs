//! Module to deal with Scratch asset

use crate::prelude::*;

/// Costume Asset.
/// Is considered backdrop if stage.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    /// The x-coordinate of the rotation center.
    pub rotation_center_x: Number,

    /// The y-coordinate of the rotation center.
    pub rotation_center_y: Number,

    /// The reciprocal of a costume scaling factor for bitmap costumes.
    /// This may be absent. In Scratch 3.0, all bitmap costumes are double-resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitmap_resolution: Option<u64>,

    /// See [`Asset`]
    #[serde(flatten)]
    pub asset: Asset,
}

/// Sound Asset.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
    /// The sampling rate of the sound in Hertz.
    pub rate: u64,

    /// The number of samples.
    pub sample_count: u64,

    /// This is for some reason exists in the file but is not documented on the wiki.
    /// I'm not exactly sure what they do since this is always empty.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    format: Option<String>,

    /// See [`Asset`]
    #[serde(flatten)]
    pub asset: Asset,
}

/// An asset is a costume or sound. (Backdrops are considered costumes.)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// The MD5 hash of the asset file.
    pub asset_id: Id,

    /// The name.
    pub name: Name,

    /// The name of the asset file.
    /// None if using the default asset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md5ext: Option<String>,

    /// The name of the format of the asset file.
    pub data_format: String,
}
