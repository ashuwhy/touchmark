"""Reference solutions (fast) and brute-force definitions (slow) for the round 2 tasks.
Every fast reference is checked against its brute force by check_refs.py."""

import bisect, functools, itertools, math
from collections import Counter

import numpy as np


# palindromic_subarray_sum

def pal_sum(a):
    n = len(a)
    pre = [0] * (n + 1)
    for i, x in enumerate(a):
        pre[i + 1] = pre[i] + x
    best = max(a)
    d1 = [0] * n
    l, r = 0, -1
    for i in range(n):
        k = 1 if i > r else min(d1[l + r - i], r - i + 1)
        while i - k >= 0 and i + k < n and a[i - k] == a[i + k]:
            k += 1
        d1[i] = k
        if i + k - 1 > r:
            l, r = i - k + 1, i + k - 1
        best = max(best, pre[i + k] - pre[i - k + 1])
    d2 = [0] * n
    l, r = 0, -1
    for i in range(n):
        k = 0 if i > r else min(d2[l + r - i + 1], r - i + 1)
        while i - k - 1 >= 0 and i + k < n and a[i - k - 1] == a[i + k]:
            k += 1
        d2[i] = k
        if i + k - 1 > r:
            l, r = i - k, i + k - 1
        if k:
            best = max(best, pre[i + k] - pre[i - k])
    return best


def pal_sum_brute(a):
    n = len(a)
    return max(sum(a[l:r + 1]) for l in range(n) for r in range(l, n) if a[l:r + 1] == a[l:r + 1][::-1])


# widest_fence

def fence(p):
    c = Counter(p)
    vals = sorted(c)
    acc = Counter()
    for x in vals:
        acc[2 * x] += c[x] // 2
    for i in range(len(vals)):
        ci = c[vals[i]]
        for j in range(i + 1, len(vals)):
            acc[vals[i] + vals[j]] += min(ci, c[vals[j]])
    best = max(c.values())
    for s, v in acc.items():
        best = max(best, v + c.get(s, 0))
    return best


def fence_brute(p):
    n = len(p)
    heights = set(p) | {p[i] + p[j] for i in range(n) for j in range(i + 1, n)}

    def most(h, used, i):
        while i < n and used >> i & 1:
            i += 1
        if i == n:
            return 0
        used |= 1 << i
        best = most(h, used, i + 1)
        if p[i] == h:
            best = max(best, 1 + most(h, used, i + 1))
        for j in range(i + 1, n):
            if not used >> j & 1 and p[i] + p[j] == h:
                best = max(best, 1 + most(h, used | 1 << j, i + 1))
        return best

    return max(most(h, 0, 0) for h in heights)


# min_max_waiting_time

def waiting(demand, fuel):
    n = len(demand)

    @functools.lru_cache(maxsize=None)
    def best(i, f0, f1, r0, r1):
        # (cars served from i on, smallest possible largest wait among them), more cars first
        if i == n:
            return (0, 0)
        d = demand[i]
        out = None
        for j in (0, 1):
            f, rj = (f0, r0) if j == 0 else (f1, r1)
            if f < d:
                continue
            w = rj
            if j == 0:
                c, m = best(i + 1, f0 - d, f1, d, max(0, r1 - w))
            else:
                c, m = best(i + 1, f0, f1 - d, max(0, r0 - w), d)
            cand = (c + 1, max(w, m))
            if out is None or cand[0] > out[0] or (cand[0] == out[0] and cand[1] < out[1]):
                out = cand
        return out if out is not None else (0, 0)

    c, m = best(0, fuel[0], fuel[1], 0, 0)
    return -1 if c == 0 else m


def waiting_brute(demand, fuel):
    outcomes = []

    def go(i, fuel_left, free_at, now, waits):
        if i == len(demand) or all(fuel_left[j] < demand[i] for j in (0, 1)):
            outcomes.append((i, max(waits) if waits else 0))
            return
        for j in (0, 1):
            if fuel_left[j] >= demand[i]:
                start = max(now, free_at[j])
                fl = list(fuel_left)
                fl[j] -= demand[i]
                fa = list(free_at)
                fa[j] = start + demand[i]
                go(i + 1, fl, fa, start, waits + [start - now])

    go(0, list(fuel), [0, 0], 0, [])
    served = max(c for c, _ in outcomes)
    if served == 0:
        return -1
    return min(w for c, w in outcomes if c == served)


# count_ratio_subarrays

class Fenwick:
    def __init__(self, n):
        self.t = [0] * (n + 1)

    def add(self, i, v=1):
        i += 1
        while i < len(self.t):
            self.t[i] += v
            i += i & -i

    def prefix(self, i):
        """Sum of entries 0..i-1."""
        s = 0
        while i > 0:
            s += self.t[i]
            i -= i & -i
        return s


def ratio(nums, a, b):
    pre = [0]
    for x in nums:
        pre.append(pre[-1] + (b if x % 2 == 0 else -a))
    keys = sorted(set(pre))
    fw = Fenwick(len(keys))
    count = 0
    for k, p in enumerate(pre):
        pos = bisect.bisect_left(keys, p)
        count += k - fw.prefix(pos)  # earlier prefixes >= p
        fw.add(pos)
    return count


def ratio_brute(nums, a, b):
    n = len(nums)
    count = 0
    for i in range(n):
        x = y = 0
        for j in range(i, n):
            if nums[j] % 2 == 0:
                x += 1
            else:
                y += 1
            if y > 0 and x * b <= a * y:
                count += 1
    return count


# elevator_penalty

def elevator(n, start, requests):
    pts = sorted(r for r in requests if r != start)
    m = len(pts)
    if m == 0:
        return 0
    s = bisect.bisect_left(pts, start)
    INF = float("inf")
    # f[l][side] for intervals of the current length: covered pts[l..r], standing at pts[l] (0) or pts[r] (1)
    cur = {}
    if s > 0:
        cur[(s - 1, s - 1)] = [(start - pts[s - 1]) * m, INF]
    if s < m:
        cur[(s, s)] = [INF, (pts[s] - start) * m]
    # intervals of length L always contain s-1 or s; grow them one point at a time
    for length in range(1, m):
        nxt = {}
        for (l, r), (at_l, at_r) in cur.items():
            left = m - (r - l + 1)
            for pos, cost in ((pts[l], at_l), (pts[r], at_r)):
                if cost == INF:
                    continue
                if l > 0:
                    c = cost + (pos - pts[l - 1]) * left
                    e = nxt.setdefault((l - 1, r), [INF, INF])
                    e[0] = min(e[0], c)
                if r < m - 1:
                    c = cost + (pts[r + 1] - pos) * left
                    e = nxt.setdefault((l, r + 1), [INF, INF])
                    e[1] = min(e[1], c)
        cur = nxt
    return min(min(v) for v in cur.values())


def elevator_brute(n, start, requests):
    targets = [r for r in requests if r != start]
    best = None
    for order in itertools.permutations(targets):
        t, pos, done, total = 0, start, set(), 0
        for goal in order:
            if goal in done:
                continue
            step = 1 if goal > pos else -1
            while pos != goal:
                pos += step
                t += 1
                if pos in targets and pos not in done:
                    done.add(pos)
                    total += t
        best = total if best is None else min(best, total)
    return best if best is not None else 0


# max_valid_splits

def splits_count(arr):
    m = len(arr)
    if m < 2:
        return 0
    g = 0
    for x in arr:
        g = math.gcd(g, x)
    p = 0
    L = None
    for i, x in enumerate(arr):
        p = math.gcd(p, x)
        if p == g:
            L = i
            break
    s = 0
    R = None
    for i in range(m - 1, -1, -1):
        s = math.gcd(s, arr[i])
        if s == g:
            R = i
            break
    return max(0, R - L)


def splits(nums):
    n = len(nums)
    pg = [0] * (n + 1)  # pg[i] = gcd(nums[0..i-1])
    for i, x in enumerate(nums):
        pg[i + 1] = math.gcd(pg[i], x)
    sg = [0] * (n + 2)  # sg[i] = gcd(nums[i..n-1])
    for i in range(n - 1, -1, -1):
        sg[i] = math.gcd(sg[i + 1], nums[i])
    # sparse table for range gcd
    table = [list(nums)]
    j = 1
    while (1 << j) <= n:
        prev = table[-1]
        h = 1 << (j - 1)
        table.append([math.gcd(prev[i], prev[i + h]) for i in range(n - (1 << j) + 1)])
        j += 1

    def rg(l, r):  # gcd(nums[l..r]), l <= r
        k = (r - l + 1).bit_length() - 1
        return math.gcd(table[k][l], table[k][r - (1 << k) + 1])

    first_pg = {}
    for i in range(1, n + 1):
        first_pg.setdefault(pg[i], i - 1)  # earliest index i-1 whose prefix gcd is pg[i]
    last_sg = {}
    for i in range(n - 1, -1, -1):
        last_sg.setdefault(sg[i], i)
    best = splits_count(nums)
    for k in range(n):
        if n - 1 < 2:
            break
        G = math.gcd(pg[k], sg[k + 1])
        # L': first index of the new array whose prefix gcd is G
        if k > 0 and first_pg.get(G, n) < k:
            Lp = first_pg[G]
        else:
            lo, hi = k + 1, n - 1  # smallest t with gcd(pg[k], rg(k+1, t)) == G; t = n-1 works
            while lo < hi:
                mid = (lo + hi) // 2
                if math.gcd(pg[k], rg(k + 1, mid)) == G:
                    hi = mid
                else:
                    lo = mid + 1
            Lp = lo - 1
        # R': last index of the new array whose suffix gcd is G
        if k < n - 1 and last_sg.get(G, -1) > k:
            Rp = last_sg[G] - 1
        else:
            lo, hi = 0, k - 1  # largest t with gcd(rg(t, k-1), sg[k+1]) == G; t = 0 works
            while lo < hi:
                mid = (lo + hi + 1) // 2
                if math.gcd(rg(mid, k - 1), sg[k + 1]) == G:
                    lo = mid
                else:
                    hi = mid - 1
            Rp = lo
        best = max(best, Rp - Lp)
    return best


def splits_brute(nums):
    best = 0
    cands = [list(nums)] + [nums[:k] + nums[k + 1:] for k in range(len(nums))]
    for arr in cands:
        c = 0
        for i in range(len(arr) - 1):
            if functools.reduce(math.gcd, arr[:i + 1]) == functools.reduce(math.gcd, arr[i + 1:]):
                c += 1
        best = max(best, c)
    return best


# min_ops_subset_sum

def subset_ops(nums, total):
    INF = 10**9
    dp = np.full(total + 1, INF, dtype=np.int64)
    dp[0] = 0
    for x in nums:
        cost = {}
        k = 0
        while (x >> k) > 0:
            base = x >> k
            m = 0
            while (base << m) <= total:
                y = base << m
                cost[y] = min(cost.get(y, INF), k + m)
                m += 1
            k += 1
        new = dp.copy()
        for y, c in cost.items():
            np.minimum(new[y:], dp[:total + 1 - y] + c, out=new[y:])
        dp = new
    return -1 if dp[total] >= INF else int(dp[total])


def subset_ops_brute(nums, total):
    bound = 4 * max(total, max(nums)) + 4
    tables = []
    for x in nums:
        dist = {x: 0}
        frontier = [x]
        while frontier:
            nxt = []
            for v in frontier:
                for w in (2 * v, v // 2):
                    if w <= bound and w not in dist:
                        dist[w] = dist[v] + 1
                        nxt.append(w)
            frontier = nxt
        tables.append(dist)
    best = None
    reach = {0: 0}
    for dist in tables:
        new = dict(reach)
        for s, c in reach.items():
            for v, d in dist.items():
                if v > 0 and s + v <= total:
                    if s + v not in new or new[s + v] > c + d:
                        new[s + v] = c + d
        reach = new
    best = reach.get(total)
    return -1 if best is None else best


# distant_subarrays

def distant(nums, goal, k):
    n = len(nums)
    if k == 0:
        return n * (n + 1) // 2
    pre = [0]
    for x in nums:
        pre.append(pre[-1] + x)
    keys = sorted(set(pre))
    fw = Fenwick(len(keys))
    count = 0
    hi, lo = goal + k, goal - k
    for j, p in enumerate(pre):
        # earlier P[i] <= p - hi
        count += fw.prefix(bisect.bisect_right(keys, p - hi))
        # earlier P[i] >= p - lo
        count += j - fw.prefix(bisect.bisect_left(keys, p - lo))
        fw.add(bisect.bisect_left(keys, p))
    return count


def distant_brute(nums, goal, k):
    n = len(nums)
    c = 0
    for i in range(n):
        s = 0
        for j in range(i, n):
            s += nums[j]
            if abs(s - goal) >= k:
                c += 1
    return c


# shadow_pairs

def shadow(nums):
    a = np.array(nums, dtype=np.int64)
    n = len(a)
    total = 0
    for j in range(1, n):
        prev = a[:j]
        below = prev < a[j]
        vals = np.where(below, prev, -1)
        suf = np.maximum.accumulate(vals[::-1])[::-1]
        after = np.empty(j, dtype=np.int64)
        after[:-1] = suf[1:]
        after[-1] = -1
        total += int(np.count_nonzero(below & (prev >= after)))
    return total


def shadow_brute(nums):
    n = len(nums)
    c = 0
    for i in range(n):
        for j in range(i + 1, n):
            if nums[i] < nums[j] and not any(nums[i] < nums[k] < nums[j] for k in range(i + 1, j)):
                c += 1
    return c


# largest_power

def power_of(perm):
    out = []
    for i in range(15):
        b = 14 - i
        j = 0
        while j < len(perm) and perm[j] >> b & 1:
            j += 1
        out.append(j)
    return out


def largest_power(nums):
    left = list(nums)
    perm = []
    mask = (1 << 15) - 1
    while left:
        take = [v for v in left if v & mask == mask]
        if take:
            perm += take
            left = [v for v in left if v & mask != mask]
            continue
        v = max(left, key=lambda v: v & mask)
        mask &= v
    return power_of(perm)


def largest_power_brute(nums):
    return max(power_of(list(p)) for p in itertools.permutations(nums))


# longest_negation_subarray

def negation(nums, k):
    n = len(nums)
    pre = [0]
    for x in nums:
        pre.append(pre[-1] + x)
    res = [p % k for p in pre]
    first = np.full(k, -1, dtype=np.int64)
    last = np.full(k, -1, dtype=np.int64)
    for t, r in enumerate(res):
        if first[r] < 0:
            first[r] = t
        last[r] = t
    best = 0
    for v in range(k):
        if first[v] >= 0 and last[v] > first[v]:
            best = max(best, int(last[v] - first[v]))
    q = [(2 * x) % k for x in nums]
    starts = sorted((int(first[v]), v) for v in range(k) if first[v] >= 0)
    nxt = np.full(k, n, dtype=np.int64)  # nxt[d] = smallest m >= pos with q[m] == d, n if none
    pos = n
    shift = np.arange(k)
    for f, v in reversed(starts):
        while pos > f:
            pos -= 1
            if pos < n:
                nxt[q[pos]] = pos
        w = (v + shift) % k
        lw = last[w]
        ok = (lw >= 0) & (nxt <= lw - 1)
        if ok.any():
            best = max(best, int((lw[ok] - f).max()))
    return best


def negation_brute(nums, k):
    n = len(nums)
    best = 0
    for l in range(n):
        for r in range(l, n):
            s = sum(nums[l:r + 1])
            if s % k == 0 or any((s - 2 * x) % k == 0 for x in nums[l:r + 1]):
                best = max(best, r - l + 1)
    return best


# max_valid_pair_sum (pilot)

def pair_sum(nums, k):
    lead = 0
    best = 0
    for j in range(k, len(nums)):
        lead = max(lead, nums[j - k])
        best = max(best, lead + nums[j])
    return best


def pair_sum_brute(nums, k):
    return max(nums[i] + nums[j] for i in range(len(nums)) for j in range(i + k, len(nums)))
