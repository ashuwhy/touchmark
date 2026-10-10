// Interval DP over the sorted request floors plus the start floor. The set of
// floors already passed is always a contiguous run around `start`, and the
// elevator sits at one end of it. Moving d floors adds d to the cost of every
// request not yet fulfilled, so a state only needs (run, which end, cost).
// O(k^2) time and O(k) memory for k <= 1501 floors.
pub fn elevator_penalty(n: i64, start: i64, requests: &[i64]) -> i64 {
    debug_assert!(0 <= start && start < n);
    let mut floors: Vec<i64> = requests.to_vec();
    floors.sort_unstable();
    let start_requested = floors.binary_search(&start).is_ok();
    if !start_requested {
        let at = floors.partition_point(|&f| f < start);
        floors.insert(at, start);
    }
    let k = floors.len();
    let s = floors.binary_search(&start).unwrap();
    let total = requests.len() as i64;

    // prefix[i] = number of requests among floors[0..i]
    let mut prefix = vec![0i64; k + 1];
    for i in 0..k {
        let weight = if i == s && !start_requested { 0 } else { 1 };
        prefix[i + 1] = prefix[i] + weight;
    }
    let pending = |l: usize, r: usize| total - (prefix[r + 1] - prefix[l]);

    const INF: i64 = i64::MAX / 4;
    let mut at_left = vec![INF; k];
    let mut at_right = vec![INF; k];
    at_left[s] = 0;
    at_right[s] = 0;
    let mut next_left = vec![INF; k];
    let mut next_right = vec![INF; k];

    for len in 2..=k {
        let lo = (s + 1).saturating_sub(len);
        let hi = s.min(k - len);
        for l in lo..=hi {
            let r = l + len - 1;

            let mut best = INF;
            if l < s {
                let waiting = pending(l + 1, r);
                best = best
                    .min(at_left[l + 1] + (floors[l + 1] - floors[l]) * waiting)
                    .min(at_right[l + 1] + (floors[r] - floors[l]) * waiting);
            }
            next_left[l] = best;

            let mut best = INF;
            if r > s {
                let waiting = pending(l, r - 1);
                best = best
                    .min(at_right[l] + (floors[r] - floors[r - 1]) * waiting)
                    .min(at_left[l] + (floors[r] - floors[l]) * waiting);
            }
            next_right[l] = best;
        }
        std::mem::swap(&mut at_left, &mut next_left);
        std::mem::swap(&mut at_right, &mut next_right);
    }

    at_left[0].min(at_right[0])
}
