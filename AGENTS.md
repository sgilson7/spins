# Instructions for a coding agent in a Builder project

`CLAUDE.md` holds the same rules for agents that read that file instead.

## Where code goes

1. Read `BRIEF.md`, `BUILDER-SPEC.md` and `GLOSSARY.md` before changing code.
2. The core (`crates/core`) owns every application decision and performs no input or output. Scripts and tools that need a rule read the core's output; they do not recompute it.
3. The shim (`crates/shim`) translates values and decides no application behaviour.
4. The page (`web/`) draws what the core returns and does not reproduce a core rule. The core describes the screen (`ui`) and takes settings, screens and flags through one function (`act`). Presentation numbers go in `data/*.json`; shaders go in `data/shaders/` and take rule values as uniforms.

## How to work

5. Run `cargo test` after each meaningful core change.
6. For each new behaviour test, add a row to `tests/breaks.json` and run `python3 scripts/break_tests.py`: the test must fail with its behaviour broken. Work expected values out by hand; never compare a value with the function under test. When a test needs a workaround to pass, the workaround is the finding: record it.
7. Keep `SECOND-ORDER.md` while you work: one row per assumption, surprise, divergence from the plan, or item the person must do, written when you notice it. It is how the person sees what you decided alone. A row that produced a rule names where the rule now lives; a row with status *person* is named in `HANDOFF.md`.
8. A scripted edit that replaces text asserts that it matched. A count in a document comes from `./scripts/count-tests.sh`.
9. Do not deploy unless `BRIEF.md`'s Autonomy paragraph says you may. Deploying is otherwise the person's decision (stage 8).
10. Before handoff, run `./scripts/check.sh` and record the result in `HANDOFF.md`.

## Words

11. Use each term in the sense `GLOSSARY.md` gives it, and never a word from its `Not:` line. A new term gets an entry before it appears in a string.
12. Reader-visible text follows `TONE.md`. A string that looks wrong is reported in `SECOND-ORDER.md`, not repaired. A string you write yourself is listed at the foot of `SECOND-ORDER.md` for the person to read. A list you generate that a reader will see (names, labels, word lists) is a worklist row for the person.

## The design journal

13. Keep the design journal unless `BRIEF.md`'s Journal section says `none`. Start it with `python3 tools/journal/new_journal.py`; it lives beside the project (by default `../<this folder>-journal/`) as its own repository and is never pushed. `tools/journal/ENTRIES.md` says what each kind of entry holds.
14. Before writing `PLAN.md`, write the journal's opening entry: read Builder and the brief back to the person, with numbered rules and what holds each, the question prefix, where you stop, and your questions. When the person answers, each rule that applies to every later session goes under "Rules agreed for this project" below, and each rule the core can enforce becomes a test.
15. Every decision put to the person, visible change, divergence from the plan and deploy gets a journal entry when it happens, citing its rows (T4, R2.3, XX-Q5). Never remove an entry. `scripts/journal_check.py` checks the form; commit in the journal's repository after each change.

## Rules agreed for this project

Agreed 2026-10-24 (journal tab 1, SP-1 to SP-12; the person answered SP-Q13, Q14, Q17, Q18 and Q19, and accepted the other defaults):

- SP-1. All three views load one `spins.wasm`; no view has an engine of its own. Held by the browser check (one wasm file across views).
- SP-2. No view holds a rule. Views copy numbers from the core's table and events and animate them. Held by the page scans.
- SP-3. The computer opponent reads only its own side and `ai::Public` of yours, which holds no faces. Held by `every_skill_decides_the_same_whatever_your_faces`.
- SP-4. The opponent's faces never reach the page before the reveal. Held by `the_opponents_faces_never_reach_the_page_before_the_reveal`.
- SP-8. Only `web/shared/save.js` may use browser storage (T3). Held by `no_storage.rs`.
- SP-9. No subagents, in this session or a later one, unless the person says otherwise.
- SP-11. Deploy only to `sgilson7/spins` (GitHub Pages) and the listing on sgilson7.github.io, after a passing `./scripts/check.sh`, then run the browser check against the live address.
- SP-12. Every visible change, decision for the person, divergence and deploy gets a journal entry in `../spins-journal/`, citing its rows.

## Stopping

Write the journal's opening entry, then `PLAN.md` from the brief, and stop until the person has read it, unless `BRIEF.md`'s Autonomy paragraph approves the plan in advance. When a rule in `BUILDER-SPEC.md` seems to block the work, stop and say so in a worklist row; do not loosen a check to make it pass. When a request is refused on safety grounds, leave the feature out and say so; do not reword the request to get past the refusal.
