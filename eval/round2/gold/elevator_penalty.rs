pub fn elevator_penalty(n: i64, start: i64, requests: &[i64]) -> i64 {
    let _ = n;
    let mut pts: Vec<i64> = requests.iter().copied().filter(|&r| r != start).collect();
    pts.sort();
    let m = pts.len();
    if m == 0 {
        return 0;
    }
    // f[l] / g[l]: least cost having fulfilled pts[l..l+len], standing at its left / right end;
    // every second of movement costs one per request still waiting
    const INF: i64 = i64::MAX / 4;
    let s = pts.partition_point(|&p| p < start);
    let mut f = vec![INF; m];
    let mut g = vec![INF; m];
    if s > 0 {
        f[s - 1] = (start - pts[s - 1]) * m as i64;
        g[s - 1] = f[s - 1];
    }
    if s < m {
        f[s] = (pts[s] - start) * m as i64;
        g[s] = f[s];
    }
    for len in 2..=m {
        let left = (m - len + 1) as i64;
        let mut nf = vec![INF; m];
        let mut ng = vec![INF; m];
        for l in 0..=m - len {
            let r = l + len - 1;
            // grow [l+1, r] to the left
            let a = f[l + 1].saturating_add((pts[l + 1] - pts[l]) * left);
            let b = g[l + 1].saturating_add((pts[r] - pts[l]) * left);
            nf[l] = a.min(b);
            // grow [l, r-1] to the right
            let c = g[l].saturating_add((pts[r] - pts[r - 1]) * left);
            let e = f[l].saturating_add((pts[r] - pts[l]) * left);
            ng[l] = c.min(e);
        }
        f = nf;
        g = ng;
    }
    f[0].min(g[0])
}
