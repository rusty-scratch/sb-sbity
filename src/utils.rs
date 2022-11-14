use std::ops::Deref;
use crate::model::prelude::*;

#[derive(Debug,Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstBool<const VAL: bool>; 

impl<const VAL: bool> Deref for ConstBool<VAL> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &VAL
    }
}

pub fn is_false(b: &bool) -> bool { !b }
