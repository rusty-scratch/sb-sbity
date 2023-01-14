use super::*;

test_json! {
    Sprite {
        simple_sprite => r#"{
          "isStage": false,
          "name": "Sprite1",
          "variables": {},
          "lists": {},
          "broadcasts": {},
          "blocks": {},
          "comments": {},
          "currentCostume": 0,
          "costumes": [],
          "sounds": [],
          "volume": 100,
          "layerOrder": 1,
          "visible": true,
          "x": -118,
          "y": -125,
          "size": 100,
          "direction": 90,
          "draggable": false,
          "rotationStyle": "all around"
        }"#
    }
    Stage {
        simple_stage => r#"{
          "isStage": true,
          "name": "Stage",
          "variables": {},
          "lists": {},
          "broadcasts": {},
          "blocks": {},
          "comments": {},
          "currentCostume": 0,
          "costumes": [],
          "sounds": [],
          "volume": 100,
          "layerOrder": 0,
          "tempo": 60,
          "videoTransparency": 50,
          "videoState": "on",
          "textToSpeechLanguage": null
        }"#
    }
}
