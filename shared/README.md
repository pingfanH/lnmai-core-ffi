# Shared Rust FFI Sources

This directory contains the single-source Rust implementation files shared by:

- the root crate in `src/`
- the embedded example bindings in `lnmai-core/bindings/rust/`

Files:

- `rust_ffi_api.rs` — typed helpers around the string-based parse/build/step APIs
- `rust_ffi_raw.rs` — raw Lean/FFI symbol declarations and runtime init
- `rust_ffi_session.rs` — typestate session wrapper and JSON envelope helpers
- `rust_ffi_types.rs` — typed Rust mirrors of Lean JSON payloads

Guideline:

- when refining Rust-side FFI behavior or payload schemas, update these shared files first
- keep `src/` and `lnmai-core/bindings/rust/` as thin include-based entrypoints

Current slide head/body schema notes:

- lowered charts now expose explicit `slideHeads` plus slide-body `slides`
- ordinary slides lower to head + body; no-head singleton slides and connected child parts lower to body only
- lowered head/body pairs now carry explicit `logicalSlideId`
- lowered slide heads and slide bodies now use distinct `noteIndex` values while sharing `logicalSlideId`
- lowered slide bodies use `headTiming` as the body-side head anchor field in the current schema
- Rust mirrors require `headTiming` for slide-body payloads
- normalized slide mirrors expose explicit `hasHeadNote` and `hasBody`; `isSlideNoHead` remains compatibility metadata rather than the long-term semantic authority
