use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type TimePoint = i64;
pub type Duration = i64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FfiErrorInfo {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct FfiEnvelope<T = serde_json::Value> {
    pub ok: bool,
    #[serde(default)]
    pub result: Option<T>,
    #[serde(default)]
    pub error: Option<FfiErrorInfo>,
    #[serde(default)]
    pub details: Option<serde_json::Value>,
}

pub type FfiResult = FfiEnvelope<serde_json::Value>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum JudgeGrade {
    Miss,
    LateGood,
    LateGreat3rd,
    LateGreat2nd,
    LateGreat,
    LatePerfect3rd,
    LatePerfect2nd,
    Perfect,
    FastPerfect2nd,
    FastPerfect3rd,
    FastGreat,
    FastGreat2nd,
    FastGreat3rd,
    FastGood,
    TooFast,
}

pub type JudgeCounts = BTreeMap<JudgeGrade, u64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum JudgeEventKind {
    Tap,
    Hold,
    Slide,
    Touch,
    Break,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RuntimeSlideKind {
    Single,
    Wifi,
    ConnPart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AreaPolicy {
    Or,
    And,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum JudgeStyle {
    Default,
    Maji,
    Gachi,
    Gori,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SensorArea {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ButtonZone {
    K1, K2, K3, K4, K5, K6, K7, K8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OuterSlot {
    S1, S2, S3, S4, S5, S6, S7, S8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rational {
    pub num: i64,
    pub den: u64,
    pub decimal: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimePos {
    #[serde(default)]
    pub button: Option<ButtonZone>,
    #[serde(default)]
    pub sensor: Option<SensorArea>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcePos {
    pub line: u64,
    pub column: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceSpan {
    pub start: SourcePos,
    pub stop: SourcePos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParseErrorKind {
    InvalidSyntax,
    InvalidShape,
    InvalidEndPosition,
    InvalidTurnPosition,
    InvalidChainTiming,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub raw_text: String,
    pub message: String,
    #[serde(default)]
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SyntaxSlideKind {
    Line,
    Circle,
    V,
    Turn,
    Pq,
    Ppqq,
    S,
    Wifi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalSlideShape {
    #[serde(rename = "line")]
    Line { #[serde(rename = "relEnd")] rel_end: u64 },
    #[serde(rename = "circle")]
    Circle { #[serde(rename = "relEnd")] rel_end: u64 },
    #[serde(rename = "v")]
    V { #[serde(rename = "relEnd")] rel_end: u64 },
    #[serde(rename = "turn")]
    Turn { #[serde(rename = "relEnd")] rel_end: u64 },
    #[serde(rename = "pq")]
    Pq { #[serde(rename = "relEnd")] rel_end: u64 },
    #[serde(rename = "ppqq")]
    Ppqq { #[serde(rename = "relEnd")] rel_end: u64 },
    #[serde(rename = "s")]
    S,
    #[serde(rename = "wifi")]
    Wifi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideSymmetry {
    pub rotation_steps: u64,
    pub mirrored: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideShape {
    pub canonical: CanonicalSlideShape,
    pub symmetry: SlideSymmetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SlideBodyKind {
    Line,
    CircleRight,
    CircleLeft,
    CircleUp,
    V,
    Pp,
    Qq,
    P,
    Q,
    S,
    Z,
    Turn,
    Wifi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedSlideBody {
    pub raw_text: String,
    pub start_lane: OuterSlot,
    pub kind: SlideBodyKind,
    #[serde(default)]
    pub end_area: Option<SensorArea>,
    #[serde(default)]
    pub turn_area: Option<SensorArea>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideNoteSemantics {
    pub raw_text: String,
    pub start_slot: OuterSlot,
    pub end_area: SensorArea,
    pub shape: SlideShape,
    pub is_just_right: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RawNoteKind {
    Tap,
    Hold,
    Slide,
    Touch,
    TouchHold,
    Rest,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaidataMetadata {
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaidataChartBlock {
    pub level_index: u64,
    pub raw_body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawNoteToken {
    pub raw_text: String,
    pub kind: RawNoteKind,
    pub timing: TimePoint,
    pub bpm: Rational,
    pub h_speed: Rational,
    pub divisor: u64,
    #[serde(default)]
    pub slot: Option<OuterSlot>,
    #[serde(default)]
    pub sensor_pos: Option<SensorArea>,
    #[serde(default)]
    pub slide_body: Option<ParsedSlideBody>,
    #[serde(default)]
    pub length: Option<Duration>,
    #[serde(default)]
    pub star_wait: Option<Duration>,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    #[serde(default)]
    pub is_hanabi: bool,
    #[serde(default)]
    pub is_slide_no_head: bool,
    #[serde(default)]
    pub is_force_star: bool,
    #[serde(default)]
    pub is_fake_rotate: bool,
    #[serde(default)]
    pub is_slide_break: bool,
    #[serde(default)]
    pub source_group_id: Option<u64>,
    #[serde(default)]
    pub source_group_index: Option<u64>,
    #[serde(default)]
    pub source_group_size: Option<u64>,
    #[serde(default)]
    pub source_pos: Option<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceNote {
    pub token: RawNoteToken,
    #[serde(default)]
    pub source_pos: Option<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceEvent {
    pub timing: TimePoint,
    pub bpm: Rational,
    pub h_speed: Rational,
    pub divisor: u64,
    pub notes: Vec<SourceNote>,
    #[serde(default)]
    pub source_pos: Option<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceChart {
    pub events: Vec<SourceEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideAreaSpec {
    pub target_areas: Vec<SensorArea>,
    pub policy: AreaPolicy,
    #[serde(default)]
    pub is_last: bool,
    #[serde(default)]
    pub is_skippable: bool,
    pub arrow_progress_when_on: u64,
    pub arrow_progress_when_finished: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedSlideDebug {
    pub note_index: u64,
    pub raw_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedTap {
    pub timing: TimePoint,
    pub slot: OuterSlot,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    #[serde(default)]
    pub is_hanabi: bool,
    #[serde(default)]
    pub is_force_star: bool,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedHold {
    pub timing: TimePoint,
    pub slot: OuterSlot,
    pub length: Duration,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    #[serde(default)]
    pub is_hanabi: bool,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedTouch {
    pub timing: TimePoint,
    pub sensor_pos: SensorArea,
    #[serde(default)]
    pub is_break: bool,
    #[serde(default)]
    pub is_hanabi: bool,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedTouchHold {
    pub timing: TimePoint,
    pub sensor_pos: SensorArea,
    pub length: Duration,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    #[serde(default)]
    pub is_hanabi: bool,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedSlide {
    pub timing: TimePoint,
    pub slot: OuterSlot,
    pub length: Duration,
    pub start_timing: TimePoint,
    pub h_speed: Rational,
    pub slide_kind: RuntimeSlideKind,
    #[serde(default)]
    pub is_classic: bool,
    pub track_count: u64,
    #[serde(default)]
    pub judge_at: Option<TimePoint>,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    #[serde(default)]
    pub is_hanabi: bool,
    #[serde(default)]
    pub is_slide_no_head: bool,
    #[serde(default)]
    pub is_force_star: bool,
    #[serde(default)]
    pub is_fake_rotate: bool,
    #[serde(default)]
    pub is_slide_break: bool,
    #[serde(default)]
    pub is_conn_slide: bool,
    #[serde(default)]
    pub parent_note_index: Option<u64>,
    #[serde(default)]
    pub is_group_head: bool,
    #[serde(default)]
    pub is_group_end: bool,
    pub total_judge_queue_len: u64,
    pub judge_queues: Vec<Vec<SlideAreaSpec>>,
    #[serde(default)]
    pub source_group_id: Option<u64>,
    #[serde(default)]
    pub source_group_index: Option<u64>,
    #[serde(default)]
    pub source_group_size: Option<u64>,
    pub note_index: u64,
    pub simai_shape: SlideShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedChart {
    pub taps: Vec<NormalizedTap>,
    pub holds: Vec<NormalizedHold>,
    pub touches: Vec<NormalizedTouch>,
    pub touch_holds: Vec<NormalizedTouchHold>,
    pub slides: Vec<NormalizedSlide>,
    pub slide_debug: Vec<NormalizedSlideDebug>,
    pub slide_skipping: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendChartInspection {
    pub metadata: MaidataMetadata,
    pub chart: MaidataChartBlock,
    pub source: SourceChart,
    pub tokens: Vec<RawNoteToken>,
    pub slide_notes: Vec<SlideNoteSemantics>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendSemanticChart {
    pub normalized: NormalizedChart,
    pub lowered: ChartSpec,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendChartResult {
    pub semantic: FrontendSemanticChart,
    pub inspection: FrontendChartInspection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TapChartNote {
    pub timing: TimePoint,
    pub slot: OuterSlot,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    pub button_queue_index: u64,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldChartNote {
    pub timing: TimePoint,
    pub slot: OuterSlot,
    pub length: Duration,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    #[serde(default)]
    pub is_touch: bool,
    #[serde(default)]
    pub is_classic: Option<bool>,
    pub button_queue_index: u64,
    #[serde(default)]
    pub touch_hold_group_id: Option<u64>,
    #[serde(default)]
    pub touch_hold_group_size: Option<u64>,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchChartNote {
    pub timing: TimePoint,
    pub sensor_pos: SensorArea,
    #[serde(default)]
    pub is_break: bool,
    pub touch_queue_index: u64,
    #[serde(default)]
    pub touch_group_id: Option<u64>,
    #[serde(default)]
    pub touch_group_size: Option<u64>,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchHoldChartNote {
    pub timing: TimePoint,
    pub sensor_pos: SensorArea,
    pub length: Duration,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    pub touch_queue_index: u64,
    #[serde(default)]
    pub touch_group_id: Option<u64>,
    #[serde(default)]
    pub touch_group_size: Option<u64>,
    #[serde(default)]
    pub touch_hold_group_id: Option<u64>,
    #[serde(default)]
    pub touch_hold_group_size: Option<u64>,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideChartNote {
    pub timing: TimePoint,
    pub slot: OuterSlot,
    pub length: Duration,
    pub start_timing: TimePoint,
    pub slide_kind: RuntimeSlideKind,
    #[serde(default)]
    pub is_classic: bool,
    #[serde(default)]
    pub is_conn_slide: bool,
    #[serde(default)]
    pub parent_note_index: Option<u64>,
    #[serde(default)]
    pub is_group_head: bool,
    #[serde(default)]
    pub is_group_end: bool,
    #[serde(default)]
    pub parent_finished: bool,
    #[serde(default)]
    pub parent_pending_finish: bool,
    pub total_judge_queue_len: u64,
    pub track_count: u64,
    #[serde(default)]
    pub judge_at: Option<TimePoint>,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    pub note_index: u64,
    pub judge_queues: Vec<Vec<SlideAreaSpec>>,
    #[serde(default)]
    pub debug_simai: Option<(String, (String, bool))>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartSpec {
    pub taps: Vec<TapChartNote>,
    pub holds: Vec<HoldChartNote>,
    pub touches: Vec<TouchChartNote>,
    pub touch_holds: Vec<TouchHoldChartNote>,
    pub slides: Vec<SlideChartNote>,
    #[serde(default)]
    pub slide_skipping: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimedInputEvent {
    #[serde(rename = "buttonClick")]
    ButtonClick { tp: TimePoint, zone: ButtonZone },
    #[serde(rename = "buttonHold")]
    ButtonHold { tp: TimePoint, zone: ButtonZone, #[serde(rename = "isDown")] is_down: bool },
    #[serde(rename = "sensorClick")]
    SensorClick { tp: TimePoint, area: SensorArea },
    #[serde(rename = "sensorHold")]
    SensorHold { tp: TimePoint, area: SensorArea, #[serde(rename = "isDown")] is_down: bool },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimedInputBatch {
    pub current_time: TimePoint,
    pub events: Vec<TimedInputEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameInput {
    pub button_clicked: Vec<bool>,
    pub button_held: Vec<bool>,
    pub sensor_clicked: Vec<bool>,
    pub sensor_held: Vec<bool>,
    pub button_click_count: Vec<u64>,
    pub sensor_click_count: Vec<u64>,
    pub delta: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneQueue<T> {
    pub notes: Vec<T>,
    pub current_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonNoteParams {
    pub judge_timing: TimePoint,
    pub judge_offset: Duration,
    #[serde(default)]
    pub is_break: bool,
    #[serde(rename = "isEX", default)]
    pub is_ex: bool,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoldStart {
    #[serde(rename = "button")]
    Button { zone: ButtonZone },
    #[serde(rename = "sensor")]
    Sensor { area: SensorArea },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TapState {
    #[serde(rename = "Waiting")]
    Waiting,
    #[serde(rename = "Judgeable")]
    Judgeable,
    #[serde(rename = "Judged")]
    Judged { grade: JudgeGrade },
    #[serde(rename = "Ended")]
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TapNote {
    pub params: CommonNoteParams,
    pub lane: OuterSlot,
    pub state: TapState,
    pub button_queue_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoldSubState {
    #[serde(rename = "HeadWaiting")]
    HeadWaiting,
    #[serde(rename = "HeadJudgeable")]
    HeadJudgeable,
    #[serde(rename = "HeadJudged")]
    HeadJudged { grade: JudgeGrade },
    #[serde(rename = "BodyHeld")]
    BodyHeld,
    #[serde(rename = "BodyReleased")]
    BodyReleased,
    #[serde(rename = "Ended")]
    Ended { grade: JudgeGrade },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldNote {
    pub params: CommonNoteParams,
    pub start: HoldStart,
    pub state: HoldSubState,
    pub length: Duration,
    pub button_queue_index: u64,
    pub head_diff: Duration,
    pub head_grade: JudgeGrade,
    pub player_release_time: Duration,
    #[serde(default)]
    pub is_classic: bool,
    #[serde(default)]
    pub is_touch_hold: bool,
    pub touch_queue_index: u64,
    #[serde(default)]
    pub touch_group_id: Option<u64>,
    pub touch_group_size: u64,
    #[serde(default)]
    pub touch_hold_group_id: Option<u64>,
    pub touch_hold_group_size: u64,
    #[serde(default)]
    pub touch_hold_group_triggered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TouchState {
    #[serde(rename = "Waiting")]
    Waiting,
    #[serde(rename = "Judgeable")]
    Judgeable,
    #[serde(rename = "Judged")]
    Judged { grade: JudgeGrade },
    #[serde(rename = "Ended")]
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchNote {
    pub params: CommonNoteParams,
    pub state: TouchState,
    pub sensor_pos: SensorArea,
    pub touch_queue_index: u64,
    #[serde(default)]
    pub touch_group_id: Option<u64>,
    pub touch_group_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideState {
    #[serde(rename = "Waiting")]
    Waiting,
    #[serde(rename = "Active")]
    Active { #[serde(rename = "waitTime")] wait_time: Duration },
    #[serde(rename = "Judged")]
    Judged {
        grade: JudgeGrade,
        #[serde(rename = "waitTime")] wait_time: Duration,
        #[serde(rename = "judgeDiff")] judge_diff: Duration,
    },
    #[serde(rename = "Ended")]
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideArea {
    pub target_areas: Vec<SensorArea>,
    pub policy: AreaPolicy,
    #[serde(default)]
    pub is_last: bool,
    #[serde(default)]
    pub is_skippable: bool,
    pub arrow_progress_when_on: u64,
    pub arrow_progress_when_finished: u64,
    #[serde(default)]
    pub was_on: bool,
    #[serde(default)]
    pub was_off: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideNote {
    pub params: CommonNoteParams,
    pub lane: OuterSlot,
    pub state: SlideState,
    pub length: Duration,
    pub timing: TimePoint,
    pub start_timing: TimePoint,
    pub slide_kind: RuntimeSlideKind,
    #[serde(default)]
    pub is_classic: bool,
    #[serde(default)]
    pub is_conn_slide: bool,
    #[serde(default)]
    pub parent_note_index: Option<u64>,
    #[serde(default)]
    pub is_group_part_head: bool,
    #[serde(default)]
    pub is_group_part_end: bool,
    #[serde(default)]
    pub parent_finished: bool,
    #[serde(default)]
    pub parent_pending_finish: bool,
    pub initial_queue_remaining: u64,
    pub total_judge_queue_len: u64,
    pub track_count: u64,
    #[serde(default)]
    pub is_checkable: bool,
    pub judge_queues: Vec<Vec<SlideArea>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupState {
    pub group_id: u64,
    pub count: u64,
    pub size: u64,
    pub grade: JudgeGrade,
    pub diff: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteTypeJudgeCounts {
    pub tap_count: JudgeCounts,
    pub hold_count: JudgeCounts,
    pub slide_count: JudgeCounts,
    pub touch_count: JudgeCounts,
    pub break_count: JudgeCounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreState {
    pub combo: u64,
    pub p_combo: u64,
    pub c_p_combo: u64,
    pub total_base: u64,
    pub total_extra: u64,
    pub earned_base: u64,
    pub earned_extra: u64,
    pub lost_base: u64,
    pub lost_extra: u64,
    pub dx_score: i64,
    pub max_dx_score: u64,
    pub fast_count: u64,
    pub late_count: u64,
    pub counts: NoteTypeJudgeCounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub current_time: TimePoint,
    pub prev_button: Vec<bool>,
    pub prev_sensor: Vec<bool>,
    pub button_queue_frontiers: Vec<u64>,
    pub touch_queue_frontiers: Vec<u64>,
    pub tap_queues: Vec<ZoneQueue<TapNote>>,
    pub hold_queues: Vec<ZoneQueue<HoldNote>>,
    pub touch_hold_queues: Vec<ZoneQueue<HoldNote>>,
    pub touch_queues: Vec<ZoneQueue<TouchNote>>,
    pub slides: Vec<SlideNote>,
    pub active_holds: Vec<(ButtonZone, HoldNote)>,
    pub active_touch_holds: Vec<(SensorArea, HoldNote)>,
    pub touch_group_states: Vec<GroupState>,
    pub touch_hold_group_states: Vec<GroupState>,
    pub current_batch: TimedInputBatch,
    pub score: ScoreState,
    pub judge_style: JudgeStyle,
    pub touch_panel_offset: Duration,
    pub use_button_ring_for_touch: bool,
    pub subdivide_slide_judge_grade: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JudgeEvent {
    pub kind: JudgeEventKind,
    pub grade: JudgeGrade,
    pub diff: Duration,
    pub position: RuntimePos,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum AudioCommand {
    #[serde(rename = "PlayJudgeSfx")]
    PlayJudgeSfx {
        kind: JudgeEventKind,
        grade: JudgeGrade,
        at_time: TimePoint,
        note_index: u64,
    },
    #[serde(rename = "PlaySlideCue")]
    PlaySlideCue {
        note_index: u64,
        track_index: u64,
        at_time: TimePoint,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum RenderCommand {
    #[serde(rename = "ShowJudgeResult")]
    ShowJudgeResult {
        kind: JudgeEventKind,
        grade: JudgeGrade,
        diff: Duration,
        note_index: u64,
    },
    #[serde(rename = "UpdateSlideProgress")]
    UpdateSlideProgress {
        note_index: u64,
        remaining: u64,
    },
    #[serde(rename = "UpdateSlideTrackProgress")]
    UpdateSlideTrackProgress {
        note_index: u64,
        track_index: u64,
        remaining: u64,
    },
    #[serde(rename = "HideAllSlideBars")]
    HideAllSlideBars {
        note_index: u64,
    },
    #[serde(rename = "HideSlideBars")]
    HideSlideBars {
        note_index: u64,
        end_index: u64,
    },
    #[serde(rename = "HideSlideTrackBars")]
    HideSlideTrackBars {
        note_index: u64,
        track_index: u64,
        end_index: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStepResult {
    pub state: GameState,
    pub events: Vec<JudgeEvent>,
    pub audio_commands: Vec<AudioCommand>,
    pub render_commands: Vec<RenderCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStepLightResult {
    pub events: Vec<JudgeEvent>,
    pub audio_commands: Vec<AudioCommand>,
    pub render_commands: Vec<RenderCommand>,
    pub score: ScoreState,
    pub current_time: TimePoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedChartSummary {
    pub tap_count: u64,
    pub hold_count: u64,
    pub touch_count: u64,
    pub touch_hold_count: u64,
    pub slide_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionState {
    pub handle: u64,
    pub state: String,
    #[serde(default)]
    pub summary: Option<LoadedChartSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleOnly {
    pub handle: u64,
}
