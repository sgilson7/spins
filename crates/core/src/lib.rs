//! tool_core: every rule of Spins.
//!
//! Builder requirement: this crate performs no input or output. It takes
//! values and returns values, so `cargo test` reaches every rule without a
//! browser. `tests/boundary.rs` enforces that, along with the dependency
//! allowlist and the reference ban on floats, hash maps and clocks.
//!
//! `api.rs` is the one door the shim calls through. All three views (2D,
//! 3D, pixel) load this same module (SP-1).

pub mod ai;
pub mod api;
pub mod glossary;
pub mod rng;
pub mod rules;
pub mod save;
pub mod table;
