use super::*;

// Monitor, Mode, MonitorOpCode,
// NumberName, Parameter, Slider, ListOrValue

// monitor_default_vars(Vec<Monitor>):
//     &from_file("src/test_json/monitor_default_vars_testcase.json");
test_json!{
    Mode {
        monitor_mode_default => "default",
        monitor_mode_large   => "large",
        monitor_mode_slider  => "slider",
        monitor_mode_list    => "list"
    },
    
    MonitorOpCode {
        monitor_opcode_data_variable            => "data_variable",
        monitor_opcode_data_listcontents        => "data_listcontents",
        monitor_opcode_motion_xposition         => "motion_xposition",
        monitor_opcode_motion_yposition         => "motion_yposition",
        monitor_opcode_motion_direction         => "motion_direction",
        monitor_opcode_looks_costumenumbername  => "looks_costumenumbername",
        monitor_opcode_looks_backdropnumbername => "looks_backdropnumbername",
        monitor_opcode_looks_size               => "looks_size",
        monitor_opcode_sensing_loudness         => "sensing_loudness",
        monitor_opcode_sensing_timer            => "sensing_timer",
        monitor_opcode_sensing_username         => "sensing_username"
    },
    
    Monitor {
        monitor_slider_local_int =>
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
            }"##,
    
        monitor_large_local_float =>
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
            }"#,
    
        monitor_local_float_int =>
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
            }"#,
    
        monitor_local_str =>
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
            }"#,
    
        monitor_global_int =>
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
            }"#,
    
        monitor_local_list =>
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
            }"#,
    
        monitor_global_list =>
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
            }"#
    }
}