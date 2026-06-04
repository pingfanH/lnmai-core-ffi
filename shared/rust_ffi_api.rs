use crate::raw;
use crate::session::{LnmaiError, Result};
use crate::types;
use lean_sys::{lean_object, lean_string_cstr};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ffi::{CStr, CString};

fn mk_lean_string(content: &str) -> *mut lean_object {
    let c = CString::new(content).expect("content contains interior NUL");
    unsafe { raw::lean_mk_string(c.as_ptr()) }
}

fn into_string(result: *mut lean_object) -> String {
    unsafe {
        let ptr = lean_string_cstr(result);
        let value = CStr::from_ptr(ptr as *const i8).to_string_lossy().into_owned();
        lean_sys::lean_dec_ref(result);
        value
    }
}

fn decode_envelope<T: DeserializeOwned>(json: String) -> Result<T> {
    let envelope: types::FfiEnvelope<T> =
        serde_json::from_str(&json).map_err(|_| LnmaiError { json: json.clone() })?;
    if envelope.ok {
        envelope.result.ok_or(LnmaiError { json })
    } else {
        Err(LnmaiError { json })
    }
}

fn call_parse<T: DeserializeOwned>(
    content: &str,
    level_index: u32,
    f: unsafe extern "C" fn(*mut lean_object, u32) -> *mut lean_object,
) -> Result<T> {
    let content_obj = mk_lean_string(content);
    let json = unsafe { into_string(f(content_obj, level_index)) };
    decode_envelope(json)
}

fn call_json_input<I: Serialize, O: DeserializeOwned>(
    input: &I,
    f: unsafe extern "C" fn(*mut lean_object) -> *mut lean_object,
) -> Result<O> {
    let input_json = serde_json::to_string(input)
        .map_err(|err| LnmaiError { json: err.to_string() })?;
    let input_obj = mk_lean_string(&input_json);
    let json = unsafe { into_string(f(input_obj)) };
    decode_envelope(json)
}

pub fn parse_frontend_chart(content: &str, level_index: u32) -> Result<types::FrontendChartResult> {
    call_parse(content, level_index, raw::lnmai_parse_frontend_chart_json)
}

pub fn parse_frontend_semantic_chart(
    content: &str,
    level_index: u32,
) -> Result<types::FrontendSemanticChart> {
    call_parse(content, level_index, raw::lnmai_parse_frontend_semantic_chart_json)
}

pub fn parse_frontend_inspection_chart(
    content: &str,
    level_index: u32,
) -> Result<types::FrontendChartInspection> {
    call_parse(content, level_index, raw::lnmai_parse_frontend_inspection_chart_json)
}

pub fn parse_normalized_chart(content: &str, level_index: u32) -> Result<types::NormalizedChart> {
    call_parse(content, level_index, raw::lnmai_parse_normalized_chart_json)
}

pub fn parse_lowered_chart(content: &str, level_index: u32) -> Result<types::ChartSpec> {
    call_parse(content, level_index, raw::lnmai_parse_lowered_chart_json)
}

/// Builds runtime state from a lowered chart payload.
///
/// The lowered schema now uses explicit `slideHeads` plus slide-body `slides`.
/// Matching lowered head/body objects are linked by `logicalSlideId`.
/// Lowered slide bodies serialize `headTiming` as the body-side preserved head
/// anchor.
pub fn build_game_state(chart_spec: &types::ChartSpec) -> Result<types::GameState> {
    call_json_input(chart_spec, raw::lnmai_build_game_state_json)
}

pub fn default_tactic_from_chart(
    chart_spec: &types::ChartSpec,
) -> Result<types::ManualTacticSequence> {
    call_json_input(chart_spec, raw::lnmai_default_tactic_from_chart_json)
}

pub fn step_game_state(
    state: &types::GameState,
    batch: &types::TimedInputBatch,
) -> Result<types::RuntimeStepResult> {
    let state_json = serde_json::to_string(state)
        .map_err(|err| LnmaiError { json: err.to_string() })?;
    let batch_json = serde_json::to_string(batch)
        .map_err(|err| LnmaiError { json: err.to_string() })?;
    let state_obj = mk_lean_string(&state_json);
    let batch_obj = mk_lean_string(&batch_json);
    let json = unsafe { into_string(raw::lnmai_step_game_state_json(state_obj, batch_obj)) };
    decode_envelope(json)
}
