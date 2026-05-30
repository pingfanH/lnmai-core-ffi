use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum JudgeEventKind {
    Tap,
    Hold,
    Slide,
    Touch,
    Break,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SensorArea {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ButtonZone {
    K1, K2, K3, K4, K5, K6, K7, K8,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RuntimePos {
    pub button: Option<ButtonZone>,
    pub sensor: Option<SensorArea>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct JudgeEvent {
    pub kind: JudgeEventKind,
    pub grade: JudgeGrade,
    pub diff: i64,
    pub position: RuntimePos,
    pub note_index: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "tag")]
pub enum RenderCommand {
    #[serde(rename = "ShowJudgeResult")]
    ShowJudgeResult {
        kind: JudgeEventKind,
        grade: JudgeGrade,
        diff: i64,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "tag")]
pub enum AudioCommand {
    #[serde(rename = "PlayJudgeSfx")]
    PlayJudgeSfx {
        kind: JudgeEventKind,
        grade: JudgeGrade,
        at_time: i64,
        note_index: u64,
    },
    #[serde(rename = "PlaySlideCue")]
    PlaySlideCue {
        note_index: u64,
        track_index: u64,
        at_time: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RuntimeStepLightResult {
    pub events: Vec<JudgeEvent>,
    #[serde(rename = "audioCommands")]
    pub audio_commands: Vec<AudioCommand>,
    #[serde(rename = "renderCommands")]
    pub render_commands: Vec<RenderCommand>,
    pub score: ScoreState,
    #[serde(rename = "currentTime")]
    pub current_time: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ScoreState {
    pub combo: u64,
    #[serde(rename = "pCombo")]
    pub p_combo: u64,
    #[serde(rename = "cPCombo")]
    pub c_p_combo: u64,
    #[serde(rename = "totalBase")]
    pub total_base: u64,
    #[serde(rename = "totalExtra")]
    pub total_extra: u64,
    #[serde(rename = "earnedBase")]
    pub earned_base: u64,
    #[serde(rename = "earnedExtra")]
    pub earned_extra: u64,
    #[serde(rename = "lostBase")]
    pub lost_base: u64,
    #[serde(rename = "lostExtra")]
    pub lost_extra: u64,
    #[serde(rename = "dxScore")]
    pub dx_score: i64,
    #[serde(rename = "maxDxScore")]
    pub max_dx_score: u64,
    #[serde(rename = "fastCount")]
    pub fast_count: u64,
    #[serde(rename = "lateCount")]
    pub late_count: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FfiResult {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}
