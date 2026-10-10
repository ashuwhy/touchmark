"""Every hidden test must lie inside the domain its specification states."""

from gen import G
from tasks import BY_NAME

B = 10**9


def ints(a, n_lo, n_hi, lo, hi):
    return n_lo <= len(a) <= n_hi and all(lo <= x <= hi for x in a)


DOMAIN = {
    "palindromic_subarray_sum": lambda a: ints(a, 1, 100_000, 1, B),
    "widest_fence": lambda a: ints(a, 1, 1_000, 1, B),
    "min_max_waiting_time": lambda d, f: ints(d, 1, 50, 1, 20) and ints(f, 2, 2, 1, 50),
    "count_ratio_subarrays": lambda a, x, y: ints(a, 1, 100_000, 1, B) and 1 <= x <= B and 1 <= y <= B,
    "elevator_penalty": lambda n, s, r: 1 <= n <= B and 0 <= s < n and ints(r, 1, 1_500, 0, n - 1)
                                        and len(set(r)) == len(r),
    "max_valid_splits": lambda a: ints(a, 2, 100_000, 1, B),
    "min_ops_subset_sum": lambda a, t: ints(a, 1, 100, 1, 500) and 1 <= t <= 5_000,
    "distant_subarrays": lambda a, g, k: ints(a, 1, 100_000, -B, B) and -B <= g <= B and 0 <= k <= B,
    "shadow_pairs": lambda a: ints(a, 3, 50_000, 1, B),
    "largest_power": lambda a: ints(a, 1, 50_000, 0, 32_767),
    "longest_negation_subarray": lambda a, k: ints(a, 1, 100_000, -100_000, 100_000) and 1 <= k <= 3_000,
    "max_valid_pair_sum": lambda a, k: ints(a, 2, 100_000, 1, B) and 1 <= k <= len(a) - 1,
}

bad = 0
for t in BY_NAME.values():
    for k, args in enumerate(t["tests"]):
        real = [a.values() if isinstance(a, G) else a for a in args]
        if not DOMAIN[t["name"]](*real):
            bad += 1
            print("outside the domain:", t["name"], k, repr(args)[:120])
print(bad, "tests outside their domain,", sum(len(t["tests"]) for t in BY_NAME.values()), "checked")
