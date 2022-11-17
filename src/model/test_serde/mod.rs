use std::fmt::Debug;

use crate::model::prelude::*;
use crate::utils::{
    ConstBool,
    ConstStr_mutation,
};
use crate::model::{
    string_hashmap::StringHashMap,
    target::{
        Meta,
        Project,
        RotationStyle,
        Sprite,
        SpriteOrStage,
        Stage,
        VideoState,
    },
    name::Name,
    opcode::OpCode,
    value::{
        Float, Id, Int,
        Value, Text, Number,
    },
    asset::{Costume, Sound},
    monitor::{
        Monitor, Mode, MonitorOpCode,
        NumberName, Parameter, Slider, ListOrValue
    },
    script_object::{
        Broadcast, Comment,
        List, Variable,
        block::{
            Block,
            BlockInputValue,
            BlockMutation,
            ShadowInputType,
        },
    }
};

fn json_equal<T>(v_json: &str)
where T: DeserializeOwned + Serialize + PartialEq + Debug
{
    let v = serde_json::from_str::<T>(v_json).unwrap();
    let v_json_after = serde_json::to_string(&v).unwrap();
    let v_after = serde_json::from_str::<T>(&v_json_after).unwrap();

    assert_eq!(v, v_after, "Assert type consistency");
    assert_eq!(
        serde_json::from_str::<Json>(v_json).unwrap(),
        serde_json::from_str::<Json>(&v_json_after).unwrap(),
        "Assert json consistency"
)
}

macro_rules! test_json {
    () => {};
    ($($type:ty {$($fname:ident => $json:expr),*}),*) => {
        $(
            $(
                #[test]
                fn $fname() {
                    json_equal::<$type>($json)
                }
            )*
        )*
    }
}

fn from_file<P: AsRef<std::path::Path>>(p: P) -> String {
    std::fs::read_to_string(p).unwrap()
}


#[cfg(test)] mod hashmap;
#[cfg(test)] mod utils;
#[cfg(test)] mod id;
#[cfg(test)] mod values;
#[cfg(test)] mod monitor;
#[cfg(test)] mod asset;
#[cfg(test)] mod script_data;

test_json!{
    // intermediate_test(intermediate::Project):
    //     &from_file("src/test_json/intermediate_testcase.json");
}