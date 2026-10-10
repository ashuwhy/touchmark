pub fn min_max_waiting_time(demand: &[i64], fuel: &[i64]) -> i64 {
    // state before car i: fuel left in dispenser 0, which dispenser took car i-1, and how long
    // the other one stays busy; car i-1 has just started, so its own dispenser is busy demand[i-1]
    const INF: i64 = i64::MAX;
    let n = demand.len();
    let idx = |f0: i64, last: usize, other: i64| ((f0 as usize) * 2 + last) * 21 + other as usize;
    let mut cur = vec![INF; 51 * 2 * 21];
    cur[idx(fuel[0], 0, 0)] = 0;
    let mut used = 0;
    let mut best_count = 0;
    let mut best_wait = 0;
    for i in 0..n {
        let d = demand[i];
        let prev = if i == 0 { 0 } else { demand[i - 1] };
        let mut next = vec![INF; 51 * 2 * 21];
        let mut any = false;
        for f0 in 0..=50i64 {
            let f1 = fuel[0] + fuel[1] - used - f0;
            if f1 < 0 || f1 > 50 {
                continue;
            }
            for last in 0..2 {
                for other in 0..=20i64 {
                    let w = cur[idx(f0, last, other)];
                    if w == INF {
                        continue;
                    }
                    let mut stuck = true;
                    for j in 0..2 {
                        let f = if j == 0 { f0 } else { f1 };
                        if f < d {
                            continue;
                        }
                        stuck = false;
                        let (wait, rest) = if j == last { (prev, (other - prev).max(0)) } else { (other, (prev - other).max(0)) };
                        let nf0 = if j == 0 { f0 - d } else { f0 };
                        let k = idx(nf0, j, rest);
                        next[k] = next[k].min(w.max(wait));
                        any = true;
                    }
                    if stuck {
                        if i > best_count || (i == best_count && w < best_wait) {
                            best_count = i;
                            best_wait = w;
                        }
                    }
                }
            }
        }
        if !any {
            break;
        }
        used += d;
        cur = next;
        if i + 1 == n {
            for &w in &cur {
                if w != INF && (n > best_count || w < best_wait) {
                    best_count = n;
                    best_wait = w;
                }
            }
        }
    }
    if best_count == 0 { -1 } else { best_wait }
}
