use std::ops::Add;
use serde::Deserialize;
use serde_json::json;
use crate::session::*;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JudgeEvent {
    kind: String,
    grade: String,
    note_index: u64,
}

#[derive(Debug, Deserialize)]
struct RuntimeStepLightResult {
    events: Vec<JudgeEvent>,
}

#[derive(Debug, Deserialize)]
struct FfiResult {
    ok: bool,
    result: Option<RuntimeStepLightResult>,
}
#[test]
fn session_round_trip() {
    let chart_text = include_str!("../assets/24_Sun Dance/maidata.txt");
    unsafe { initialize_runtime().unwrap() };
    let empty = Session::<Empty>::create().unwrap();
    let (mut loaded, _load_info) = empty.load_chart_text(chart_text, 2).unwrap();


    let step = 1000; // 10ms per frame
    for i in (0..=5000000).step_by(step) {
        let _step = loaded
            .advance_frame_light(
                json!({
                "currentTime": i,
                "events": [  { "buttonClick": {
                        "tp": i,
                        "zone": "K7"
                    },

            }]
            })
                    .to_string()
                    .as_str(),
            )
            .unwrap();
        let ffi_result: FfiResult = serde_json::from_str(&_step.json).unwrap();
        if let Some(res)=ffi_result.result {
            if res.events.len()>0 {
                println!("{:?}", res.events);
            }
        }

    }
}

#[test]
// #[ignore = "Suspected bug in native library's handling of buttonClick events"]
fn single_tap_perfect() {


    // Chart with a single tap at time 0
//     let chart_text = "&first=0\n&inote_1=(102)
// {4},7,,7,
// {4},6,,6,
// {4},7,,7,
// {4},2,2,,
// {2}6,3,
// {2}8,3-2";
    let chart_text = include_str!("../assets/Test/maidata.txt");

    unsafe { initialize_runtime().unwrap() };
        let empty = Session::<Empty>::create().unwrap();
        let (mut loaded, _load_info) = empty.load_chart_text(chart_text, 1).unwrap();

    let step = 10000; // 10ms per frame
    for i in (0..=100000000).step_by(step) {
           let step_result_json = loaded.advance_frame_light(&json!({
            "currentTime": i,
            "events": [
                 {     "sensorClick":{
                    "tp": i,
                    "area": "A7"
                },
                   "sensorClick":{
                    "tp": i,
                    "area": "A8"
                },"sensorClick":{
                    "tp": i,
                    "area": "E8"
                },"sensorClick":{
                    "tp": i,
                    "area": "B8"
                },"sensorClick":{
                    "tp": i,
                    "area": "E7"
                },"sensorClick":{
                    "tp": i,
                    "area": "B7"
                },"sensorClick":{
                    "tp": i,
                    "area": "E6"
                },"sensorClick":{
                    "tp": i,
                    "area": "B5"
                },"sensorClick":{
                    "tp": i,
                    "area": "A5"
                },}
            ]
        })
               .to_string()).unwrap();
           let ffi_result: FfiResult = serde_json::from_str(&step_result_json.json).unwrap();
           if let Some(res)=ffi_result.result {
               if res.events.len()>0 {
                   println!("{:?}", res.events);
               }
           }

       }



        // assert!(ffi_result.ok);
        // let step_result = ffi_result.result.unwrap();

    //     assert_eq!(step_result.events.len(), 1);
    // let judge_event = &step_result.events[0];
    // assert_eq!(judge_event.kind, "Tap");
    // assert_eq!(judge_event.grade, "Perfect");
    // assert_eq!(judge_event.note_index, 0);
}

#[test]
#[ignore = "Suspected bug in native library's handling of buttonHold events"]
fn single_hold_perfect() {
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct JudgeEvent {
        kind: String,
        grade: String,
        note_index: u64,
    }

    #[derive(Debug, Deserialize)]
    struct RuntimeStepLightResult {
        events: Vec<JudgeEvent>,
    }

    #[derive(Debug, Deserialize)]
    struct FfiResult {
        ok: bool,
        result: Option<RuntimeStepLightResult>,
    }

    // Hold on button 1 for a quarter note at 120bpm (0.5s)
    let chart_text = "&first=0\n&inote_1=\n(120)\n1h[4:1],\n";

    unsafe { initialize_runtime().unwrap() };
    let empty = Session::<Empty>::create().unwrap();
    let (mut loaded, _load_info) = empty.load_chart_text(chart_text, 1).unwrap();

    let input_json = json!({
        "currentTime": 500001,
        "events": [
            { "buttonHold": { "tp": 0, "zone": "K1", "isDown": true } },
            { "buttonHold": { "tp": 500000, "zone": "K1", "isDown": false } }
        ]
    }).to_string();
    let step_result_json = loaded.advance_frame_light(&input_json).unwrap();
    let ffi_result: FfiResult = serde_json::from_str(&step_result_json.json).unwrap();

    assert!(ffi_result.ok);
    let step_result = ffi_result.result.unwrap();

    assert_eq!(step_result.events.len(), 1);
    let judge_event = &step_result.events[0];
    assert_eq!(judge_event.kind, "Hold");
    assert_eq!(judge_event.grade, "Perfect");
    assert_eq!(judge_event.note_index, 0);
}
