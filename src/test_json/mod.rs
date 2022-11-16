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
        block::Block,
    }
};

fn json_equal<T>(v_json: &str, equal: bool)
where T: DeserializeOwned + Serialize + PartialEq + Debug
{
    let v = serde_json::from_str::<T>(v_json).unwrap();
    let v_json_after = serde_json::to_string(&v).unwrap();
    let v_after = serde_json::from_str::<T>(&v_json_after).unwrap();

    if equal {
        assert_eq!(v, v_after, "Assert type consistency");
        assert_eq!(
            serde_json::from_str::<Json>(v_json).unwrap(),
            serde_json::from_str::<Json>(&v_json_after).unwrap(),
            "Assert json consistency"
        )
    } else {
        assert_ne!(v, v_after, "Assert type consistency not equal");
        assert_ne!(
            serde_json::from_str::<Json>(v_json).unwrap(),
            serde_json::from_str::<Json>(&v_json_after).unwrap(),
            "Assert json consistency not equal"
        )
    }
}

macro_rules! test_json {
    () => {};
    ($name:ident($ty:ty): $json:expr; $($chain:tt)*) => {
        #[test]
        fn $name() {
            json_equal::<$ty>($json, true)
        }

        test_json!{$($chain)*}
    };
    (unequal_json $name:ident($ty:ty): $json:expr; $($chain:tt)*) => {
        #[test]
        fn $name() {
            test_t::<$ty>($json, false)
        }

        test_json!{$($chain)*}
    };
}

#[test]
mod test_utils {
    use super::*;
    
    test_json!{
        const_bool_true(ConstBool<true>): r"true";
        const_bool_false(ConstBool<false>): r"false";
        const_str_mutation(ConstStr_mutation): r"mutation";
    }
}

test_json!{
    asset_id(Id): r#""83a9787d4cb6f3b7632b4ddfebf74367""#;
    item_id(Id): r#""{6B[NX:abWD|;$w~~Yl?""#;
    var_id(Id): r#""`jEk@4|i[#Fk?(8x)AV.-my variable""#;

    name(Name): r#""johndoe""#;

    id_hash_map(StringHashMap<i32>):
        r#"{
            "{6B[NX:abWD|;$w~~Yl?": 23,
            "{osdhgo324tg7gbbwd34": 16,
            "o49834ihkgjafsdkfjh3": 802
        }"#;
    
    monitor_slider_local_int(Monitor):
        r##"{
            "id":"#[`9wZIkt35Iad}bQLa_",
            "mode":"slider",
            "opcode":"data_variable",
            "params":{"VARIABLE":"local var int"},
            "spriteName":"Cat",
            "value":"23",
            "width":0,
            "height":0,
            "x":5,
            "y":5,
            "visible":true,
            "sliderMin":0,
            "sliderMax":100,
            "isDiscrete":true
        }"##;
    
    monitor_large_local_float(Monitor):
        r#"{
            "id":"^fJ8}DC5@}XKqsTJ,*Ct",
            "mode":"large",
            "opcode":"data_variable",
            "params":{"VARIABLE":"local var float"},
            "spriteName":"Cat",
            "value":"9.6",
            "width":0,
            "height":0,
            "x":5,
            "y":32,
            "visible":true,
            "sliderMin":0,
            "sliderMax":100,
            "isDiscrete":true
        }"#;
    
    monitor_local_float_int(Monitor):
        r#"{
            "id":"mP%]jA?,6zw7QtQAZ|*8",
            "mode":"slider",
            "opcode":"data_variable",
            "params":{"VARIABLE":"local var float int"},
            "spriteName":"Cat",
            "value":"40.0",
            "width":0,
            "height":0,
            "x":205,
            "y":5,
            "visible":true,
            "sliderMin":-100,
            "sliderMax":100,
            "isDiscrete":false
        }"#;
    
    monitor_local_str(Monitor):
        r#"{
            "id":"xD#(GuL@+XHK0=OXG,Yt",
            "mode":"default",
            "opcode":"data_variable",
            "params":{"VARIABLE":"local var str"},
            "spriteName":"Cat",
            "value":"cock n balls",
            "width":0,
            "height":0,
            "x":5,
            "y":59,
            "visible":true,
            "sliderMin":0,
            "sliderMax":100,
            "isDiscrete":true
        }"#;
    
    monitor_global_int(Monitor):
        r#"{
            "id":"XS@{/ji:?7$cj0_|$t|s",
            "mode":"default",
            "opcode":"data_variable",
            "params":{"VARIABLE":"global var int"},
            "spriteName":null,
            "value":0,
            "width":0,
            "height":0,
            "x":5,
            "y":86,
            "visible":true,
            "sliderMin":0,
            "sliderMax":100,
            "isDiscrete":true
        }"#;
    
    monitor_local_list(Monitor):
        r#"{
            "id":"%%~iKGUwz^/@M8d[6ERY",
            "mode":"list",
            "opcode":"data_listcontents",
            "params":{"LIST":"local list"},
            "spriteName":"Cat",
            "value":[
                "thing",
                "balss",
                "23"
            ],
            "width":0,
            "height":0,
            "x":155,
            "y":86,
            "visible":false
        }"#;
    
    monitor_global_list(Monitor):
        r#"{
            "id":"C:iLcT6auGWSFFs={#Bt",
            "mode":"list",
            "opcode":"data_listcontents",
            "params":{"LIST":"global list"},
            "spriteName":null,
            "value":["dck"],
            "width":0,
            "height":0,
            "x":5,
            "y":113,
            "visible":true
        }"#;
    
    monitor_default_vars(Vec<Monitor>):
        &from_file("src/test_json/monitor_default_vars_testcase.json");
        

    costume(Costume):
        r#"{
            "name":"costume1",
            "dataFormat":"svg",
            "assetId":"cd21514d0531fdffb22204e0ec5ed84a",
            "md5ext":"cd21514d0531fdffb22204e0ec5ed84a.svg",
            "rotationCenterX":240,
            "rotationCenterY":180
        }"#;
    
    costume_float(Costume):
        r#"{
            "assetId":"00f1913f7e7ddb13a4ba3631a172094d",
            "name":"costume2",
            "bitmapResolution":1,
            "md5ext":"00f1913f7e7ddb13a4ba3631a172094d.svg",
            "dataFormat":"svg",
            "rotationCenterX":37.525696938976154,
            "rotationCenterY":47.345544038121545
        }"#;

    sound(Sound):
        r#"{
            "name":"pop",
            "assetId":"83a9787d4cb6f3b7632b4ddfebf74367",
            "dataFormat":"wav",
            "format":"",
            "rate":48000,
            "sampleCount":1123,
            "md5ext":"83a9787d4cb6f3b7632b4ddfebf74367.wav"
        }"#;
    
    number1(Number): r#"25"#;
    number2(Number): r#"-25"#;
    number3(Number): r#"234.23498327"#;

    value_int_string(Value): r#""230""#;
    value_float_string(Value): r#""69.8""#;
    value_string(Value): r#""double door""#;
    value_number1(Value): r#"0"#;
    value_number2(Value): r#"25"#;
    value_number3(Value): r#"-40"#;
    value_number4(Value): r#"89.56984"#;
    value_number5(Value): r#"-23948.2391"#;

    variable1(Variable): r#"["local var int","23"]"#;
    variable2(Variable): r#"["local var float","9.6"]"#;
    variable3(Variable): r#"["local var str","cock n balls"]"#;
    variable4(Variable): r#"["local var float int","40.0"]"#;
    variable5(Variable): r#"["global var int",0]"#;
    variable_cloud(Variable): r#"["cloud", 25, true]"#;

    list1(List): r#"["global list",["dck"]]"#;
    list2(List): r#"["local list",["thing","balss","23"]]"#;
    
    broadcast(Broadcast): r#""some broadcast""#;

    comment1(Comment): r#"{
        "blockId":"MoiPT~h_k~gQqU4lDh~?",
        "x":332.41550925925924,
        "y":184.14814814814895,
        "width":200,
        "height":200,
        "minimized":false,
        "text":"test comment 1"
    }"#;
    comment2(Comment): r#"{
        "blockId":"X)Lwe0v2P_:XU$,?)ElJ",
        "x":317.8115442064073,
        "y":435.1111111111119,
        "width":200,
        "height":200,
        "minimized":false,
        "text":"test deez nut"
    }"#;
    comment3(Comment): r#"{
        "blockId":null,
        "x":585.185185185185,
        "y":210.37037037037,
        "width":200,
        "height":200,
        "minimized":false,
        "text":"floating comment"
    }"#;

    block_input_value(Vec<BlockInputValue>): r##"[
        [
            4,
            "12"
        ],[
            9,
            "#FF00FF"
        ],[
            10,
            "a string"
        ],[
            11,
            "hear me out boys",
            "asdlhjwejkhj34hiys9y"
        ],[
            12,
            "a variable",
            "gkuehr3r8y87ayrgjdfb"
        ],[
            12,
            "a variable",
            "gkuehr3r8y87ayrgjdfb",
            45.324234,
            90.349283473
        ],[
            13,
            "a list",
            "gkuehr3r8y87ayrgjdfb"
        ],[
            13,
            "a list",
            "gkuehr3r8y87ayrgjdfb",
            45.324234,
            90.349283473
        ]
    ]"##;

    block1(StringHashMap<Block>):
    r##"{
        "6l33?jCu4;a#[)W-FL0:": {
            "opcode": "motion_movesteps",
            "next": "a6s8L|+X=vAM([R^%},K",
            "parent": "Mxi#ehRbzX(!OvVL-~WS",
            "inputs": {
                "STEPS": [
                    3,
                    "m|LElVUoVOi#^}5g)?Jb",
                    [
                        4,
                        "10"
                    ]
                ]
            },
            "fields": {
                "TEST": [
                    "balls"
                ]
            },
            "shadow": false,
            "topLevel": false
        },
        "0LJ7hD[DXu#Y/$VAQjz7": {
            "opcode": "operator_mathop",
            "next": null,
            "parent": "uhZWtSzTDiwIF;.+kD)t",
            "inputs": {
                "NUM": [
                    1,
                    [
                        4,
                        "1"
                    ]
                ]
            },
            "fields": {
                "OPERATOR": [
                    "abs",
                    null
                ]
            },
            "shadow": false,
            "topLevel": false
        }
    }"##;

    block_general(StringHashMap<Block>):
        &from_file("src/test_json/general_block_testcase.json");

    block_procedural(StringHashMap<Block>):
        &from_file("src/test_json/procedural_block_testcase.json");

    block_mutation(Vec<BlockMutation>):
        &from_file("src/test_json/mutations.json");
    
    block_control_stop(StringHashMap<Block>):
        &from_file("src/test_json/control_stop_block_testcase.json");
    
    intermediate_test(intermediate::Project):
        &from_file("src/test_json/intermediate_testcase.json");
}