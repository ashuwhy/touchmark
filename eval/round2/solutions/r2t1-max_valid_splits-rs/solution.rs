// A split is valid exactly when both sides have gcd equal to the gcd G of the whole
// array: if the two side gcds are equal, their gcd is G, so each of them is G. Prefix
// gcds only shrink and suffix gcds only grow, so with p the first index whose prefix
// gcd is G and q the last index whose suffix gcd is G, the valid splits are p..q-1
// and the score is max(0, q - p). For each deleted index the same p and q are found
// by binary search over prefix/suffix gcds and a sparse table of range gcds.

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a as u64, b as u64);
    if a == 0 {
        return b as i64;
    }
    if b == 0 {
        return a as i64;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            return (a << shift) as i64;
        }
    }
}

struct RangeGcd {
    // levels[j][i] = gcd of nums[i .. i + 2^j]
    levels: Vec<Vec<i64>>,
}

impl RangeGcd {
    fn new(nums: &[i64]) -> Self {
        let mut levels = vec![nums.to_vec()];
        let mut width = 1;
        while width * 2 <= nums.len() {
            let prev = &levels[levels.len() - 1];
            let next = (0..=nums.len() - width * 2)
                .map(|i| gcd(prev[i], prev[i + width]))
                .collect();
            levels.push(next);
            width *= 2;
        }
        RangeGcd { levels }
    }

    // gcd of nums[l..=r]; requires l <= r
    fn query(&self, l: usize, r: usize) -> i64 {
        let level = (usize::BITS - 1 - (r - l + 1).leading_zeros()) as usize;
        let row = &self.levels[level];
        gcd(row[l], row[r + 1 - (1 << level)])
    }
}

pub fn max_valid_splits(nums: &[i64]) -> i64 {
    let n = nums.len();
    // pre[i] = gcd(nums[..i]), suf[i] = gcd(nums[i..]); gcd of nothing is 0
    let mut pre = vec![0i64; n + 1];
    for i in 0..n {
        pre[i + 1] = gcd(pre[i], nums[i]);
    }
    let mut suf = vec![0i64; n + 1];
    for i in (0..n).rev() {
        suf[i] = gcd(suf[i + 1], nums[i]);
    }
    let ranges = RangeGcd::new(nums);

    let whole = pre[n];
    let first = pre[1..].partition_point(|&v| v != whole);
    let last = suf[..n].partition_point(|&v| v == whole) - 1;
    let mut best = last as i64 - first as i64;

    // Indices below are positions in the array with nums[k] removed: original index t
    // maps to t when t < k and to t - 1 when t > k.
    for k in 0..n {
        let left = pre[k];
        let right = suf[k + 1];
        let g = gcd(left, right);

        let first = if left == g {
            // k >= 1 here, and pre[k] == g, so the answer lies in 0..k
            pre[1..=k].partition_point(|&v| v != g)
        } else {
            // the prefix reaches g only after the gap: smallest t in k+1..n with
            // gcd(left, nums[k+1..=t]) == g, and t = n - 1 always qualifies
            let (mut lo, mut hi) = (k + 1, n - 1);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if gcd(left, ranges.query(k + 1, mid)) == g {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            lo - 1
        };

        let last = if right == g {
            // k <= n - 2 here; suf[k+1] == g, so at least one t in k+1..n qualifies
            k + suf[k + 1..n].partition_point(|&v| v == g) - 1
        } else {
            // largest t in 0..k with gcd(nums[t..k], right) == g; t = 0 always qualifies
            let (mut lo, mut hi) = (0, k - 1);
            while lo < hi {
                let mid = lo + (hi - lo + 1) / 2;
                if gcd(ranges.query(mid, k - 1), right) == g {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            lo
        };

        best = best.max(last as i64 - first as i64);
    }
    best.max(0)
}
