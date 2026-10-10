pub fn min_ops_subset_sum(nums: &[i64], target: i64) -> i64 {
    // an element x can become (x >> k) << m at cost k + m and nothing else is cheaper
    const INF: i64 = i64::MAX / 2;
    let t = target as usize;
    let mut dp = vec![INF; t + 1];
    dp[0] = 0;
    for &x in nums {
        let mut cost = vec![INF; t + 1];
        let mut k = 0;
        while (x >> k) > 0 {
            let mut y = x >> k;
            let mut m = 0;
            while y <= target {
                cost[y as usize] = cost[y as usize].min(k + m);
                y <<= 1;
                m += 1;
            }
            k += 1;
        }
        let opts: Vec<(usize, i64)> = (1..=t).filter(|&y| cost[y] < INF).map(|y| (y, cost[y])).collect();
        let mut next = dp.clone();
        for &(y, c) in &opts {
            for s in y..=t {
                if dp[s - y] + c < next[s] {
                    next[s] = dp[s - y] + c;
                }
            }
        }
        dp = next;
    }
    if dp[t] >= INF { -1 } else { dp[t] }
}
