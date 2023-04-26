use super::super::*;

// block_general(StringHashMap<Block>):
//     &from_file("src/test_json/general_block_testcase.json");

// block_procedural(StringHashMap<Block>):
//     &from_file("src/test_json/procedural_block_testcase.json");

// block_mutation(Vec<BlockMutation>):
//     &from_file("src/test_json/mutations.json");

// block_control_stop(StringHashMap<Block>):
//     &from_file("src/test_json/control_stop_block_testcase.json");

test_json! {
    Variable {
        variable_local_var_int       => r#"["local var int","23"]"#,
        variable_local_var_float     => r#"["local var float","9.6"]"#,
        variable_local_var_str       => r#"["local var str","cock n balls"]"#,
        variable_local_var_float_int => r#"["local var float int","40.0"]"#,
        variable_global_var_int      => r#"["global var int",0]"#,
        variable_cloud               => r#"["cloud", 26, true]"#
    }

    List {
        list_global => r#"["global list",["dck"]]"#,
        list_local => r#"["local list",["thing","balss","23"]]"#
    }

    Broadcast {
        broadcast => r#""some broadcast""#
    }

    Comment {
        comment1 => r#"{
            "blockId":"MoiPT~h_k~gQqU4lDh~?",
            "x":332.41550925925924,
            "y":184.14814814814895,
            "width":200,
            "height":200,
            "minimized":false,
            "text":"test comment 1"
        }"#,
        comment2 => r#"{
            "blockId":"X)Lwe0v2P_:XU$,?)ElJ",
            "x":317.8115442064073,
            "y":435.1111111111119,
            "width":200,
            "height":200,
            "minimized":false,
            "text":"test deez nut"
        }"#,
        comment3 => r#"{
            "blockId":null,
            "x":585.185185185185,
            "y":210.37037037037,
            "width":200,
            "height":200,
            "minimized":false,
            "text":"floating comment"
        }"#
    }
}
