use serde_json::json;
use crate::session::*;

#[test]
fn session_round_trip() {
    let chart_text = include_str!("../assets/24_Sun Dance/maidata.txt");
    unsafe { initialize_runtime().unwrap() };
    let empty = Session::<Empty>::create().unwrap();
    let (mut loaded, _load_info) = empty.load_chart_text(chart_text, 2).unwrap();
    let _step = loaded
        .advance_frame_light(
            json!({
                "currentTime": 0,
                "events": []
            })
            .to_string()
            .as_str(),
        )
        .unwrap();
    let (_empty, _unload_info) = loaded.unload_chart().unwrap();
}
