pub fn count_ratio_subarrays(nums: &[i64], a: i64, b: i64) -> i64 {
    // x / y <= a / b with y > 0 is x * b - y * a <= 0; an even element adds b, an odd one -a,
    // so count pairs of prefix sums i < j with p[i] >= p[j]
    let mut p = vec![0i64];
    for &x in nums {
        let last = *p.last().unwrap();
        p.push(last + if x % 2 == 0 { b } else { -a });
    }
    let n = p.len();
    let mut buf = vec![0i64; n];
    let mut count = 0i64;
    let mut width = 1;
    while width < n {
        let mut lo = 0;
        while lo + width < n {
            let mid = lo + width;
            let hi = (lo + 2 * width).min(n);
            // both halves ascending: for each right element, left elements >= it
            let mut i = lo;
            for j in mid..hi {
                while i < mid && p[i] < p[j] {
                    i += 1;
                }
                count += (mid - i) as i64;
            }
            let (mut x, mut y, mut k) = (lo, mid, lo);
            while k < hi {
                if y >= hi || (x < mid && p[x] <= p[y]) {
                    buf[k] = p[x];
                    x += 1;
                } else {
                    buf[k] = p[y];
                    y += 1;
                }
                k += 1;
            }
            p[lo..hi].copy_from_slice(&buf[lo..hi]);
            lo = hi;
        }
        width *= 2;
    }
    count
}
