"""Cross-check every fast reference against its brute force and against the published examples."""

import random, sys
from refs import *

EXAMPLES = [
    (pal_sum, ([10, 10],), 20), (pal_sum, ([1, 2, 3, 2, 1, 5, 6],), 9),
    (pal_sum, ([7, 1, 2, 1, 7, 3, 4, 3, 4],), 18), (pal_sum, ([1, 2, 3, 4, 5],), 5), (pal_sum, ([1000],), 1000),
    (fence, ([1, 3, 2, 5, 7, 5, 4, 2, 1],), 4), (fence, ([2, 3, 7],), 1),
    (waiting, ([6, 8, 4, 6, 5], [16, 13]), 6), (waiting, ([10, 15], [12, 17]), 0), (waiting, ([10, 5], [8, 8]), -1),
    (ratio, ([1, 2, 1, 2], 3, 2), 7), (ratio, ([2, 2, 1], 2, 1), 3), (ratio, ([2, 2, 2], 1, 1), 0),
    (elevator, (6, 4, [1, 5]), 6), (elevator, (8, 3, [3, 7, 1]), 10), (elevator, (10, 5, [0, 2, 9]), 22),
    (splits, ([10, 30, 15, 10],), 2), (splits, ([2, 10, 14],), 1), (splits, ([2, 4],), 0),
    (subset_ops, ([10, 2], 13), 3), (subset_ops, ([6, 3], 8), 2), (subset_ops, ([2, 2], 7), -1),
    (distant, ([1, 2, 1], 4, 1), 5), (distant, ([2, -1, 3], 2, 2), 2), (distant, ([-3, 1, 2], 0, 3), 2),
    (shadow, ([3, 1, 4, 2, 5],), 5), (shadow, ([6, 7, 8, 9],), 3),
    (largest_power, ([7, 5],), [0] * 12 + [2, 1, 2]), (largest_power, ([3, 1, 7],), [0] * 12 + [1, 2, 3]),
    (negation, ([4, 1, 2], 3), 3), (negation, ([5, 3, 4], 7), 2), (negation, ([2, 2, 5], 6), 2),
    (pair_sum, ([1, 3, 5, 2, 8], 2), 13), (pair_sum, ([5, 1, 9], 1), 14),
]

for fn, args, want in EXAMPLES:
    got = fn(*args)
    assert got == want, (fn.__name__, args, got, want)
print("examples ok", len(EXAMPLES))

rng = random.Random(int(sys.argv[1]) if len(sys.argv) > 1 else 1)
ROUNDS = int(sys.argv[2]) if len(sys.argv) > 2 else 300


def arr(n, lo, hi):
    return [rng.randint(lo, hi) for _ in range(n)]


def pal_arr(n):
    # short alphabets and mirrored blocks make long palindromes likely
    base = arr(rng.randint(1, n), 1, rng.choice([2, 3, 10**9]))
    a = base + base[::-1][rng.randint(0, 1):]
    return a[:n] if len(a) > n else a


cases = {
    "pal_sum": (pal_sum, pal_sum_brute, lambda: (rng.choice([arr(rng.randint(1, 12), 1, rng.choice([2, 3, 9])), pal_arr(12)]),)),
    "fence": (fence, fence_brute, lambda: (arr(rng.randint(1, 8), 1, rng.choice([3, 6, 12])),)),
    "waiting": (waiting, waiting_brute, lambda: (arr(rng.randint(1, 11), 1, rng.choice([5, 20])), arr(2, 1, rng.choice([20, 50])))),
    "ratio": (ratio, ratio_brute, lambda: (arr(rng.randint(1, 30), 1, rng.choice([4, 10**9])), rng.randint(1, 5), rng.randint(1, 5))),
    "elevator": (elevator, elevator_brute, None),
    "splits": (splits, splits_brute, lambda: ([rng.choice([1, 2, 3, 4, 6, 8, 12, 18, 24, 36, 5, 10, 30]) * rng.choice([1, 1, 1, 7]) for _ in range(rng.randint(2, 14))],)),
    "subset_ops": (subset_ops, subset_ops_brute, lambda: (arr(rng.randint(1, 4), 1, rng.choice([8, 40])), rng.randint(1, 60))),
    "distant": (distant, distant_brute, lambda: (arr(rng.randint(1, 25), -6, 6), rng.randint(-8, 8), rng.choice([0, 0, 1, 2, 3, 5, 9]))),
    "shadow": (shadow, shadow_brute, lambda: (arr(rng.randint(3, 25), 1, rng.choice([3, 6, 30])),)),
    "largest_power": (largest_power, largest_power_brute, lambda: (arr(rng.randint(1, 6), 0, rng.choice([7, 31, 2**15 - 1])),)),
    "negation": (negation, negation_brute, lambda: (arr(rng.randint(1, 18), -12, 12), rng.randint(1, 13))),
    "pair_sum": (pair_sum, pair_sum_brute, None),
}


def elevator_case():
    n = rng.randint(1, 15)
    m = rng.randint(1, min(n, 6))
    reqs = rng.sample(range(n), m)
    return (n, rng.randrange(n), reqs)


cases["elevator"] = (elevator, elevator_brute, elevator_case)


def pair_case():
    a = arr(rng.randint(2, 15), 1, rng.choice([3, 20, 10**9]))
    return (a, rng.randint(1, len(a) - 1))


cases["pair_sum"] = (pair_sum, pair_sum_brute, pair_case)

for name, (fast, slow, make) in cases.items():
    for _ in range(ROUNDS):
        args = make()
        a, b = fast(*args), slow(*args)
        assert a == b, (name, args, a, b)
    print(name, "ok")
