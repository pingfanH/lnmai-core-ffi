use serde_json::json;
use crate::session::*;
use crate::types::*;

/// Demonstrates how slide judgment data flows from Lean to Rust.
///
/// This test loads a real chart, steps frames with sensor input that
/// triggers slide progress, and parses the resulting JSON into typed
/// Rust structs. It shows exactly how:
///
/// - `RenderCommand::HideSlideBars { note_index, end_index }` marks
///   individual slide areas as completed (the player's hand passed through).
///
/// - `RenderCommand::UpdateSlideProgress { note_index, remaining }` is
///   emitted only when the remaining area count changes (not every frame).
///
/// - `JudgeEvent { kind: Slide, grade, diff, note_index }` is emitted
///   once when the slide finishes judgment (after the wait countdown).
///
/// - `note_index` is the sole identifier linking all commands to a
///   specific slide. The Rust side must use it to map back to the
///   chart's slide data.
#[test]
fn slide_judgment_parse_instance() {
    let chart_text = include_str!("../assets/24_Sun Dance/maidata.txt");
    unsafe { initialize_runtime().unwrap() };
    let empty = Session::<Empty>::create().unwrap();
    let (mut loaded, _load_info) = empty.load_chart_text(chart_text, 6).unwrap();

    // Step 1: Advance at time 0 with no input.
    // This lets notes that start at t=0 get processed.
    let step0 = loaded.advance_frame_light(
        &json!({ "currentTime": 0, "events": [] }).to_string(),
    ).unwrap();

    // Parse the raw JSON envelope into our typed result.
    let envelope: FfiResult = serde_json::from_str(&step0.json).unwrap();
    assert!(envelope.ok);

    let _result0: RuntimeStepLightResult =
        serde_json::from_value(envelope.result.unwrap()).unwrap();

    // Step 2: Simulate a slide being touched.
    // Hold sensor A1 at t=500ms (500000 microseconds) to start progressing a slide.
    let step1 = loaded.advance_frame_light(
        &json!({
            "currentTime": 500_000,
            "events": [{
                "tag": "sensorHold",
                "tp": 500_000,
                "area": "A1",
                "isDown": true
            }]
        }).to_string(),
    ).unwrap();

    let envelope1: FfiResult = serde_json::from_str(&step1.json).unwrap();
    let result1: RuntimeStepLightResult =
        serde_json::from_value(envelope1.result.unwrap()).unwrap();

    // Inspect render commands — these tell the renderer what changed.
    for cmd in &result1.render_commands {
        match cmd {
            // Individual area completed: hide the slide bar up to arrow `end_index`.
            RenderCommand::HideSlideBars { note_index, end_index } => {
                eprintln!(
                    "slide noteIndex={} area completed, hide bar up to arrow {}",
                    note_index, end_index
                );
            }
            // Overall remaining count changed (emitted only on change, not every frame).
            RenderCommand::UpdateSlideProgress { note_index, remaining } => {
                eprintln!(
                    "slide noteIndex={} progress updated, {} areas remaining",
                    note_index, remaining
                );
            }
            // Wifi/connected slide track-specific progress.
            RenderCommand::UpdateSlideTrackProgress { note_index, track_index, remaining } => {
                eprintln!(
                    "slide noteIndex={} track={} progress, {} areas remaining",
                    note_index, track_index, remaining
                );
            }
            // Slide fully ended (judged or too-late).
            RenderCommand::HideAllSlideBars { note_index } => {
                eprintln!("slide noteIndex={} all bars hidden (ended)", note_index);
            }
            // Non-slide render command (e.g., ShowJudgeResult for taps).
            _ => {}
        }
    }

    // Inspect judge events — a Slide event appears only once per slide.
    for evt in &result1.events {
        if evt.kind == JudgeEventKind::Slide {
            eprintln!(
                "SLIDE JUDGED: noteIndex={} grade={:?} diff={}μs",
                evt.note_index, evt.grade, evt.diff
            );
            // `evt.position` tells you which sensor area the slide was at.
            // For slides this is typically the last area in the path.
            eprintln!("  position={:?}", evt.position);
        }
    }

    // Inspect audio commands — PlaySlideCue fires when a new track area activates.
    for cmd in &result1.audio_commands {
        if let AudioCommand::PlaySlideCue { note_index, track_index, at_time } = cmd {
            eprintln!(
                "slide noteIndex={} track={} cue at t={}μs",
                note_index, track_index, at_time
            );
        }
    }

    let (_empty, _unload_info) = loaded.unload_chart().unwrap();
}
