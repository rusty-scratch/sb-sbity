use super::*;

test_json!{
    Costume {
        costume =>
            r#"{
                "name":"costume1",
                "dataFormat":"svg",
                "assetId":"cd21514d0531fdffb22204e0ec5ed84a",
                "md5ext":"cd21514d0531fdffb22204e0ec5ed84a.svg",
                "rotationCenterX":240,
                "rotationCenterY":180
            }"#,
    
        costume_float =>
            r#"{
                "assetId":"00f1913f7e7ddb13a4ba3631a172094d",
                "name":"costume2",
                "bitmapResolution":1,
                "md5ext":"00f1913f7e7ddb13a4ba3631a172094d.svg",
                "dataFormat":"svg",
                "rotationCenterX":37.525696938976154,
                "rotationCenterY":47.345544038121545
            }"#
    }

    Sound {
        sound =>
            r#"{
                "name":"pop",
                "assetId":"83a9787d4cb6f3b7632b4ddfebf74367",
                "dataFormat":"wav",
                "format":"",
                "rate":48000,
                "sampleCount":1123,
                "md5ext":"83a9787d4cb6f3b7632b4ddfebf74367.wav"
            }"#
    }
}
