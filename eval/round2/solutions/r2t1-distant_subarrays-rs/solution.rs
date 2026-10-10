// A subarray is distant unless goal - k < sum < goal + k, so count the near
// ones with prefix sums and subtract from the total. Prefix sums reach 1e14
// in magnitude and the count reaches n(n+1)/2 ~ 5e9, both fine in i64.

fn count_sum_at_most(prefix: &[i64], sorted: &[i64], bound: i64) -> i64 {
    // pairs i < j with prefix[j] - prefix[i] <= bound,
    // i.e. prefix[i] >= prefix[j] - bound
    let mut fenwick = vec![0i64; sorted.len() + 1];
    let mut count = 0i64;
    for (inserted, &p) in prefix.iter().enumerate() {
        let below = sorted.partition_point(|&v| v < p - bound);
        let mut idx = below;
        let mut smaller = 0i64;
        while idx > 0 {
            smaller += fenwick[idx];
            idx &= idx - 1;
        }
        count += inserted as i64 - smaller;

        let mut idx = sorted.partition_point(|&v| v < p) + 1;
        while idx < fenwick.len() {
            fenwick[idx] += 1;
            idx += idx & idx.wrapping_neg();
        }
    }
    count
}

pub fn distant_subarrays(nums: &[i64], goal: i64, k: i64) -> i64 {
    let n = nums.len() as i64;
    let total = n * (n + 1) / 2;
    if k == 0 {
        return total;
    }

    let mut prefix = Vec::with_capacity(nums.len() + 1);
    prefix.push(0i64);
    for &x in nums {
        let last = *prefix.last().unwrap();
        prefix.push(last + x);
    }
    let mut sorted = prefix.clone();
    sorted.sort_unstable();
    sorted.dedup();

    let near = count_sum_at_most(&prefix, &sorted, goal + k - 1)
        - count_sum_at_most(&prefix, &sorted, goal - k);
    total - near
}
