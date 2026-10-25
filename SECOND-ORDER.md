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

## Strings written by the agent

Each reader-visible string the agent wrote, with its key, for the person to read.

| Key | String |
| --- | --- |
