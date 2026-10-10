"""Round 2 tasks: the 11 LeetCode problems labelled Hard, from contests that started between
2026-07-01 and 2026-09-30, whose parameters and results are integers, booleans or integer arrays.
Specifications are my own wording of each problem. Kept out of the agents' reach until every run
is scored; its SHA-256 is committed in eval/round2/PROTOCOL.md before the first counted run.

Array arguments are either literal lists or G(...) generator specs, which the test program
expands in-language (see gen.py) so that 100,000-element inputs do not become literals."""

import random

from gen import G
from refs import (pal_sum, fence, waiting, ratio, elevator, splits, subset_ops, distant, shadow,
                  largest_power, negation, pair_sum)

TASKS = []


def task(name, source, params, ret, spec, ref):
    t = {"name": name, "source": source, "params": params, "ret": ret, "spec": spec, "ref": ref, "tests": []}
    TASKS.append(t)
    return t


def rnd(seed):
    return random.Random(seed)


def lits(seed, count, n_lo, n_hi, lo, hi):
    r = rnd(seed)
    return [[r.randint(lo, hi) for _ in range(r.randint(n_lo, n_hi))] for _ in range(count)]


B = 10**9

t = task("palindromic_subarray_sum", "weekly-contest-509 Q4 palindromic-subarray-sum",
         [("nums", "ints")], "int", """\
`nums` has length between 1 and 100_000, and every element satisfies 1 <= nums[i] <= 1_000_000_000.
A subarray is a non-empty run of consecutive elements; it is a palindrome if it reads the same
forwards and backwards. Return the largest sum of a subarray that is a palindrome.
Examples: palindromic_subarray_sum([1, 2, 3, 2, 1, 5, 6]) = 9 (the subarray [1, 2, 3, 2, 1]);
palindromic_subarray_sum([1, 2, 3, 4, 5]) = 5 (one element is a palindrome);
palindromic_subarray_sum([10, 10]) = 20.""", pal_sum)
t["tests"] = [
    ([10, 10],), ([1, 2, 3, 2, 1, 5, 6],), ([7, 1, 2, 1, 7, 3, 4, 3, 4],), ([1, 2, 3, 4, 5],), ([1000],),
    ([1],), ([B],), ([B, B],), ([1, 2, 1, 100],), ([3, 9, 9, 3, 1],), ([5, 1, 5, 1, 5, 2],),
    *[(a,) for a in lits(101, 3, 20, 200, 1, 2)], *[(a,) for a in lits(102, 2, 100, 200, 1, 3)],
    (G(100_000, 11, B, B),), (G(100_000, 12, 1, 2),), (G(100_000, 13, 1, B),),
    (G(100_000, 14, B - 20_000, B, 3, 20_000),), (G(100_000, 15, 1, B, 1),),
]

t = task("widest_fence", "biweekly-contest-188 Q2 widest-possible-fence",
         [("planks", "ints")], "int", """\
`planks` has length between 1 and 1_000, and every element satisfies 1 <= planks[i] <= 1_000_000_000:
the height of a plank of width 1. A fence is a set of pieces that all have the same height. A piece
is either one plank, with its own height, or two different planks (two different indices; their
heights may be equal) joined end to end, with the sum of their heights. Each plank is used in at most
one piece, and planks may be left unused. Return the largest possible number of pieces in a fence.
Examples: widest_fence([1, 3, 2, 5, 7, 5, 4, 2, 1]) = 4 (four pieces of height 5: 5, 5, 1 + 4 and
3 + 2); widest_fence([2, 3, 7]) = 1.""", fence)
t["tests"] = [
    ([1, 3, 2, 5, 7, 5, 4, 2, 1],), ([2, 3, 7],), ([1],), ([5, 5],), ([1, 1],), ([1, 1, 2],),
    ([3, 3, 3, 3, 6],), ([B, B, B],), ([B, B, B, B, B - 1, 1],), ([2, 2, 2, 4, 4, 1, 3],),
    *[(a,) for a in lits(201, 2, 5, 9, 1, 6)], *[(a,) for a in lits(202, 2, 40, 60, 1, 20)],
    *[(a,) for a in lits(203, 1, 200, 200, 1, B)], *[(a,) for a in lits(204, 1, 200, 200, 1, 50)],
    (G(1_000, 21, 1, 30),), (G(1_000, 22, 1, B),), (G(1_000, 23, 1, 1_000, 1),), (G(1_000, 24, 7, 7),),
    (G(1_000, 25, 1, 1_000, 4),),
]

t = task("min_max_waiting_time", "biweekly-contest-188 Q4 minimum-possible-maximum-waiting-time",
         [("demand", "ints"), ("fuel", "ints")], "int", """\
Cars 0, 1, ..., n-1 refuel at two dispensers, numbered 0 and 1. `demand` has length n between 1 and 50,
with 1 <= demand[i] <= 20: car i needs demand[i] units of fuel, and refuelling it keeps a dispenser
busy for demand[i] seconds. `fuel` has length exactly 2, with 1 <= fuel[j] <= 50: the fuel dispenser
j holds at the start.
Cars are released one at a time in index order: car 0 at time 0, and car i (i > 0) at the moment car
i - 1 starts refuelling. When a car is released it must be assigned a dispenser whose remaining fuel is
at least its demand; if both qualify, either may be chosen, whatever their current queues. If neither
qualifies, the process stops: that car and all later cars are not served. An assigned car starts
refuelling as soon as its dispenser has finished all earlier cars (it cannot switch or wait longer),
takes demand[i] fuel from the dispenser when it starts, and keeps it busy for demand[i] seconds. A
car's waiting time is its start time minus its release time.
Among the assignments that serve the largest possible number of cars, return the smallest possible
value of the largest waiting time of a served car. Return -1 if no car can be served.
Examples: min_max_waiting_time([6, 8, 4, 6, 5], [16, 13]) = 6; min_max_waiting_time([10, 15], [12, 17])
= 0; min_max_waiting_time([10, 5], [8, 8]) = -1.""", waiting)
r = rnd(301)
t["tests"] = [
    ([6, 8, 4, 6, 5], [16, 13]), ([10, 15], [12, 17]), ([10, 5], [8, 8]), ([1], [1, 1]), ([20], [19, 19]),
    ([20], [1, 20]), ([5, 5, 5, 5], [10, 50]), ([20, 20, 20], [50, 50]), ([1] * 50, [50, 50]),
    ([20] * 50, [50, 50]), ([3, 7, 2, 9, 4, 4, 8, 1, 6, 5], [30, 20]),
    *[([r.randint(1, 20) for _ in range(r.randint(5, 50))], [r.randint(1, 50), r.randint(1, 50)]) for _ in range(8)],
    *[([r.randint(1, 6) for _ in range(50)], [r.randint(30, 50), r.randint(30, 50)]) for _ in range(3)],
]

t = task("count_ratio_subarrays", "weekly-contest-513 Q4 count-subarrays-with-even-odd-ratio-ii",
         [("nums", "ints"), ("a", "int"), ("b", "int")], "int", """\
`nums` has length between 1 and 100_000, and every element satisfies 1 <= nums[i] <= 1_000_000_000;
1 <= a <= 1_000_000_000 and 1 <= b <= 1_000_000_000. For a subarray (a non-empty run of consecutive
elements), let x be the number of even elements in it and y the number of odd elements. The subarray
is valid if y > 0 and x / y <= a / b, comparing exact rational values. Return the number of valid
subarrays. Examples: count_ratio_subarrays([1, 2, 1, 2], 3, 2) = 7; count_ratio_subarrays([2, 2, 1], 2, 1)
= 3; count_ratio_subarrays([2, 2, 2], 1, 1) = 0.""", ratio)
r = rnd(401)
t["tests"] = [
    ([1, 2, 1, 2], 3, 2), ([2, 2, 1], 2, 1), ([2, 2, 2], 1, 1), ([1], 1, 1), ([2], B, 1), ([1, 2], 1, B),
    ([B - 1, B, B - 1], B, B), ([2, 1, 2, 2, 1, 2, 2, 2, 1], 2, 1),
    *[(a, r.randint(1, 4), r.randint(1, 4)) for a in lits(402, 3, 10, 40, 1, 4)],
    *[(a, r.randint(1, B), r.randint(1, B)) for a in lits(403, 2, 150, 200, 1, B)],
    (G(100_000, 41, 1, 1), 1, 1), (G(100_000, 42, 1, B), 1, 1), (G(100_000, 43, 1, B), B, 1),
    (G(100_000, 44, 1, B), 1, B), (G(100_000, 45, 1, 6, 3, 5), 2, 3), (G(100_000, 46, 2, 2), B, B),
]

t = task("elevator_penalty", "biweekly-contest-189 Q4 elevator-requests-ii",
         [("n", "int"), ("start", "int"), ("requests", "ints")], "int", """\
A building has floors 0 to n - 1, with 1 <= n <= 1_000_000_000. An elevator is at floor `start`
(0 <= start <= n - 1) at time 0. `requests` has length between 1 and 1_500 and holds distinct floors,
each between 0 and n - 1. Every second, until all requests are fulfilled, the elevator moves exactly one
floor up or down. A request is fulfilled the first time the elevator is at its floor (at time 0 if its
floor is `start`), and it costs the time at which it is fulfilled. Return the smallest possible total
cost of all requests. Examples: elevator_penalty(6, 4, [1, 5]) = 6 (floor 5 at time 1, floor 1 at time
5); elevator_penalty(8, 3, [3, 7, 1]) = 10; elevator_penalty(10, 5, [0, 2, 9]) = 22.""", elevator)
r = rnd(501)


def floors(n, m):
    return r.sample(range(n), m)


t["tests"] = [
    (6, 4, [1, 5]), (8, 3, [3, 7, 1]), (10, 5, [0, 2, 9]), (1, 0, [0]), (B, 0, [B - 1]),
    (B, B // 2, [0, B - 1]), (B, B - 1, [0, 1, 2, B - 2]), (12, 6, [0, 11, 5, 7, 6]),
    *[(n, r.randrange(n), floors(n, r.randint(1, min(n, 8)))) for n in (9, 15, 40, 1000)],
    (B, r.randrange(B), floors(B, 200)), (300, 150, floors(300, 200)),
    (B, 0, G(1_500, 51, 0, B - 1, 4)), (B, B // 2, G(1_500, 52, 0, B - 1, 4)),
    (B, B - 1, G(1_500, 53, 0, B - 1, 4)), (1_500, 700, G(1_500, 54, 0, 1_499, 4)),
    (2_000, 1_999, G(1_500, 55, 0, 1_999, 4)), (B, 123_456_789, G(1_500, 56, 0, B - 1, 4)),
]

t = task("max_valid_splits", "biweekly-contest-190 Q4 maximum-valid-split-positions-ii",
         [("nums", "ints")], "int", """\
`nums` has length between 2 and 100_000, and every element satisfies 1 <= nums[i] <= 1_000_000_000.
You may delete at most one element (or none); call the remaining array arr, of length m, in the
original order. A split position i of arr (0 <= i < m - 1) is valid if gcd(arr[0], ..., arr[i]) equals
gcd(arr[i + 1], ..., arr[m - 1]), where gcd is the greatest common divisor. The score of arr is its
number of valid split positions (an array of length 1 has none). Return the largest score you can reach.
Examples: max_valid_splits([10, 30, 15, 10]) = 2 (delete 15); max_valid_splits([2, 10, 14]) = 1;
max_valid_splits([2, 4]) = 0.""", splits)
r = rnd(601)
DIV = [1, 2, 3, 4, 6, 8, 12, 18, 24, 36, 5, 10, 30, 60, 90, 120]
t["tests"] = [
    ([10, 30, 15, 10],), ([2, 10, 14],), ([2, 4],), ([1, 1],), ([5, 7],), ([6, 4, 6],), ([7, 7, 7, 7],),
    ([B, B],), ([4, 6, 2, 8, 12, 3],), ([12, 18, 5, 30, 6],),
    *[([r.choice(DIV) * r.choice([1, 1, 7]) for _ in range(r.randint(3, 30))],) for _ in range(4)],
    ([r.choice(DIV) * 999_983 for _ in range(200)],),
    (G(100_000, 61, B, B),), (G(100_000, 62, 1, B),), (G(100_000, 63, 6, B, 6, 6),),
    (G(100_000, 64, 30, B, 6, 30),), (G(100_000, 65, 1, 100_000, 1),),
]

t = task("min_ops_subset_sum", "weekly-contest-517 Q4 minimum-operations-to-form-subset-sum-ii",
         [("nums", "ints"), ("target", "int")], "int", """\
`nums` has length between 1 and 100, and every element satisfies 1 <= nums[i] <= 500;
1 <= target <= 5_000. One operation picks an element with current value v and replaces it with 2 * v
or with floor(v / 2). Operations on the same element may be applied any number of times, in any order.
Return the smallest total number of operations after which some subset of the elements sums to
exactly `target`, or -1 if no number of operations achieves that. Examples:
min_ops_subset_sum([10, 2], 13) = 3 (10 -> 5, and 2 -> 4 -> 8; then 5 + 8 = 13);
min_ops_subset_sum([6, 3], 8) = 2 (3 -> 1 -> 2; then 6 + 2 = 8); min_ops_subset_sum([2, 2], 7) = -1.""",
         subset_ops)
r = rnd(701)
t["tests"] = [
    ([10, 2], 13), ([6, 3], 8), ([2, 2], 7), ([1], 1), ([1], 5_000), ([500] * 100, 5_000), ([3], 4_096),
    ([1, 1], 7), ([1, 1, 1], 7), ([500], 1), ([7, 7, 7], 5_000), ([5, 6, 10], 4),
    *[([r.randint(1, 500) for _ in range(100)], r.randint(1, 5_000)) for _ in range(3)],
    *[([r.randint(1, 20) for _ in range(r.randint(2, 6))], r.randint(100, 5_000)) for _ in range(3)],
    ([r.randint(1, 500) for _ in range(100)], 4_999), ([1] * 100, 5_000),
]

t = task("distant_subarrays", "biweekly-contest-191 Q4 count-subarrays-with-distant-sums",
         [("nums", "ints"), ("goal", "int"), ("k", "int")], "int", """\
`nums` has length between 1 and 100_000, and every element satisfies
-1_000_000_000 <= nums[i] <= 1_000_000_000; -1_000_000_000 <= goal <= 1_000_000_000 and
0 <= k <= 1_000_000_000. A subarray (a non-empty run of consecutive elements) is distant if the
absolute difference between its sum and goal is at least k. Return the number of distant subarrays.
Examples: distant_subarrays([1, 2, 1], 4, 1) = 5; distant_subarrays([2, -1, 3], 2, 2) = 2;
distant_subarrays([-3, 1, 2], 0, 3) = 2.""", distant)
r = rnd(801)
t["tests"] = [
    ([1, 2, 1], 4, 1), ([2, -1, 3], 2, 2), ([-3, 1, 2], 0, 3), ([5], 0, 0), ([1, 2, 3], 6, 0),
    ([B, B, B], -B, B), ([-B, -B], B, B), ([0, 0, 0], 0, 1), ([3, -3, 3, -3], 0, 3),
    *[(a, r.randint(-5, 5), r.randint(0, 4)) for a in lits(802, 3, 10, 40, -4, 4)],
    *[(a, r.randint(-B, B), r.randint(0, B)) for a in lits(803, 2, 150, 200, -B, B)],
    (G(100_000, 81, -B, B), 12_345, 1_000), (G(100_000, 82, B, B), -B, B), (G(100_000, 83, -B, B), 0, 0),
    (G(100_000, 84, -3, 3), 0, 1), (G(100_000, 85, -B, B), -B, B), (G(100_000, 86, -5, 5, 3, 5), 7, 3),
]

t = task("shadow_pairs", "weekly-contest-519 Q4 count-shadow-pairs-ii",
         [("nums", "ints")], "int", """\
`nums` has length n between 3 and 50_000, and every element satisfies 1 <= nums[i] <= 1_000_000_000.
A pair of indices (i, j) is a shadow pair if i < j, nums[i] < nums[j], and no index k with i < k < j
has nums[i] < nums[k] < nums[j]. Return the number of shadow pairs.
Examples: shadow_pairs([3, 1, 4, 2, 5]) = 5; shadow_pairs([6, 7, 8, 9]) = 3.""", shadow)
t["tests"] = [
    ([3, 1, 4, 2, 5],), ([6, 7, 8, 9],), ([1, 1, 1],), ([1, 2, 3],), ([3, 2, 1],), ([1, 1, 2],),
    ([2, 1, 2, 1, 2],), ([B, 1, B],), ([4, 3, 2, 1, 8, 7, 6, 5],),
    *[(a,) for a in lits(901, 3, 10, 40, 1, 5)], *[(a,) for a in lits(902, 2, 150, 200, 1, B)],
    (G(50_000, 91, 1, 60_000, 5, 25_000),), (G(50_000, 92, 1, 2),), (G(50_000, 93, 1, B),),
    (G(50_000, 94, 1, 30_000, 1),), (G(50_000, 95, 1, B, 2),), (G(50_000, 96, 1, 1_000, 3, 999),),
]

t = task("largest_power", "weekly-contest-520 Q4 lexicographically-largest-power-array",
         [("nums", "ints")], "ints", """\
`nums` has length n between 1 and 50_000, and every element satisfies 0 <= nums[i] <= 32_767 (15 bits).
For an ordering p of the elements of nums (any permutation), define the array power of length 15:
power[i] is the largest j with 0 <= j <= n such that p[0], ..., p[j - 1] all have bit (14 - i) set
(bit 0 is the least significant bit). Return the lexicographically largest power array over all
orderings, as an array of 15 integers (compare two arrays at the first index where they differ).
Examples: largest_power([7, 5]) = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2];
largest_power([3, 1, 7]) = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3].""", largest_power)
t["tests"] = [
    ([7, 5],), ([3, 1, 7],), ([0],), ([32_767],), ([32_767] * 3,), ([25, 22, 17],), ([12, 11, 7, 13],),
    ([16_384, 8_191, 24_576, 1],),
    *[(a,) for a in lits(1001, 4, 5, 12, 0, 63)], *[(a,) for a in lits(1002, 2, 150, 200, 0, 32_767)],
    (G(50_000, 101, 0, 32_767),), (G(50_000, 102, 32_767, 32_767),), (G(50_000, 103, 16_384, 32_767),),
    (G(50_000, 104, 0, 32_767, 1),), (G(50_000, 105, 7, 32_767, 6, 7),),
]

t = task("longest_negation_subarray", "biweekly-contest-192 Q4 longest-subarray-divisible-by-k-with-at-most-one-negation-ii",
         [("nums", "ints"), ("k", "int")], "int", """\
`nums` has length between 1 and 100_000, and every element satisfies -100_000 <= nums[i] <= 100_000;
1 <= k <= 3_000. A subarray (a non-empty run of consecutive elements) is valid if its sum is divisible
by k (s = k * q for some integer q, so 0 and negative multiples count), or becomes divisible by k after
negating exactly one of its elements (replacing x by -x). Return the length of the longest valid
subarray, or 0 if there is none. Examples: longest_negation_subarray([4, 1, 2], 3) = 3
(4 + 1 - 2 = 3); longest_negation_subarray([5, 3, 4], 7) = 2; longest_negation_subarray([2, 2, 5], 6)
= 2.""", negation)
r = rnd(1101)
t["tests"] = [
    ([4, 1, 2], 3), ([5, 3, 4], 7), ([2, 2, 5], 6), ([0], 5), ([1], 1), ([1], 2), ([-3], 3),
    ([100_000, -100_000], 3_000), ([-7, -5, 11, 1], 4), ([1, 1, 1, 1, 1], 3_000),
    *[(a, r.randint(1, 12)) for a in lits(1102, 3, 10, 40, -20, 20)],
    *[(a, r.randint(1, 3_000)) for a in lits(1103, 2, 150, 200, -100_000, 100_000)],
    (G(100_000, 111, -100_000, 100_000), 3_000), (G(100_000, 112, -100_000, 100_000), 1),
    (G(100_000, 113, -100_000, 100_000), 2), (G(100_000, 114, 1, 1), 3_000),
    (G(100_000, 115, -100_000, 100_000), 2_999), (G(100_000, 116, -50, 50, 3, 100), 97),
]

# The pilot checks the harness, the prompts and the token accounting; it is not counted.
t = task("max_valid_pair_sum", "biweekly-contest-186 Q2 maximum-valid-pair-sum (Medium)",
         [("nums", "ints"), ("k", "int")], "int", """\
`nums` has length n between 2 and 100_000, every element satisfies 1 <= nums[i] <= 1_000_000_000, and
1 <= k <= n - 1. Return the largest nums[i] + nums[j] over all pairs of indices with i < j and
j - i >= k. Examples: max_valid_pair_sum([1, 3, 5, 2, 8], 2) = 13 (5 + 8);
max_valid_pair_sum([5, 1, 9], 1) = 14.""", pair_sum)
r = rnd(1201)
t["tests"] = [
    ([1, 3, 5, 2, 8], 2), ([5, 1, 9], 1), ([1, 1], 1), ([B, B], 1), ([9, 1, 1, 9], 3), ([9, 1, 1, 9], 1),
    ([1, 9, 9, 1], 2), ([3, 2, 1], 2),
    *[(a, r.randint(1, len(a) - 1)) for a in lits(1202, 4, 2, 30, 1, 50)],
    (G(100_000, 121, 1, B), 1), (G(100_000, 122, 1, B), 99_999), (G(100_000, 123, B, B), 50_000),
    (G(100_000, 124, 1, B, 2), 1), (G(100_000, 125, 1, B, 1), 7),
]
PILOT = TASKS.pop()

BY_NAME = {t["name"]: t for t in TASKS + [PILOT]}
