pub fn max_valid_pair_sum(nums: &[i64], k: i64) -> i64 {
    let k = k as usize;
    let mut lead = 0;
    let mut best = 0;
    for j in k..nums.len() {
        lead = lead.max(nums[j - k]);
        best = best.max(lead + nums[j]);
    }
    best
}
