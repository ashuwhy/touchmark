// Prefix sums P[0..=n], subarray (l, r] has sum P[r] - P[l]. Negating a_i
// with l < i <= r makes it P[r] - P[l] - 2*a_i, so the subarray is valid when
// P[l] + 2*a_i ≡ P[r] (mod k) for some such i, or P[l] ≡ P[r].
//
// For a fixed residue of P[l], its first occurrence dominates every later l:
// it is smaller and leaves at least as many choices of i. So only the first
// occurrence of each residue is a candidate left end (at most k of them).
//
// A pair (prefix residue c, d = 2*a_i mod k) becomes usable at the first i
// past first[c] with 2*a_i ≡ d, and stays usable. best[c + d] keeps the
// smallest first[c] over usable pairs. When d shows up at step i, the pairs
// that turn on are exactly those whose first[c] lies in [last[d], i - 1], a
// suffix of the residues in order of appearance, so each pair is visited once:
// at most k * k = 9e6 updates, independent of n.
pub fn longest_negation_subarray(nums: &[i64], k: i64) -> i64 {
    const UNSEEN: usize = usize::MAX;
    let m = k as usize;

    let mut first = vec![UNSEEN; m];
    let mut appeared: Vec<usize> = Vec::with_capacity(m);
    let mut best = vec![UNSEEN; m];
    // 0 doubles as "never seen": element positions start at 1, and every
    // first[c] >= 0, so a missing d turns on every pair as it should.
    let mut last = vec![0usize; m];

    first[0] = 0;
    appeared.push(0);
    let mut prefix = 0usize;
    let mut longest = 0usize;

    for (pos, &x) in nums.iter().enumerate() {
        let r = pos + 1;
        let d = (2 * x).rem_euclid(k) as usize;
        let since = last[d];
        for &c in appeared.iter().rev() {
            let l = first[c];
            if l < since {
                break;
            }
            let target = if c + d >= m { c + d - m } else { c + d };
            if l < best[target] {
                best[target] = l;
            }
        }
        last[d] = r;

        prefix = (prefix + x.rem_euclid(k) as usize) % m;
        if first[prefix] == UNSEEN {
            first[prefix] = r;
            appeared.push(prefix);
        } else {
            longest = longest.max(r - first[prefix]);
        }
        if best[prefix] != UNSEEN {
            longest = longest.max(r - best[prefix]);
        }
    }

    longest as i64
}
