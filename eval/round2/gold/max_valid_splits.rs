fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

pub fn max_valid_splits(nums: &[i64]) -> i64 {
    // a split is valid exactly when both sides have the gcd g of the whole array, so the count is
    // (last index whose suffix gcd is g) - (first index whose prefix gcd is g), if positive
    let n = nums.len();
    let mut pg = vec![0i64; n + 1];
    for i in 0..n {
        pg[i + 1] = gcd(pg[i], nums[i]);
    }
    let mut sg = vec![0i64; n + 2];
    for i in (0..n).rev() {
        sg[i] = gcd(sg[i + 1], nums[i]);
    }
    let mut levels = 1;
    while (1 << levels) <= n {
        levels += 1;
    }
    let mut table = vec![0i64; levels * n];
    table[..n].copy_from_slice(nums);
    for j in 1..levels {
        let h = 1 << (j - 1);
        for i in 0..=n - (1 << j) {
            table[j * n + i] = gcd(table[(j - 1) * n + i], table[(j - 1) * n + i + h]);
        }
    }
    let rg = |l: usize, r: usize| {
        let mut k = 0;
        while (2usize << k) <= r - l + 1 {
            k += 1;
        }
        gcd(table[k * n + l], table[k * n + r + 1 - (1 << k)])
    };
    let g = pg[n];
    let first = (0..n).find(|&i| pg[i + 1] == g).unwrap();
    let last = (0..n).rev().find(|&i| sg[i] == g).unwrap();
    let mut best = (last as i64 - first as i64).max(0);
    if n < 3 {
        return best;
    }
    for k in 0..n {
        let gk = gcd(pg[k], sg[k + 1]);
        // first index of the shortened array whose prefix gcd is gk
        // prefix gcds only shrink, so the first one <= gk is the only candidate before k
        let t = pg[1..=n].partition_point(|&x| x > gk);
        let lp = if t < k && pg[t + 1] == gk {
            t
        } else {
            let (mut lo, mut hi) = (k + 1, n - 1);
            while lo < hi {
                let mid = (lo + hi) / 2;
                if gcd(pg[k], rg(k + 1, mid)) == gk { hi = mid } else { lo = mid + 1 }
            }
            lo - 1
        };
        let u = sg[..n].partition_point(|&x| x <= gk);
        let rp = if u > 0 && u - 1 > k && sg[u - 1] == gk {
            u - 2
        } else {
            let (mut lo, mut hi) = (0, k - 1);
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if gcd(rg(mid, k - 1), sg[k + 1]) == gk { lo = mid } else { hi = mid - 1 }
            }
            lo
        };
        best = best.max(rp as i64 - lp as i64);
    }
    best
}
