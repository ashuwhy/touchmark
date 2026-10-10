pub fn widest_fence(planks: &[i64]) -> i64 {
    let mut v = planks.to_vec();
    v.sort();
    let mut vals: Vec<i64> = Vec::new();
    let mut cnt: Vec<i64> = Vec::new();
    for &x in &v {
        if vals.last() == Some(&x) {
            *cnt.last_mut().unwrap() += 1;
        } else {
            vals.push(x);
            cnt.push(1);
        }
    }
    let d = vals.len();
    // (height, pieces made of two planks) for every pair of values
    let mut pairs: Vec<(i64, i64)> = Vec::new();
    for i in 0..d {
        pairs.push((2 * vals[i], cnt[i] / 2));
        for j in i + 1..d {
            pairs.push((vals[i] + vals[j], cnt[i].min(cnt[j])));
        }
    }
    pairs.sort();
    let mut best = *cnt.iter().max().unwrap();
    let mut i = 0;
    while i < pairs.len() {
        let h = pairs[i].0;
        let mut total = 0;
        while i < pairs.len() && pairs[i].0 == h {
            total += pairs[i].1;
            i += 1;
        }
        if let Ok(p) = vals.binary_search(&h) {
            total += cnt[p];
        }
        best = best.max(total);
    }
    best
}
