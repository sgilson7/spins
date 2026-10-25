# Plan

Stage 2, written by the agent from `BRIEF.md`. `B2 Require step-by-step justification`.

The journal's tabs 2 and 3 (`../spins-journal/`) show this plan with pictures; the questions are SP-Q1 to SP-Q20 there. The person reads tabs 1–3 before step 1; after that the plan counts as approved (BRIEF.md, Autonomy).

| # | Decision | Reason | Test that shows it holds |
| --- | --- | --- | --- |
| 1 | Rules in `crates/core`: `wheel.rs` (strips, spins from a seed), `count.rs` (3oak+), `legend.rs` (bars, attacks, XP), `table.rs` (turn machine, locks, resolution events). | R1: every rule in the core. | Hand-worked tests for B3–B14 of tab 2, each with a `tests/breaks.json` row. |
| 2 | `ai.rs`: three skills reading only public state and their own wheels. | SP-3: the opponent cannot see the user's faces. | Changing the user's faces leaves the opponent's locks unchanged; 1,000-match balance report. |
| 3 | `api.rs` `ui()` describes every screen and the table, with covered faces left out, and resolution as an event list. | SP-4; R4. | A test that the opponent's faces are absent before the reveal; copy and glossary tests. |
| 4 | `web/2d/`: a DOM renderer with CSS drums in Lowballer's style. | No canvas: cheapest to load and run. | The browser check plays a full match in three engines. |
| 5 | `web/pixel/`: the same DOM shape with pixel sprites, a vendored pixel font and stepped spins. | A second 2D style on the same page code. | Browser check. |
| 6 | `crates/view3d` + `web/3d/`: POTS's WebGL 2 renderer, instances built in Rust. | D3 holds in 3D too. | Browser check; frame time in `MEASUREMENTS.md`. |
| 7 | One `spins.wasm` for all three pages. | SP-1: byte-identical engines. | A test that every page imports the one glue module and `dist/` holds one `.wasm`. |
| 8 | Publish `sgilson7/spins` to GitHub Pages and list it on sgilson7.github.io. | Granted in BRIEF.md. | Browser check against the live address. |

## Open questions for the person

SP-Q1 to SP-Q20, in the journal (tab 1: Q1–Q4; tab 3: Q5–Q20). Each has a default the build proceeds on.

## Status

- [ ] Read by the person on: 
