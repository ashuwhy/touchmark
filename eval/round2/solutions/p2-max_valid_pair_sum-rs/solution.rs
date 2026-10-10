/// Largest nums[i] + nums[j] with i < j and j - i >= k.
///
/// Walks j from k upward, keeping the max of nums[0..=j-k] as the best
/// partner for j. Sums fit in i64: each element is at most 1e9.
pub fn max_valid_pair_sum(nums: &[i64], k: i64) -> i64 {
    let gap = k as usize;
    let mut best_left = i64::MIN;
    let mut best = i64::MIN;
    for j in gap..nums.len() {
        best_left = best_left.max(nums[j - gap]);
        best = best.max(best_left + nums[j]);
    }
    best
}
