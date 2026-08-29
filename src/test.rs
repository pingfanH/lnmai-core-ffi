use crate::api;
use crate::session::*;
use crate::types::*;
use crate::{
    lean_mk_string, lnmai_parse_frontend_chart_json, lnmai_parse_frontend_inspection_chart_json,
    lnmai_parse_frontend_semantic_chart_json, lnmai_parse_lowered_chart_json,
    lnmai_parse_normalized_chart_json,
};
use lean_sys::{lean_object, lean_string_cstr};
use serde_json::json;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::sync::{Mutex, OnceLock};

fn test_guard() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poison| poison.into_inner())
}

fn ensure_runtime() {
    static INIT: OnceLock<()> = OnceLock::new();
    INIT.get_or_init(|| unsafe { initialize_runtime().unwrap() });
}

fn single_judge_count(grade: JudgeGrade, count: u64) -> JudgeCounts {
    let mut counts = BTreeMap::new();
    counts.insert(grade, count);
    counts
}

fn legacy_touch_mode_json_key() -> String {
    ["use", "Button", "Ring", "For", "Touch"].concat()
}

fn call_string_ffi(f: impl FnOnce(*mut lean_object) -> *mut lean_object, input: &str) -> String {
    let c = CString::new(input).unwrap();
    let content = unsafe { lean_mk_string(c.as_ptr()) };
    // Exported Lean functions consume owned Lean object arguments.
    let result = f(content);
    unsafe {
        let ptr = lean_string_cstr(result);
        let value = CStr::from_ptr(ptr as *const i8)
            .to_string_lossy()
            .into_owned();
        lean_sys::lean_dec_ref(result);
        value
    }
}

#[test]
fn ffi_version_roundtrips() {
    let _guard = test_guard();
    ensure_runtime();

    let version = api::ffi_version().unwrap();
    assert_eq!(version.abi_version, 1);
    assert_eq!(version.schema, "lnmai-core-ffi-json");
}

#[test]
fn parser_outputs_roundtrip_into_rust_types() {
    let _guard = test_guard();
    let chart_text = include_str!("../assets/24_Sun Dance/maidata.txt");
    ensure_runtime();

    let frontend_json = call_string_ffi(
        |content| unsafe { lnmai_parse_frontend_chart_json(content, 6) },
        chart_text,
    );
    let frontend: crate::types::FfiEnvelope<FrontendChartResult> =
        serde_json::from_str(&frontend_json).unwrap();
    assert!(frontend.ok);

    let semantic_json = call_string_ffi(
        |content| unsafe { lnmai_parse_frontend_semantic_chart_json(content, 6) },
        chart_text,
    );
    let semantic: crate::types::FfiEnvelope<FrontendSemanticChart> =
        serde_json::from_str(&semantic_json).unwrap();
    assert!(semantic.ok);

    let inspection_json = call_string_ffi(
        |content| unsafe { lnmai_parse_frontend_inspection_chart_json(content, 6) },
        chart_text,
    );
    let inspection: crate::types::FfiEnvelope<FrontendChartInspection> =
        serde_json::from_str(&inspection_json).unwrap();
    assert!(inspection.ok);

    let normalized_json = call_string_ffi(
        |content| unsafe { lnmai_parse_normalized_chart_json(content, 6) },
        chart_text,
    );
    let normalized: crate::types::FfiEnvelope<NormalizedChart> =
        serde_json::from_str(&normalized_json).unwrap();
    assert!(normalized.ok);

    let lowered_json = call_string_ffi(
        |content| unsafe { lnmai_parse_lowered_chart_json(content, 6) },
        chart_text,
    );
    let lowered: crate::types::FfiEnvelope<ChartSpec> =
        serde_json::from_str(&lowered_json).unwrap();
    assert!(lowered.ok);
}

#[test]
fn typed_api_helpers_roundtrip_runtime_structures() {
    let _guard = test_guard();
    let chart_text = include_str!("../assets/24_Sun Dance/maidata.txt");
    ensure_runtime();

    let lowered = api::parse_lowered_chart(chart_text, 6).unwrap();
    assert!(!lowered.slides.is_empty());

    let tactic = api::default_tactic_from_chart(&lowered).unwrap();
    assert!(!tactic.events.is_empty());

    let state = api::build_game_state(&lowered).unwrap();
    assert_eq!(state.current_time, 0);
    assert!(state.score.total_base > 0);
    assert!(state.score.max_dx_score > 0);
    assert_eq!(
        state.score.dx_score_remaining(),
        state.score.max_dx_score as i64
    );
    assert_eq!(state.score.combo_state(), ComboState::None);
    let state_json = serde_json::to_value(&state).unwrap();
    assert!(state_json.get(legacy_touch_mode_json_key()).is_none());
    assert_eq!(state.note_fast_late_display, JudgeDisplayOption::All);
    assert_eq!(state.break_fast_late_display, JudgeDisplayOption::Disable);
    assert_eq!(state_json["noteFastLateDisplay"], json!("All"));
    assert_eq!(state_json["breakFastLateDisplay"], json!("Disable"));

    let batch = TimedInputBatch {
        current_time: 0,
        events: vec![],
    };
    let step = api::step_game_state(&state, &batch).unwrap();
    assert_eq!(step.state.current_time, 0);
    assert!(step.state.score.total_base > 0);
    let step_state_json = serde_json::to_value(&step.state).unwrap();
    assert!(step_state_json.get(legacy_touch_mode_json_key()).is_none());
}

#[test]
fn rust_score_helpers_match_core_combo_categories() {
    let score = ScoreState {
        combo: 3,
        p_combo: 1,
        c_p_combo: 1,
        total_base: 3_500,
        total_extra: 100,
        earned_base: 3_400,
        earned_extra: 50,
        lost_base: 100,
        lost_extra: 50,
        dx_score: -3,
        max_dx_score: 9,
        fast_count: 0,
        late_count: 2,
        counts: NoteTypeJudgeCounts {
            tap_count: single_judge_count(JudgeGrade::LateGreat2nd, 1),
            hold_count: BTreeMap::new(),
            slide_count: BTreeMap::new(),
            touch_count: BTreeMap::new(),
            break_count: single_judge_count(JudgeGrade::LatePerfect3rd, 1),
        },
    };
    assert_eq!(score.combo_state(), ComboState::FCPlus);
    assert_eq!(score.dx_score_remaining(), 6);

    let ap_plus = ScoreState {
        counts: NoteTypeJudgeCounts {
            tap_count: single_judge_count(JudgeGrade::Perfect, 1),
            hold_count: BTreeMap::new(),
            slide_count: BTreeMap::new(),
            touch_count: BTreeMap::new(),
            break_count: BTreeMap::new(),
        },
        ..score
    };
    assert_eq!(ap_plus.combo_state(), ComboState::APPlus);
}

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
/// - `JudgeEvent { kind: Slide, is_break, grade, diff, note_index }` is emitted
///   once when the slide finishes judgment (after the wait countdown).
///
/// - `note_index` is the sole identifier linking all commands to a
///   specific slide. The Rust side must use it to map back to the
///   chart's slide data.
#[test]
fn slide_judgment_parse_instance() {
    let _guard = test_guard();
    let chart_text = include_str!("../assets/24_Sun Dance/maidata.txt");
    ensure_runtime();
    let empty = Session::<Empty>::create().unwrap();
    let (mut loaded, _load_info) = empty.load_chart_text(chart_text, 6).unwrap();

    let lowered_chart: ChartSpec = loaded
        .get_lowered_chart_json()
        .unwrap()
        .decode_result()
        .unwrap();
    assert!(!lowered_chart.slides.is_empty());

    let state: GameState = loaded.get_state_json().unwrap().decode_result().unwrap();
    assert_eq!(state.current_time, 0);

    // Step 1: Advance at time 0 with no input.
    // This lets notes that start at t=0 get processed.
    let step0 = loaded
        .advance_frame_light(&json!({ "currentTime": 0, "events": [] }).to_string())
        .unwrap();

    // Parse the raw JSON envelope into our typed result.
    let envelope: FfiResult = serde_json::from_str(&step0.json).unwrap();
    assert!(envelope.ok);

    let _result0: RuntimeStepLightResult =
        serde_json::from_value(envelope.result.unwrap()).unwrap();

    // Step 2: Simulate a slide being touched.
    // Hold sensor A1 at t=500ms (500000 microseconds) to start progressing a slide.
    let batch = TimedInputBatch {
        current_time: 500_000,
        events: vec![TimedInputEvent::SensorHold {
            tp: 500_000,
            area: SensorArea::A1,
            is_down: true,
        }],
    };
    let step1 = loaded
        .advance_frame_light(&serde_json::to_string(&batch).unwrap())
        .unwrap();

    let envelope1: FfiResult = serde_json::from_str(&step1.json).unwrap();
    let result1: RuntimeStepLightResult =
        serde_json::from_value(envelope1.result.unwrap()).unwrap();

    // Inspect render commands — these tell the renderer what changed.
    for cmd in &result1.render_commands {
        match cmd {
            // Individual area completed: hide the slide bar up to arrow `end_index`.
            RenderCommand::HideSlideBars {
                note_index,
                end_index,
            } => {
                eprintln!(
                    "slide noteIndex={} area completed, hide bar up to arrow {}",
                    note_index, end_index
                );
            }
            // Overall remaining count changed (emitted only on change, not every frame).
            RenderCommand::UpdateSlideProgress {
                note_index,
                remaining,
            } => {
                eprintln!(
                    "slide noteIndex={} progress updated, {} areas remaining",
                    note_index, remaining
                );
            }
            // Wifi/connected slide track-specific progress.
            RenderCommand::UpdateSlideTrackProgress {
                note_index,
                track_index,
                remaining,
            } => {
                eprintln!(
                    "slide noteIndex={} track={} progress, {} areas remaining",
                    note_index, track_index, remaining
                );
            }
            // Slide fully ended (judged or too-late).
            RenderCommand::HideAllSlideBars { note_index } => {
                eprintln!("slide noteIndex={} all bars hidden (ended)", note_index);
            }
            // Non-slide render command (e.g., ShowJudgeResult for taps/break taps).
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
        if let AudioCommand::PlaySlideCue {
            note_index,
            track_index,
            is_break,
            at_time,
        } = cmd
        {
            eprintln!(
                "slide noteIndex={} track={} break={} cue at t={}μs",
                note_index, track_index, is_break, at_time
            );
        }
    }

    let (_empty, _unload_info) = loaded.unload_chart().unwrap();
}

#[test]
fn prestart_slide_empty_frames_stay_dormant_through_session_ffi() {
    let _guard = test_guard();
    ensure_runtime();

    let chart = ChartSpec {
        taps: vec![],
        holds: vec![],
        touches: vec![],
        touch_holds: vec![],
        slide_heads: vec![SlideHeadChartNote {
            timing: 360_000,
            slot: OuterSlot::S1,
            is_break: false,
            is_ex: false,
            logical_slide_id: 600,
            note_index: 599,
        }],
        slides: vec![SlideChartNote {
            head_timing: 360_000,
            slot: OuterSlot::S1,
            length: 500_000,
            start_timing: 360_000,
            slide_kind: RuntimeSlideKind::Single,
            is_classic: false,
            is_slide_no_head: false,
            is_conn_slide: false,
            parent_note_index: None,
            is_group_head: false,
            is_group_end: false,
            parent_finished: false,
            parent_pending_finish: false,
            total_judge_queue_len: 1,
            track_count: 1,
            judge_at: Some(860_000),
            is_break: false,
            is_ex: false,
            multiple: 1,
            logical_slide_id: 600,
            note_index: 600,
            judge_queues: vec![vec![SlideAreaSpec {
                target_areas: vec![SensorArea::A1],
                policy: AreaPolicy::Or,
                is_last: true,
                is_skippable: true,
                arrow_progress_when_on: 0,
                arrow_progress_when_finished: 1,
            }]],
            debug_simai: None,
        }],
        slide_skipping: Some(true),
    };
    let chart_json = serde_json::to_string(&chart).unwrap();
    let empty = Session::<Empty>::create().unwrap();
    let (mut loaded, _load_info) = empty.load_chart_json(&chart_json).unwrap();

    for current_time in [0, 16_667, 33_334, 50_001, 66_668, 83_335, 100_002] {
        let batch = json!({ "currentTime": current_time, "events": [] }).to_string();
        let result: RuntimeStepLightResult = loaded
            .advance_frame_light(&batch)
            .unwrap()
            .decode_result()
            .unwrap();
        assert_eq!(result.current_time, current_time);
        assert!(result.events.is_empty());
        assert!(result.audio_commands.is_empty());
        assert!(result.render_commands.is_empty());
    }

    let state: GameState = loaded.get_state_json().unwrap().decode_result().unwrap();
    assert_eq!(state.current_time, 100_002);
    assert_eq!(state.slides.len(), 1);

    let slide = &state.slides[0];
    assert!(matches!(slide.state, SlideState::Waiting));
    assert!(!slide.is_checkable);
    assert_eq!(slide.judge_queues.len(), 1);
    assert_eq!(slide.judge_queues[0].len(), 1);
    assert!(!slide.judge_queues[0][0].was_on);
    assert!(!slide.judge_queues[0][0].was_off);

    let (_empty, _unload_info) = loaded.unload_chart().unwrap();
}

#[test]
fn light_step_returns_the_updated_score() {
    let _guard = test_guard();
    ensure_runtime();

    let chart = ChartSpec {
        taps: vec![TapChartNote {
            timing: 0,
            slot: OuterSlot::S1,
            is_break: false,
            is_ex: false,
            button_queue_index: 0,
            note_index: 0,
        }],
        holds: vec![],
        touches: vec![],
        touch_holds: vec![],
        slide_heads: vec![],
        slides: vec![],
        slide_skipping: None,
    };
    let chart_json = serde_json::to_string(&chart).unwrap();
    let empty = Session::<Empty>::create().unwrap();
    let (mut loaded, _load_info) = empty.load_chart_json(&chart_json).unwrap();
    let batch = json!({
        "currentTime": 0,
        "events": [{ "buttonClick": { "tp": 0, "zone": "K1" } }]
    });

    let result: RuntimeStepLightResult = loaded
        .advance_frame_light(&batch.to_string())
        .unwrap()
        .decode_result()
        .unwrap();

    assert_eq!(result.score.combo, 1);
    assert_eq!(result.score.earned_base, 500);
    let state: GameState = loaded.get_state_json().unwrap().decode_result().unwrap();
    assert_eq!(result.score, state.score);

    let (_empty, _unload_info) = loaded.unload_chart().unwrap();
}

#[test]
fn command_and_event_break_flags_roundtrip() {
    let event_json = r#"{
      "kind": "Tap",
      "grade": "Perfect",
      "diff": 0,
      "position": { "button": "K1" },
      "noteIndex": 12,
      "isBreak": true
    }"#;
    let event: JudgeEvent = serde_json::from_str(event_json).unwrap();
    assert_eq!(event.kind, JudgeEventKind::Tap);
    assert!(event.is_break);

    let audio_json = r#"{
      "PlayJudgeSfx": {
        "noteIndex": 12,
        "kind": "Tap",
        "isBreak": true,
        "grade": "Perfect",
        "atTime": 0
      }
    }"#;
    let audio: AudioCommand = serde_json::from_str(audio_json).unwrap();
    match audio {
        AudioCommand::PlayJudgeSfx {
            kind,
            grade,
            is_break,
            at_time,
            note_index,
        } => {
            assert_eq!(kind, JudgeEventKind::Tap);
            assert_eq!(grade, JudgeGrade::Perfect);
            assert!(is_break);
            assert_eq!(at_time, 0);
            assert_eq!(note_index, 12);
        }
        _ => panic!("expected PlayJudgeSfx"),
    }
    let audio_roundtrip: serde_json::Value = serde_json::to_value(&audio).unwrap();
    let audio_expected: serde_json::Value = serde_json::from_str(audio_json).unwrap();
    assert_eq!(audio_roundtrip, audio_expected);

    let slide_cue_json = r#"{
      "PlaySlideCue": {
        "trackIndex": 0,
        "noteIndex": 72,
        "isBreak": true,
        "atTime": 0
      }
    }"#;
    let slide_cue: AudioCommand = serde_json::from_str(slide_cue_json).unwrap();
    match slide_cue {
        AudioCommand::PlaySlideCue {
            note_index,
            track_index,
            is_break,
            at_time,
        } => {
            assert_eq!(note_index, 72);
            assert_eq!(track_index, 0);
            assert!(is_break);
            assert_eq!(at_time, 0);
        }
        _ => panic!("expected PlaySlideCue"),
    }
    let slide_cue_roundtrip: serde_json::Value = serde_json::to_value(&slide_cue).unwrap();
    let slide_cue_expected: serde_json::Value = serde_json::from_str(slide_cue_json).unwrap();
    assert_eq!(slide_cue_roundtrip, slide_cue_expected);

    let render_json = r#"{
      "ShowJudgeResult": {
        "noteIndex": 12,
        "kind": "Tap",
        "isBreak": true,
        "grade": "Perfect",
        "diff": 0
      }
    }"#;
    let render: RenderCommand = serde_json::from_str(render_json).unwrap();
    match render {
        RenderCommand::ShowJudgeResult {
            kind,
            grade,
            is_break,
            diff,
            note_index,
        } => {
            assert_eq!(kind, JudgeEventKind::Tap);
            assert_eq!(grade, JudgeGrade::Perfect);
            assert!(is_break);
            assert_eq!(diff, 0);
            assert_eq!(note_index, 12);
        }
        _ => panic!("expected ShowJudgeResult"),
    }
    let render_roundtrip: serde_json::Value = serde_json::to_value(&render).unwrap();
    let render_expected: serde_json::Value = serde_json::from_str(render_json).unwrap();
    assert_eq!(render_roundtrip, render_expected);
}

#[test]
fn game_state_touch_hold_body_groups_roundtrip() {
    let _guard = test_guard();
    let chart_text = include_str!("../assets/24_Sun Dance/maidata.txt");
    ensure_runtime();

    let lowered = api::parse_lowered_chart(chart_text, 6).unwrap();
    let state = api::build_game_state(&lowered).unwrap();
    let mut state_json = serde_json::to_value(&state).unwrap();

    let touch_group_states = json!([
        {
            "groupId": 140,
            "count": 2,
            "size": 3,
            "grade": "Perfect",
            "diff": 0
        }
    ]);
    let touch_hold_body_groups = json!([
        {
            "groupId": 240,
            "memberNoteIndices": [390, 391, 392],
            "triggeredNoteIndices": [391, 392]
        }
    ]);

    state_json["touchGroupStates"] = touch_group_states.clone();
    state_json["touchHoldGroupStates"] = touch_hold_body_groups.clone();

    let decoded: GameState = serde_json::from_value(state_json).unwrap();
    assert_eq!(decoded.touch_group_states.len(), 1);
    assert_eq!(decoded.touch_group_states[0].group_id, 140);
    assert_eq!(decoded.touch_hold_group_states.len(), 1);
    assert_eq!(decoded.touch_hold_group_states[0].group_id, 240);
    assert_eq!(
        decoded.touch_hold_group_states[0].member_note_indices,
        vec![390, 391, 392]
    );
    assert_eq!(
        decoded.touch_hold_group_states[0].triggered_note_indices,
        vec![391, 392]
    );

    let encoded = serde_json::to_value(&decoded).unwrap();
    assert_eq!(encoded["touchGroupStates"], touch_group_states);
    assert_eq!(encoded["touchHoldGroupStates"], touch_hold_body_groups);
}

#[test]
fn hold_note_release_ignore_time_defaults_for_older_state_json() {
    let hold_json = json!({
        "params": {
            "judgeTiming": 0,
            "judgeOffset": 0,
            "isBreak": false,
            "isEX": false,
            "noteIndex": 77
        },
        "start": { "button": { "zone": "K1" } },
        "state": "BodyReleased",
        "length": 800000,
        "buttonQueueIndex": 0,
        "headDiff": 0,
        "headGrade": "Perfect",
        "playerReleaseTime": 16000,
        "isClassic": false,
        "isTouchHold": false,
        "touchQueueIndex": 0,
        "touchGroupSize": 1,
        "touchHoldGroupSize": 1,
        "touchHoldGroupTriggered": false
    });

    let decoded: HoldNote = serde_json::from_value(hold_json).unwrap();
    assert_eq!(decoded.release_ignore_time, 0);

    let encoded = serde_json::to_value(decoded).unwrap();
    assert_eq!(encoded["releaseIgnoreTime"], json!(0));
}

#[test]
fn lowered_touch_source_groups_roundtrip_through_typed_chart_notes() {
    let touch_json = json!({
        "timing": 0,
        "sensorPos": "A1",
        "isBreak": false,
        "sourceGroupId": 10,
        "sourceGroupIndex": 1,
        "sourceGroupSize": 3,
        "touchQueueIndex": 2,
        "touchGroupId": 4,
        "touchGroupSize": 3,
        "noteIndex": 101
    });
    let touch: TouchChartNote = serde_json::from_value(touch_json).unwrap();
    assert_eq!(touch.source_group_id, Some(10));
    assert_eq!(touch.source_group_index, Some(1));
    assert_eq!(touch.source_group_size, Some(3));

    let encoded_touch = serde_json::to_value(touch).unwrap();
    assert_eq!(encoded_touch["sourceGroupId"], json!(10));
    assert_eq!(encoded_touch["sourceGroupIndex"], json!(1));
    assert_eq!(encoded_touch["sourceGroupSize"], json!(3));

    let touch_hold_json = json!({
        "timing": 0,
        "sensorPos": "A2",
        "length": 500000,
        "isBreak": false,
        "isEX": false,
        "sourceGroupId": 20,
        "sourceGroupIndex": 0,
        "sourceGroupSize": 2,
        "touchQueueIndex": 1,
        "touchGroupId": 8,
        "touchGroupSize": 2,
        "touchHoldGroupId": 9,
        "touchHoldGroupSize": 2,
        "noteIndex": 102
    });
    let touch_hold: TouchHoldChartNote = serde_json::from_value(touch_hold_json).unwrap();
    assert_eq!(touch_hold.source_group_id, Some(20));
    assert_eq!(touch_hold.source_group_index, Some(0));
    assert_eq!(touch_hold.source_group_size, Some(2));

    let encoded_touch_hold = serde_json::to_value(touch_hold).unwrap();
    assert_eq!(encoded_touch_hold["sourceGroupId"], json!(20));
    assert_eq!(encoded_touch_hold["sourceGroupIndex"], json!(0));
    assert_eq!(encoded_touch_hold["sourceGroupSize"], json!(2));
}
