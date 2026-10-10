# Why Touchmark is frozen

I stopped work on Touchmark on 11 October 2026. Nothing is deleted. The compiler builds, `cargo test` passes,
and every result below can be regenerated from this commit.

## The claim it could not show

Touchmark stands on one claim: an agent writing Touchmark ships fewer bugs than an agent writing Rust,
because the compiler proves what the agent would otherwise have to test. I pre-registered two rounds to
test it, with fixed hashes and stop rules, and neither could answer it. Frontier agents writing plain Rust
shipped no bugs at all, so Touchmark had nothing to beat.

| Round | Tasks | Rust runs with a defect | Touchmark runs with a defect | Touchmark task tokens |
|---|---|---|---|---|
| 1 ([results](../eval/RESULTS.md)) | 12 Easy and Medium, built to provoke overflow, off-by-one, modulo and empty-array bugs, 3 trials | 0 of 36 | 0 of 36 | 3.6x Rust |
| 2 ([results](../eval/round2/RESULTS.md)) | 11 LeetCode Hard from after the model's training cutoff, 223 hidden tests | 0 of 11 | not run: the stop rule fired | 1.9x on one pilot pair |

The hidden tests were not the problem. In round 2 I planted one typical bug in each gold solution and the
tests caught 11 of 12; the twelfth is very likely not a bug.

The cause was the same in both rounds. Every agent wrote a brute force straight from the problem statement
and compared it with its own answer on 3,000 to 400,000 random inputs before saying DONE. Agents verify
what they can sample. When a specification can be run on small inputs, testing is cheap and catches what
a proof would catch, and the proof costs more tokens.

## Why I did not run round 3

Round 3 is ready to specify (see [`HANDOFF.md`](../HANDOFF.md)): the same tasks, compiler only, no running
code. Touchmark might win there, but the claim it would support is narrow: proofs stand in for tests when
tests are unavailable. Agents can almost always run tests. The other ways past the ceiling are a weaker
model, which shows less about where agents are going, or tasks with no cheap oracle, which means building
a stateful, multi-module task suite LeetCode cannot supply. That is months of work toward a claim I no
longer think is the important one.

The larger reason is that a language is the wrong unit for this bet. Nobody gets the benefit until code
moves into it, and two rounds gave no sign that agents need it to write correct small programs. The parts
of Touchmark that hold up on their own are not tied to the language.

## What is worth keeping

- **The contract lock** (`tmk lock`, `touchplate.lock`, error `E0301`). An SMT check that a new contract
  is not weaker than the pinned one, with a concrete input the new version allows and the old forbade.
  It is the one piece aimed at agents rewriting the rules instead of the code, and it would work on
  contracts written in any language.
- **Inference** of loop invariants and private functions' contracts, Houdini style, including the
  slow-caller fix in `93df923`.
- **`tmk examples`**: what a contract allows, forbids and rejects, shown with values from the solver,
  and whether it decides the result.
- **The evaluation method**: pre-registration with hashes, stop rules that saved 55 runs in round 2,
  planted bugs to test the tests, and the stricter access audit in
  [`eval/round2/posthoc/audit.py`](../eval/round2/posthoc/audit.py).
- **Speed**: proved code within 18% of Rust on the array benchmarks
  ([`bench/arrays/RESULTS.md`](../bench/arrays/RESULTS.md)).

## If I pick it up again

Start from round 3 in [`HANDOFF.md`](../HANDOFF.md). Fold `posthoc/audit.py` into `usage.py` before
pre-registering, and run the Rust calibration first with the same stop rule.
