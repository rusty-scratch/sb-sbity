use std::fmt::Debug;

use crate::model::prelude::*;
use crate::model::{
    asset::{Costume, Sound},
    monitor::{ListOrValue, Mode, Monitor, MonitorOpCode, NumberName, Parameter, Slider},
    name::Name,
    opcode::OpCode,
    script_object::{
        block::{
            Block, BlockField, BlockInput, BlockInputValue, BlockMutation, BlockMutationEnum,
            ShadowInputType,
        },
        Broadcast, Comment, List, Variable,
    },
    string_hashmap::StringHashMap,
    target::{Meta, Project, RotationStyle, Sprite, SpriteOrStage, Stage, VideoState},
    value::{Float, Id, Int, Number, Text, Value},
};
use crate::utils::{ConstBool, ConstStr_mutation};

fn json_str_equal<T>(v_json: &str)
where
    T: DeserializeOwned + Serialize + PartialEq + Debug,
{
    let v = serde_json::from_str::<T>(v_json)
        .map_err(|e| panic!("{e}"))
        .unwrap();
    let v_json_after = serde_json::to_string(&v)
        .map_err(|e| panic!("{e}"))
        .unwrap();
    let v_after = serde_json::from_str::<T>(&v_json_after).unwrap();

    assert_eq!(v, v_after, "Assert type consistency");
    assert_eq!(
        serde_json::from_str::<Json>(v_json).unwrap(),
        serde_json::from_str::<Json>(&v_json_after).unwrap(),
        "Assert json consistency"
    )
}

fn json_equal<T>(v_json: Json)
where
    T: DeserializeOwned + Serialize + PartialEq + Debug,
{
    let v = serde_json::from_value::<T>(v_json.clone()).unwrap();
    let v_json_after = serde_json::to_string(&v).unwrap();
    let v_after = serde_json::from_str::<T>(&v_json_after).unwrap();

    assert_eq!(v, v_after, "Assert type consistency");
    assert_eq!(
        v_json,
        serde_json::from_str::<Json>(&v_json_after).unwrap(),
        "Assert json consistency"
    )
}

macro_rules! test_json {
    () => {};
    ($($type:ty {$($fname:ident => $json:expr),*})*) => {
        $(
            $(
                #[test]
                fn $fname() {
                    json_str_equal::<$type>($json)
                }
            )*
        )*
    }
}

fn from_file<P: AsRef<std::path::Path>>(p: P) -> String {
    std::fs::read_to_string(p).unwrap()
}

#[cfg(test)]
mod asset;
#[cfg(test)]
mod block;
#[cfg(test)]
mod hashmap;
#[cfg(test)]
mod id;
#[cfg(test)]
mod monitor;
#[cfg(test)]
mod project;
#[cfg(test)]
mod script_data;
#[cfg(test)]
mod utils;
#[cfg(test)]
mod value;

test_json! {
    // intermediate_test(intermediate::Project):
    //     &from_file("src/test_json/intermediate_testcase.json");
}
