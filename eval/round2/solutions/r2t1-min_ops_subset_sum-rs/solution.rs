// A value's bits only ever lose their tail (halve) or gain trailing zeros
// (double), so every value reachable from v is (v >> k) << m. The cheapest
// route to it does all halvings first, costing k + m, and never passes a value
// above max(v, target). Each element then offers a small menu of
// (value, cost) pairs, and picking at most one entry per element to hit
// `target` is a grouped 0/1 knapsack.
pub fn min_ops_subset_sum(nums: &[i64], target: i64) -> i64 {
    const UNREACHED: u32 = u32::MAX / 2;
    let t = target as usize;

    let mut fewest = vec![UNREACHED; t + 1];
    fewest[0] = 0;
    let mut before = fewest.clone();
    let mut menu: Vec<(usize, u32)> = Vec::new();

    for &v in nums {
        menu.clear();
        let mut prefix = v as usize;
        let mut halvings = 0u32;
        // A zero prefix adds nothing to a sum, so the element is better left out.
        while prefix > 0 {
            let mut w = prefix;
            let mut ops = halvings;
            while w <= t {
                menu.push((w, ops));
                w <<= 1;
                ops += 1;
            }
            prefix >>= 1;
            halvings += 1;
        }
        menu.sort_unstable();
        menu.dedup_by_key(|&mut (w, _)| w);

        // Every menu value is at least 1 and reads come from the copy taken
        // before this element, so the element is used at most once.
        before.copy_from_slice(&fewest);
        for &(w, ops) in &menu {
            for (dst, &src) in fewest[w..].iter_mut().zip(&before[..=t - w]) {
                let cand = src + ops;
                if cand < *dst {
                    *dst = cand;
                }
            }
        }
    }

    if fewest[t] >= UNREACHED {
        -1
    } else {
        fewest[t] as i64
    }
}
