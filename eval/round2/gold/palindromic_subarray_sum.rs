pub fn palindromic_subarray_sum(nums: &[i64]) -> i64 {
    // every element is positive, so the widest palindrome around each centre has the largest sum
    let n = nums.len();
    let mut pre = vec![0i64; n + 1];
    for i in 0..n {
        pre[i + 1] = pre[i] + nums[i];
    }
    let mut best = 0;
    let (mut l, mut r) = (0i64, -1i64);
    let mut d1 = vec![0i64; n];
    for i in 0..n as i64 {
        let mut k = if i > r { 1 } else { d1[(l + r - i) as usize].min(r - i + 1) };
        while i - k >= 0 && i + k < n as i64 && nums[(i - k) as usize] == nums[(i + k) as usize] {
            k += 1;
        }
        d1[i as usize] = k;
        if i + k - 1 > r {
            l = i - k + 1;
            r = i + k - 1;
        }
        best = best.max(pre[(i + k) as usize] - pre[(i - k + 1) as usize]);
    }
    let (mut l, mut r) = (0i64, -1i64);
    let mut d2 = vec![0i64; n];
    for i in 0..n as i64 {
        let mut k = if i > r { 0 } else { d2[(l + r - i + 1) as usize].min(r - i + 1) };
        while i - k - 1 >= 0 && i + k < n as i64 && nums[(i - k - 1) as usize] == nums[(i + k) as usize] {
            k += 1;
        }
        d2[i as usize] = k;
        if i + k - 1 > r {
            l = i - k;
            r = i + k - 1;
        }
        best = best.max(pre[(i + k) as usize] - pre[(i - k) as usize]);
    }
    best
}
