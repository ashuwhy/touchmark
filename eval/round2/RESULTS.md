# Agent evaluation, round 2: results (stopped after the Rust calibration)

Protocol: `eval/round2/PROTOCOL.md` (pre-registered on 2026-10-01, commit `519f20c`). The tests, references,
generator, scorer and gold solutions are in `eval/round2/private/` and `eval/round2/gold/`, and every pre-registered
SHA-256 matches, checked again after scoring (`tasks.py` 9bf60028..., `harness.py` ef59b3bb..., `expected.json`
5e436b5f..., `gold/` b325aaa2... by the command given in the protocol's change log). Every scored solution and the
agents' own test files are in `eval/round2/solutions/<run>/`. `eval/round2/private/report.py` regenerates every number
below from `results.json`.

**Status.** Stopped by the pre-registered stop rule. 0 of the 11 Rust calibration runs shipped a defect, so no
Touchmark run was made and the round reports the calibration alone. The question round 2 was built to answer, whether
Touchmark cuts escaped defects, is still unanswered.

## Calibration: 11 Rust runs on LeetCode Hard

| Run | Tests | T_work | T_ctx | T_proc | Tool calls | Compiler runs | Wall s | Solution tokens |
|---|---|---|---|---|---|---|---|---|
| `r2t1-palindromic_subarray_sum-rs` | 21/21 | 7,783 | 70,118 | 405,670 | 5 | 4 | 49 | 500 |
| `r2t1-widest_fence-rs` | 21/21 | 6,437 | 68,775 | 332,236 | 4 | 3 | 43 | 430 |
| `r2t1-min_max_waiting_time-rs` | 22/22 | 10,515 | 73,149 | 489,543 | 6 | 3 | 254 | 673 |
| `r2t1-count_ratio_subarrays-rs` | 19/19 | 7,140 | 69,509 | 402,073 | 5 | 3 | 38 | 382 |
| `r2t1-elevator_penalty-rs` | 20/20 | 12,645 | 75,047 | 562,649 | 7 | 3 | 73 | 685 |
| `r2t1-max_valid_splits-rs` | 20/20 | 15,219 | 77,578 | 439,973 | 5 | 4 | 128 | 1215 |
| `r2t1-min_ops_subset_sum-rs` | 20/20 | 15,372 | 77,757 | 654,337 | 8 | 5 | 145 | 487 |
| `r2t1-distant_subarrays-rs` | 20/20 | 5,996 | 68,327 | 197,343 | 2 | 1 | 33 | 476 |
| `r2t1-shadow_pairs-rs` | 20/20 | 13,961 | 76,228 | 434,881 | 5 | 4 | 273 | 883 |
| `r2t1-largest_power-rs` | 19/19 | 6,865 | 69,255 | 267,925 | 3 | 2 | 48 | 395 |
| `r2t1-longest_negation_subarray-rs` | 21/21 | 11,458 | 73,818 | 350,378 | 4 | 2 | 93 | 663 |
| **Mean** (wall: median) | **223/223** | **10,308** | **72,687** | **412,455** | **4.9** | **3.1** | **73** | **617** |

**0 of 11 defective.** Every run passed every hidden test, answered DONE with a solution that builds, and stayed inside
its directory (the audit flagged no path). The slowest hidden test took 0.027 s against the 1.0 s limit. With 0 or 1
defective runs the protocol stops the round: even 3 Rust defects against 0 over 33 runs an arm would give Fisher
p = 0.24, so 55 more agents would have compared a ceiling with a ceiling.

Against round 1's Rust arm, the harder tasks cost more work and produced no more bugs: T_work 2.7x (10,308 against
3,765), tool calls 2x (4.9 against 2.5), median wall time 73 s against 10.9 s, solutions 3.8x longer (617 o200k tokens
against 162).

No Rust run hit a real compile error. Seven of the eleven (and the Rust pilot) saw one rustc failure, all the same
one: an agent checked its file with `rustc ... -o /dev/null`, and rustc could not create its temp directory beside the
output (`couldn't create a temp dir ... at path "/dev/rmeta..."`). That is why the table has no "with an error" column
as round 1 did.

## Why the ceiling held: every agent tested against a brute force

Each of the 11 agents wrote, beside its solution, a by-definition brute force and a seeded random differential test,
and ran it before answering DONE. All 11 also asserted the published examples and timed a large input, and most
asserted the extremes of the domain.

| Run | Oracle | Random cases |
|---|---|---|
| `palindromic_subarray_sum` | brute force | 200,000 |
| `widest_fence` | brute force | 30,000 |
| `min_max_waiting_time` | brute force over schedules | 200,000 |
| `count_ratio_subarrays` | brute force | 30,000 |
| `elevator_penalty` | step-by-step simulation | 3,600 |
| `max_valid_splits` | brute force | 400,000 |
| `min_ops_subset_sum` | brute force | 4,000 |
| `distant_subarrays` | brute force | 200,000 |
| `shadow_pairs` | the definition, and an O(n^2) scan | 3,000, plus 30 of up to 2,000 elements |
| `largest_power` | brute force | 20,000 |
| `longest_negation_subarray` | brute force | 200,000 |

These tasks have a cheap oracle: the problem statement, run on small inputs. An agent with a shell turns it into a
test of 55 to 130 lines. Most bugs these tasks invite (a missed equal case, an off-by-one at the ends, a negative
remainder) show up on ten-element inputs; the ones that need size, overflow and time, the agents went after with
extreme values and the timed input. That is the same mechanism as round 1, one level up. It is also the
case Touchmark's pitch is about, "the proof replaces the tests", and here the tests were cheap: the token breakdown puts
compiler and test runs at 930 to 2,940 tokens a run.

## Checks I added after the stop (exploratory, not pre-registered)

Neither check is part of any decision. They answer two doubts about a 0 of 11 result: that the hidden tests could not
fail anything, and that the passes were luck.

**Planted bugs.** I put one typical bug into each Rust gold and scored it with the frozen harness
(`posthoc/mutants.py`, output in `posthoc/mutants.log`):

| Task | Planted bug | Hidden tests passed |
|---|---|---|
| `palindromic_subarray_sum` | quadratic expansion instead of Manacher | 20/21 (1 timeout) |
| `widest_fence` | forgets pairs of two equal heights | 18/21 |
| `min_max_waiting_time` | 0 instead of -1 when no car is served | 20/22 |
| `count_ratio_subarrays` | strict comparison, misses x / y == a / b | 11/19 |
| `elevator_penalty` | a request on the start floor is not filtered out | 20/20 |
| `max_valid_splits` | never deletes the first or last element | 19/20 |
| `min_ops_subset_sum` | never halves an element down to 1 | 13/20 |
| `distant_subarrays` | drops the k == 0 case | 17/20 |
| `distant_subarrays` | subarray count kept in 32 bits | 19/20 |
| `shadow_pairs` | equal values never pair | 12/20 |
| `largest_power` | bit mask one bit short | 14/19 |
| `longest_negation_subarray` | `%` instead of `rem_euclid` on negative prefix sums | 11/21 (10 crashes) |

The tests failed 11 of the 12. The elevator change survived because it is very likely not a bug: with the filter removed,
the gold also passes 4,600 random inputs against a brute force, so the rest of the algorithm already gives a request
on the start floor its right cost. Three real bugs were caught by a
single test (the quadratic expansion, the first-and-last deletion, the 32-bit count), so for those tasks the suite is
thinner than its 20 tests suggest. A solution with one of those bugs would still have been scored defective.

**Fresh random inputs.** `posthoc/difftest.py` reuses the generators and brute forces in `check_refs.py` and runs
each solution on 2,000 small inputs against the brute force and 300 medium inputs near the value bounds against the
Python reference, through one Rust driver for all tasks. With two seeds, every calibration solution passed all 4,600;
the golds, run first to check the tester, passed all 2,300 (`posthoc/difftest.log`).

## Pilot (`max_valid_pair_sum`, Medium; not counted)

| Run | Tests | T_work | T_ctx | T_proc | Tool calls | Compiler runs | Wall s | Solution tokens |
|---|---|---|---|---|---|---|---|---|
| `p2-max_valid_pair_sum-rs` | 17/17 | 6,496 | 66,257 | 386,503 | 5 | 3 | 24 | 154 |
| `p2-max_valid_pair_sum-tmk` | 17/17 | 12,405 | 72,197 | 547,647 | 7 | 5 | 37 | 408 |

The Touchmark solution proves 30 checks and keeps none at run time. Its agent hit one real verifier error, E0207 (the
loop body could break an invariant, with a 1,012-element counterexample), and fixed it in the next compile. After the proof it still ran a test program of its own and checked a
100,000-element input against a Python version, as the round 1 Touchmark agents did. Reading
the reference cost 3,119 tokens and compiler runs 1,703. The T_work ratio is 1.91, under round 1's 3.0x, but it is one
Medium task and one run in each language, so it says nothing about the token rule, which the protocol judges at
round 3.

## What went differently from the protocol

All four are in the protocol's change log with dates.

- The machine restarted between the pilot (2026-10-01) and the calibration (2026-10-10) and cleared `/tmp`, which held
  the generated prompts and my ledger of agent IDs. I regenerated the prompts from the unchanged `harness.py`; all 13
  match the first message of their agents' transcripts byte for byte.
- The protocol describes the gold hash wrongly. The value is right for `cd gold && shasum -a 256 *.rs *.tmk | shasum
  -a 256`.
- `report.py` had no path for a round without Touchmark runs, and the wall times, solution sizes and token breakdown
  lived only in the transcripts. I added a calibration report and `annotate.py`, which copies those fields into
  `results.json`. The scored fields are unchanged.
- The two checks above.

## Honest reading

1. The defect question is unanswered a second time. Moving from Easy and Medium single functions to 11 LeetCode Hard
   problems published after the model's training cutoff did not make a frontier agent ship Rust bugs: 0 of 11 runs,
   223 of 223 tests. The stop rule saved 55 runs that could only have compared 0 with 0.
2. The ceiling is not about difficulty. It comes from the agent testing its own code against a brute force it writes
   from the specification, which any task with a small-input oracle allows. Harder tasks only made that testing longer
   (T_work 2.7x round 1's).
3. The token rule is unmeasured on hard tasks. The pilot's 1.9x is one pair of runs.

## Next: round 3

I would test the claim where it can lose: **no execution**. Same 11 tasks, tests and scorer; the prompt allows the
compiler (`rustc`, or `tmk check` and `tmk build`) but forbids running any binary or script, and the transcript audit
checks it. That is where "the proof replaces the tests" is supposed to pay off, and where a Rust agent loses the brute
force that carried it here. It needs its own pre-registration and the same stop rule, on a Rust calibration first.

The cost of that choice: it is a narrower claim. Real agents can usually run tests, so a Touchmark win there would show
that proofs stand in for tests when tests are unavailable, not that Touchmark agents ship fewer bugs in general. The
other two ways past the ceiling are a weaker model (Haiku 5.5), where Rust defects may exist even with tests, and tasks
without a cheap oracle (stateful, multi-module code whose specification is not executable), which LeetCode cannot
supply.
