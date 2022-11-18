use super::*;

use Block;
use BlockField;
use BlockInput;
use BlockInputValue;
use BlockMutation;
use BlockMutationEnum;
use ShadowInputType;

test_json!{
    BlockInputValue {
        block_input_value_number =>
            r#"[
                4,
                "12"
            ]"#,
        block_input_value_color =>
            r##"[
                9,
                "#FF00FF"
            ]"##,
        block_input_value_string =>
            r#"[
                10,
                "a string"
            ]"#,
        block_input_value_broadcast =>
            r#"[
                11,
                "hear me out boys",
               "asdlhjwejkhj34hiys9y"
            ]"#,
        block_input_value_variable =>
            r#"[
                12,
                "a variable",
                "gkuehr3r8y87ayrgjdfb"
            ]"#,
        block_input_value_variable_pos =>
            r#"[
                12,
                "a variable",
                "gkuehr3r8y87ayrgjdfb",
                45.324234,
                90.349283473
            ]"#,
        block_input_value_list =>
            r#"[
                13,
                "a list",
                "gkuehr3r8y87ayrgjdfb"
            ]"#,
        block_input_value_list_pos =>
            r#"[
                13,
                "a list",
                "gkuehr3r8y87ayrgjdfb",
                45.324234,
                90.349283473
            ]"#
    }
    
    ShadowInputType {
        shadow_input_shadow => "1",
        shadow_input_no_shadow => "2",
        shadow_input_shadow_obscured => "3"
    }
    
    Block {
        block1 => r#"{
            "opcode": "motion_movesteps",
            "next": "id to next block",
            "parent": "id to parent block",
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
        }"#,
        block2 => r#"{
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
        }"#
    }
    
    BlockField {
        
    }
}