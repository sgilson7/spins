# Measurements

Stage 4, recorded by the agent before building and again at handoff. `B2 Require step-by-step justification`.

The state measured rather than recalled, so later changes can be compared with it. A probe prints numbers and asserts loose bounds at most; record its output here with the command that produced it.

| What | Command | Result | When |
| --- | --- | --- | --- |
| Toolchain | `rustc --version; wasm-bindgen --version` | rustc 1.99.0, wasm-bindgen 0.2.127 | 2026-10-24 |
| Template baseline, before any edit | `./scripts/count-tests.sh`; `./scripts/check.sh` | 38 tests; all 9 steps passed in 33.5 s | 2026-10-24 |
| Tests at the MVP | `./scripts/count-tests.sh` | see HANDOFF.md | 2026-10-24 |
| WebAssembly size | printed by `./scripts/build.sh`; `gzip -9` | 404 KB raw, 113 KB gzipped (SP-10 asked under 250 KB raw: missed, R2.6) | 2026-10-24 |
| First load per view, every file the page fetches | a sum over `dist/` | 2D 468 KB raw / 131 KB gzipped; pixel 510 / 143; 3D 470 / 133; chooser 77 / 72 | 2026-10-24 |
| 3D: the most the table draws | counted once with a print in `a_full_table_stays_within_the_instance_budget` (the scan forbids leaving it in) | 479 instances in 21 instanced draws | 2026-10-24 |
| Balance, 1,000 matches each | `cargo test --release --test balance -- --nocapture` | 3.3 to 4.9 turns on average; 21% to 41% draws (R2.1) | 2026-10-24 |
| Breaks caught | `python3 scripts/break_tests.py` | 18 of 18 | 2026-10-24 |

## Not measured

Devices, browsers and conditions no measurement here covers.

- The 3D frame time on a real GPU: the browser check runs WebGL in software (SwiftShader), which says nothing about speed. The frame loop stops when nothing moves.
- Phones: the layouts were looked at in a narrow window, not on a device.
