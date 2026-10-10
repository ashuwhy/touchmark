// A plank of height H can never sit in a pair summing to H (the partner would
// need height 0), so for a fixed H the singles are exactly the planks of height
// H, and the pairs are independent per value class: values v and H - v give
// min(cnt[v], cnt[H - v]) pairs, and v = H / 2 gives cnt[v] / 2.
pub fn widest_fence(planks: &[i64]) -> i64 {
    let mut sorted = planks.to_vec();
    sorted.sort_unstable();

    let mut groups: Vec<(i64, i64)> = Vec::new();
    for &h in &sorted {
        match groups.last_mut() {
            Some((v, c)) if *v == h => *c += 1,
            _ => groups.push((h, 1)),
        }
    }

    // (height, pieces contributed to that height)
    let mut gains: Vec<(i64, i64)> = Vec::with_capacity(groups.len() * (groups.len() + 3) / 2);
    for (i, &(vi, ci)) in groups.iter().enumerate() {
        gains.push((vi, ci));
        if ci >= 2 {
            gains.push((vi + vi, ci / 2));
        }
        for &(vj, cj) in &groups[i + 1..] {
            gains.push((vi + vj, ci.min(cj)));
        }
    }
    gains.sort_unstable_by_key(|&(h, _)| h);

    let mut best = 0;
    let mut start = 0;
    while start < gains.len() {
        let height = gains[start].0;
        let mut pieces = 0;
        let mut end = start;
        while end < gains.len() && gains[end].0 == height {
            pieces += gains[end].1;
            end += 1;
        }
        best = best.max(pieces);
        start = end;
    }
    best
}
