# Handoff

Stage 9, written by the agent. `C5 Consolidate in your own words`. A fresh session starts from this file, not from memory.

## What this is

Spins is a two-player tavern wheels game after Sea of Stars' Wheels, played in the browser against a computer opponent. One Rust engine runs three views: 2D in Lowballer's look, 3D in Protection of the Spheres' look, and pixel. It lives in the public repository `sgilson7/spins` and is published at https://sgilson7.github.io/spins/.

## Where it is live

- https://sgilson7.github.io/spins/ (chooser), `/spins/2d/`, `/spins/3d/`, `/spins/pixel/`, from the `gh-pages` branch (`./scripts/publish.sh`). The browser check against the live address passed in Chromium, Firefox and WebKit, one engine file across the views.
- Listed at https://sgilson7.github.io/projects/spins/ and on `/games/` under 2D, from the website repository (`../../website`, commit f8c8e76, not pushed: that repository's source is never pushed). Deployed with its `make deploy` from a worktree, and its `make gate-live` passed (R2.13).

## Rules that hold it up

| Rule | Guarded by |
| --- | --- |
| Every rule in `crates/core`; the shim decides nothing; the pages hold no number but 0 and 1 | `boundary.rs` |
| `crates/view3d` is held to the core's scans and allowlist | `view3d_is_held_to_the_core_rules` |
| The opponent never sees your faces; the page never gets theirs before the reveal | `every_skill_decides_the_same_whatever_your_faces`; `the_opponents_faces_never_reach_the_page_before_the_reveal` |
| The brief's numbers (health 10, Barricade 5, bars 3/5/4, 3oak+, the three attacks) | `rules.rs` and `table.rs` tests, each in `tests/breaks.json` |
| Only `web/shared/save.js` touches storage | `no_storage.rs` |
| Every view loads one wasm file and plays a whole match | `tests/browser/interactions.py` |

## Shape of the code

| Path | Holds |
| --- | --- |
| `crates/core/src/rules.rs` | symbols, panels, the two wheel grades, 3oak+, proficiencies and attacks |
| `crates/core/src/table.rs` | a match: spins, locks, the end of a turn as events |
| `crates/core/src/ai.rs` | the three opponents' skills |
| `crates/core/src/api.rs` | the `Session`: screens, the table view, narration |
| `crates/core/src/save.rs` | the save text |
| `crates/view3d/src/scene.rs` | the 3D table's instances and animation |
| `web/shared/` | the code every view shares: boot, menus, event playback, board (2D), sound, save |
| `web/2d/`, `web/pixel/`, `web/3d/` | each view's page, pictures and stylesheet |
| `data/` | strings, timings, sounds, shaders, chooser thumbnails |

## Commands

See README.md. `./scripts/check.sh` prints `BUILDER CHECK PASSED`; `python3 scripts/break_tests.py` prints `18 of 18 breaks caught`.

## What will bite within the hour

- rustfmt rewraps lines, so a scripted edit or a break's `find` text must be checked after `cargo fmt`.
- The page scan reads every file under `web/` as text; binary assets go in `data/`.
- The 2D board builds its skeleton once per match. Rebuilding it cancels the wheels' CSS spins.
- The browser check runs 3D in software WebGL; it proves the view works, not that it is fast.

## The journal

`../spins-journal/index.html`, published privately at https://claude.ai/artifact/Hsf1RJnsLwYdLUxGX2CQbc. Tabs 1 to 7.

## Open for the person

- R1.2: the cut-off sentence in the description (journal tab 2, SP-Q7).
- R1.3: XP and evolution from Sea of Stars (SP-Q5, SP-Q6).
- R1.4: rename the placeholder opponents and taverns (SP-Q15).
- R2.1: matches are short (3 to 5 turns) with many draws on the ten-panel wheel; options in the notebook.
- R2.6: the wasm is 404 KB raw (113 KB gzipped), over SP-10's 250 KB.
- R2.7: "edit your lands deck" (T6): what does it refer to?
