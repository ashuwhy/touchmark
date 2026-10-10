"""Post-hoc: plant one typical bug in a Rust gold and score it with the frozen harness, to show the
hidden tests can fail a solution. Not part of the pre-registered scoring."""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
GOLD = os.path.join(HERE, "..", "gold")
sys.path.insert(0, os.path.join(HERE, "..", "private"))
os.chdir(os.path.join(HERE, "..", "private"))
import harness as h

MUTANTS = [
    ("palindromic_subarray_sum", "quadratic expansion instead of Manacher",
     "let mut k = if i > r { 1 } else { d1[(l + r - i) as usize].min(r - i + 1) };", "let mut k = 1;"),
    ("widest_fence", "forgets pairs of two equal heights",
     "pairs.push((2 * vals[i], cnt[i] / 2));", ""),
    ("min_max_waiting_time", "returns 0 instead of -1 when no car is served",
     "if best_count == 0 { -1 } else { best_wait }", "best_wait"),
    ("count_ratio_subarrays", "strict comparison: misses x / y == a / b",
     "while i < mid && p[i] < p[j] {", "while i < mid && p[i] <= p[j] {"),
    ("elevator_penalty", "a request on the start floor is not free",
     ".filter(|&r| r != start)", ".filter(|_| true)"),
    ("max_valid_splits", "never deletes the first or last element",
     "for k in 0..n {", "for k in 1..n - 1 {"),
    ("min_ops_subset_sum", "never halves an element down to 1",
     "while (x >> k) > 0 {", "while (x >> k) > 1 {"),
    ("distant_subarrays", "drops the k == 0 case (equal sums counted twice)",
     "if k == 0 {", "if false {"),
    ("distant_subarrays", "subarray count kept in 32 bits",
     "return (n * (n + 1) / 2) as i64;", "return ((n * (n + 1) / 2) as i32) as i64;"),
    ("shadow_pairs", "equal values never pair",
     "if c[x] >= v {", "if c[x] > v {"),
    ("largest_power", "bit mask one bit short",
     "let mut mask = (1 << 15) - 1;", "let mut mask = (1 << 14) - 1;"),
    ("longest_negation_subarray", "% instead of rem_euclid on negative prefix sums",
     "p = (p + nums[t - 1]).rem_euclid(k);", "p = (p + nums[t - 1]) % k;"),
]

for task, what, old, new in MUTANTS:
    src = open(os.path.join(GOLD, f"{task}.rs")).read()
    assert src.count(old) == 1, (task, old)
    s = h.score(h.BY_NAME[task], "rust", src.replace(old, new))
    bad = {x: s["results"].count(x) for x in sorted(set(s["results"])) if x != "pass"}
    print(f"{task:26s} {s['passed']:2d}/{s['total']} {bad or 'all pass'}  {what}", flush=True)
