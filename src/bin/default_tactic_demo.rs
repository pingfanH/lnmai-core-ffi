use lnmai_core_ffi::api;
use lnmai_core_ffi::session::initialize_runtime;

fn main() {
    unsafe {
        initialize_runtime().expect("failed to initialize Lean runtime");
    }

    let chart_text = include_str!("../../assets/24_Sun Dance/maidata.txt");
    let lowered = api::parse_lowered_chart(chart_text, 6)
        .expect("failed to parse lowered chart from assets/24_Sun Dance/maidata.txt");
    let tactic = api::default_tactic_from_chart(&lowered)
        .expect("failed to build default tactic from lowered chart");

    dbg!(&tactic);
}
