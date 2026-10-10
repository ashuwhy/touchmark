pub fn longest_negation_subarray(nums: &[i64], k: i64) -> i64 {
    // prefix residues: the subarray (l, r] works without negation when res[l] == res[r], and by
    // negating nums[m] (l <= m < r) when res[r] == res[l] + 2 * nums[m] (mod k). For a pair of
    // residues the widest choice is the first l and the last r, so try every pair
    let n = nums.len();
    let ku = k as usize;
    let mut first = vec![usize::MAX; ku];
    let mut last = vec![0usize; ku];
    let mut p = 0i64;
    for t in 0..=n {
        if t > 0 {
            p = (p + nums[t - 1]).rem_euclid(k);
        }
        let r = p as usize;
        if first[r] == usize::MAX {
            first[r] = t;
        }
        last[r] = t;
    }
    let mut best = 0usize;
    for v in 0..ku {
        if first[v] != usize::MAX {
            best = best.max(last[v] - first[v]);
        }
    }
    let q: Vec<usize> = nums.iter().map(|&x| (2 * x).rem_euclid(k) as usize).collect();
    let mut starts: Vec<(usize, usize)> = (0..ku).filter(|&v| first[v] != usize::MAX).map(|v| (first[v], v)).collect();
    starts.sort();
    // nxt[d]: smallest m >= the current start with q[m] == d
    let mut nxt = vec![usize::MAX; ku];
    let mut pos = n;
    for &(f, v) in starts.iter().rev() {
        while pos > f {
            pos -= 1;
            nxt[q[pos]] = pos;
        }
        for d in 0..ku {
            let w = (v + d) % ku;
            if first[w] != usize::MAX && nxt[d] != usize::MAX && nxt[d] < last[w] {
                best = best.max(last[w] - f);
            }
        }
    }
    best as i64
}
