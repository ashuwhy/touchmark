pub fn distant_subarrays(nums: &[i64], goal: i64, k: i64) -> i64 {
    // |p[j] - p[i] - goal| >= k: count pairs i < j of prefix sums with
    // p[i] <= p[j] - goal - k, or p[i] >= p[j] - goal + k (the two never overlap when k > 0)
    let n = nums.len();
    if k == 0 {
        return (n * (n + 1) / 2) as i64;
    }
    let mut p = vec![0i64];
    for &x in nums {
        let last = *p.last().unwrap();
        p.push(last + x);
    }
    let len = p.len();
    let mut buf = vec![0i64; len];
    let mut count = 0i64;
    let mut width = 1;
    while width < len {
        let mut lo = 0;
        while lo + width < len {
            let mid = lo + width;
            let hi = (lo + 2 * width).min(len);
            let (mut a, mut b) = (lo, lo);
            for j in mid..hi {
                while a < mid && p[a] <= p[j] - goal - k {
                    a += 1;
                }
                while b < mid && p[b] < p[j] - goal + k {
                    b += 1;
                }
                count += (a - lo) as i64 + (mid - b) as i64;
            }
            let (mut x, mut y, mut t) = (lo, mid, lo);
            while t < hi {
                if y >= hi || (x < mid && p[x] <= p[y]) {
                    buf[t] = p[x];
                    x += 1;
                } else {
                    buf[t] = p[y];
                    y += 1;
                }
                t += 1;
            }
            p[lo..hi].copy_from_slice(&buf[lo..hi]);
            lo = hi;
        }
        width *= 2;
    }
    count
}
