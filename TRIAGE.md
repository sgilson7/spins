# Triage

Stage 7. One row per request after the person has used the build: a bug, a change, or a new idea, in the person's own words. Rows are numbered T1, T2 and so on, and each commit that answers one names it ("Fix the timer reset (T4)"). `C4 Verify against hand-computed cases`.

Compare at least one result with a case worked out by hand. **Severity** is *blocks use*, *wrong*, or *wanted*. **Cost** is the agent's estimate. **Disposition** is *done*, *open*, *declined* (with the reason), or *try* (built on a branch and published as a TRY commit that says how to undo it).

| # | Request, in the person's words | Expected | What happened | Severity | Cost | Disposition |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | "SP-Q13: alternative" | Both Red Legends on the left, for comparing at a glance | Built: both sides seat Red on the left in every view | wanted | small | done |
| T2 | "SP-Q14: choose my own, the following list red ×1, red ×1 gilded, red ×2, red ×2 gilded, blue ×1, blue ×1 gilded, blue ×2, blue ×2 gilded, Barricade ×1, Barricade ×2" | Ten panels per wheel, those ten | Built as `rules::PINE`; the balance run on it is in R2.1 | wanted | small | done |
| T3 | "SP-Q17: alternative" | One save kept in the browser | Built: wins, losses and oak wheels kept in `localStorage` through `web/shared/save.js`, with a Forget my record button | wanted | medium | done |
| T4 | "SP-Q18: alternative" | Better wheel grades carry more doubles, and champions use them | Built: oak wheels turn plain singles into doubles; Halvard plays on oak; beating him wins them | wanted | medium | done |
| T5 | "SP-Q19: alternative" | Simple synthesised sounds in the MVP | Built: ten voices in `data/sounds.json`, played on spins, locks and each event, with a Sound switch | wanted | small | done |
| T6 | "you should also be able to edit your lands deck" | Unclear: Spins has no deck or lands | Not built; asked the person what it refers to (R2.7) | wanted | unknown | open |
