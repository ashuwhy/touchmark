pub fn largest_power(nums: &[i64]) -> Vec<i64> {
    // greedy: while some value has every bit of the mask, put all of them next; otherwise the
    // next value must be the one keeping the largest part of the mask
    let mut left: Vec<i64> = nums.to_vec();
    let mut order: Vec<i64> = Vec::new();
    let mut mask = (1 << 15) - 1;
    while !left.is_empty() {
        let (take, rest): (Vec<i64>, Vec<i64>) = left.iter().partition(|&&v| v & mask == mask);
        if !take.is_empty() {
            order.extend(take);
            left = rest;
        } else {
            mask &= left.iter().copied().max_by_key(|&v| v & mask).unwrap();
        }
    }
    (0..15)
        .map(|i| {
            let bit = 14 - i;
            order.iter().take_while(|&&v| v >> bit & 1 == 1).count() as i64
        })
        .collect()
}
