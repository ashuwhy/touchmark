// x/y <= a/b with y > 0  <=>  b*x - a*y <= 0. An all-even subarray gives b*x > 0,
// so the y > 0 requirement falls out of the inequality for free. Weight evens +b and
// odds -a; a subarray (i, j] is valid iff prefix[i] >= prefix[j]. |prefix| <= 1e5 * 1e9.
pub fn count_ratio_subarrays(nums: &[i64], a: i64, b: i64) -> i64 {
    let mut prefix = Vec::with_capacity(nums.len() + 1);
    let mut running = 0i64;
    prefix.push(running);
    for &v in nums {
        running += if v % 2 == 0 { b } else { -a };
        prefix.push(running);
    }

    let mut ranks = prefix.clone();
    ranks.sort_unstable();
    ranks.dedup();

    // Fenwick tree over ranks, 1-based.
    let mut fenwick = vec![0i64; ranks.len() + 1];
    let mut seen = 0i64;
    let mut valid = 0i64;
    for &p in &prefix {
        let rank = ranks.partition_point(|&r| r < p);
        let mut below = 0i64;
        let mut k = rank;
        while k > 0 {
            below += fenwick[k];
            k &= k - 1;
        }
        valid += seen - below;

        let mut k = rank + 1;
        while k < fenwick.len() {
            fenwick[k] += 1;
            k += k & k.wrapping_neg();
        }
        seen += 1;
    }
    valid
}
