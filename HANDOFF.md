## Agent evaluation round 2 (stopped after the Rust calibration, 2026-10-10)

See `eval/round2/RESULTS.md`. My pre-registered stop rule fired: 0 of the 11 Rust calibration runs on the 11
LeetCode Hard tasks shipped a defect (223/223 hidden tests), so I made no Touchmark run and the defect question is
still open. The reason is the same as round 1, one level up: every agent wrote a brute force from the specification
and checked its solution on 3,000 to 400,000 random inputs before answering DONE. Rust T_work went from 3,765 to
10,308. After the stop I planted bugs in the golds (the hidden tests fail 11 of 12; the twelfth is very likely
equivalent) and ran every solution on 4,600 more random inputs (all pass); both are labelled exploratory.
`eval/round2/private/report.py` regenerates the numbers from `results.json`.

The working copy of the private files is still at `~/touchmark-eval-r2` and the agents' directories under
`/work` (a link to `~/touchmark-work`); everything that is scored is also in `eval/round2/`.

Next: round 3, pre-registered before any run. My proposal is a no-execution condition: the same 11 tasks, tests and
scorer, but the agent may only compile (`rustc`, `tmk check`, `tmk build`), never run a binary or script, and the
transcript audit checks it. Rust calibration first, with the same stop rule. Before pre-registering, fold
`eval/round2/posthoc/audit.py` into `usage.py`: the round 2 audit compares paths by unnormalized prefix, so `..`
and relative climbs slip past it (no round 2 run did either). That is where "the proof replaces the
tests" should pay off. The fallbacks, if Rust still sits at ceiling: a weaker model arm, or tasks without a cheap
oracle. The token rule (T_work over 2x Rust) is judged at round 3 and is unmeasured on hard tasks; the one-task
pilot gave 1.9x.

Loose ends: `/usr/local/bin/tmk` is the older build that a login shell finds first (replace it with
`sudo install -m 755 ~/.local/bin/tmk /usr/local/bin/tmk`); the `round2-wip` worktree at
`../touchmark-round2` is clean and merged, so it can go.

## Agent evaluation round 1 (complete, 2026-10-01)

See `eval/RESULTS.md`. All 72 pre-registered runs scored: 0 of 36 Touchmark runs and 0 of 36 Rust runs shipped an
escaped defect (Fisher p = 1.0; ceiling effect, the defect question is unanswered). Touchmark cost 3.6x the task
tokens (T_work) of Rust pooled, 3.0x in the first-session runs alone, and 2.7x the tool calls. Nine trial-3 runs
were re-run in a second session after a usage limit; the differences are in the protocol's change log.
`eval/private/report.py` regenerates every number. The round 1 findings (sortedness, sums, `i128`, modulo, a shorter
reference) are fixed in `5275074`, `53499eb` and `93df923`.
