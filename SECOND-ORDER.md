# Second-order notebook

Stage 6, kept by the agent as the work happens, not afterwards. `B3 Build in self-questioning`.

The code and tests are the work; this file is the record about the work. It is where a decision the agent made alone, an assumption that turned out wrong, or something only the person can do becomes visible to the person. With the plan approved in advance, it is the main place those decisions surface before the person uses the tool. [`docs/procedure.md`](https://github.com/sgilson7/builder-setting/blob/main/docs/procedure.md#the-second-order-notebook) says why.

One row per assumption, surprise or open item. **Kind** is *divergence* (what was assumed, what turned out to be true, what changed), *finding*, or *worklist* (the person's to do). Number rows by round (R1.1, R1.2, then R2.1 after the first triage) so a later round does not renumber an earlier one. **Rule** is the rule the row produced for later sessions, and where it now lives (`CLAUDE.md`, a test, `tests/breaks.json`), or blank. **Status** is *done*, *open*, or *person*; every *person* row is named in `HANDOFF.md`. `scripts/notebook_check.py` checks the form of each row and that the notebook is not empty once `PLAN.md` holds a decision, not what the rows say.

| # | Kind | Assumed or found | What is true | What changed | Rule | Status |
| --- | --- | --- | --- | --- | --- | --- |
| R1.1 | finding | The description in BRIEF.md was plain prose with no quote marks. | `journal_check.py` reads a description only from quoted lines. | Quoted the description and today's request word for word; drafted the empty sections from them, each marked *Drafted*. | | done |
| R1.2 | finding | The description's sentence "Each Legend's a energy bar will maintain the" is cut off. | The sentence before it already says segments persist between turns. | Read as persistence; overflow left to SP-Q7. | | person |
| R1.3 | divergence | The brief names three symbol kinds only. | Sea of Stars adds XP through blue-backed tiles. | Proposed XP as a gilt mark on red and blue panels, not a fourth kind (SP-Q5, SP-Q6). | | person |
| R1.4 | worklist | Opponent and tavern names are needed for the screens. | I made up Old Wenna, Brother Tobble, Halvard, The Wagon Wheel, The Leaky Keg and The Crown as placeholders. | Listed in tab 3 (SP-Q15) for the person to rename. | | person |
| R1.5 | finding | Wheel strips were not given. | Chose 8 panels per wheel, the same eight on each wheel in different orders (SP-Q14). | Balance run of 1,000 matches before any tuning. | | open |
| R1.6 | finding | "Protection of the spheres" is a Sam game. | It is `sgilson7/pots` (`OGDota/` on this Mac); its renderer meets D3 by building Q16 instances in Rust. | The 3D view copies POTS's renderer files into `crates/view3d` and `web/3d/`. | | done |
| R2.1 | finding | Matches would last about eight turns, as in Sea of Stars. | On the person's ten-panel wheel, 1,000 computer matches last 3.3 to 4.9 turns on average, and 21% to 41% end in a draw (`cargo test --release --test balance -- --nocapture`). With XP turned off it is still 3.7 turns and 36% draws, so the wheel and the simultaneous end of turn set the pace, not XP. | Nothing tuned: the strip is the person's (T2). Options for the person: fewer energy panels or a blank panel; more health; a tiebreak for SP-Q9. | `matches_last_a_handful_of_turns_and_skill_tells` holds the report's bounds | person |
| R2.2 | finding | The 2D wheels could be rebuilt on every draw. | Rebuilding re-parented the strips and cancelled their CSS spin mid-flight. | `board.js` builds the table once per match and updates it in place. | | done |
| R2.3 | divergence | D17: no browser storage. | The person chose to keep progress (T3). | One file, `web/shared/save.js`, may name `localStorage`; the test allows nothing else. | `no_storage.rs` | done |
| R2.4 | finding | The 3D view needs numbers in its page. | POTS shows the way: a Rust crate builds Q16 instances; the page only uploads them. | `crates/view3d`, scanned like the core. | `view3d_is_held_to_the_core_rules` | done |
| R2.5 | divergence | One look per view, drawn by hand. | The pixel view's sprites are text grids turned into SVG by `tools/pixel/sprites.py`, so they stay editable as source. | | | done |
| R2.6 | divergence | SP-10: wasm under 250 KB. | It is 404 KB raw and 113 KB gzipped, as GitHub Pages serves it. `opt-level = "z"` made it 441 KB. `wasm-opt` is not installed here. | Budget missed as written; each view loads 131 to 143 KB gzipped in all, under SP-10's page budgets. | MEASUREMENTS.md | person |
| R2.7 | worklist | Every request concerns Spins. | During the build the person wrote "you should also be able to edit your lands deck". Spins has no deck or lands; it reads like the MTG Hand of Fate project. | Not built; asked (T6). | | person |
| R2.8 | finding | `locks_wait_for_the_first_spin…` covered "the first spin turns all five wheels". | The break was not caught: `spin()` cleared locks that the end of the turn had already cleared. | Removed the duplicate; `a_new_turn_starts_with_every_wheel_unlocked` holds the rule, and its break is caught. | `tests/breaks.json` | done |
| R2.9 | finding | `energy_past_a_full_bar_is_dropped` covered overflow. | The break was not caught: a full bar acts and empties either way. | The test now reads the energy event's value (3 of 3), which carrying over would change. | `tests/breaks.json` | done |
| R2.10 | finding | The browser check could compare a wheel's CSS position before and after a locked spin. | It raced the animation frame that sets the position. | The board stamps the core's face on each strip (`data-face`), and the check reads that. | `tests/browser/interactions.py` | done |
| R2.11 | finding | The chooser's thumbnails could sit in `web/img/`. | The page scan reads every file under `web/` as text and failed on binary images. | Moved to `data/img/`; the scan is unchanged. | | done |
| R2.12 | finding | `every_screen_and_event_finds_its_strings` caught any missing string. | It caught a key only when it filled a whole string; "It hits [you.foe] for 0." reached the 3D log. The person would have seen it. | The check now finds `[a.key]` inside a sentence, and a break row removes a key to prove it. Also: a hit on health already at none now reads "There is no health left to hit.", and "acted 1 times" reads "acted once". | `tests/breaks.json` | done |
| R2.13 | finding | The site listing could be deployed from the website repository's tree. | Another session's uncommitted Packet Alley text was already live, and the tree's `dist/` would have carried it under this deploy, while a clean archive would have reverted it and dropped every sitemap `lastmod` (no git history). | Built and deployed from a git worktree of HEAD with that one file copied in as it is live, so the deploy changed only Spins' lines; `make gate-live` passed. | | done |

## Strings written by the agent

Each reader-visible string the agent wrote, with its key, for the person to read.

| Key | String |
| --- | --- |
| `data/strings.json` (all keys) | Every string in the file was written by the agent from the brief's words; the opponent and tavern names are placeholders (R1.4). |
