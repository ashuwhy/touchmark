pub fn shadow_pairs(nums: &[i64]) -> i64 {
    // c[i] is the smallest value above nums[i] seen after i (INF if none yet); j pairs with i when
    // nums[i] < nums[j] <= c[i], and then every i with nums[i] < nums[j] gets c[i] = min(c[i], nums[j]).
    // Positions are kept in value order in blocks that know their largest c, the strictly
    // second largest and how many share the largest, so a block whose second largest is below
    // nums[j] is updated whole (blocks are rebuilt only when two of their values merge).
    const INF: i64 = i64::MAX;
    const OFF: i64 = -1; // not yet seen
    const LOW: i64 = -2;
    let n = nums.len();
    let mut ord: Vec<usize> = (0..n).collect();
    ord.sort_by_key(|&i| (nums[i], i));
    let mut rank = vec![0usize; n];
    for (r, &i) in ord.iter().enumerate() {
        rank[i] = r;
    }
    let sorted: Vec<i64> = ord.iter().map(|&i| nums[i]).collect();
    let s = 256;
    let blocks = (n + s - 1) / s;
    let mut c = vec![OFF; n];
    let mut cap = vec![INF; blocks];
    let mut mx = vec![OFF; blocks];
    let mut se = vec![LOW; blocks];
    let mut cm = vec![0i64; blocks];
    for b in 0..blocks {
        cm[b] = ((b + 1) * s).min(n) as i64 - (b * s) as i64;
    }
    let rebuild = |b: usize, c: &mut Vec<i64>, cap: &mut Vec<i64>, mx: &mut Vec<i64>, se: &mut Vec<i64>, cm: &mut Vec<i64>| {
        let (lo, hi) = (b * s, ((b + 1) * s).min(n));
        let (mut m1, mut m2, mut k) = (LOW, LOW, 0);
        for x in lo..hi {
            c[x] = c[x].min(cap[b]);
            if c[x] > m1 {
                m2 = m1;
                m1 = c[x];
                k = 1;
            } else if c[x] == m1 {
                k += 1;
            } else if c[x] > m2 {
                m2 = c[x];
            }
        }
        cap[b] = INF;
        mx[b] = m1;
        se[b] = m2;
        cm[b] = k;
    };
    let mut count = 0i64;
    for j in 0..n {
        let v = nums[j];
        let lim = sorted.partition_point(|&x| x < v);
        let mut b = 0;
        while b * s < lim {
            let hi = ((b + 1) * s).min(n);
            if hi <= lim && mx[b] < v {
            } else if hi <= lim && se[b] < v {
                count += cm[b];
                cap[b] = v;
                mx[b] = v;
            } else {
                rebuild(b, &mut c, &mut cap, &mut mx, &mut se, &mut cm);
                for x in b * s..hi.min(lim) {
                    if c[x] >= v {
                        count += 1;
                        c[x] = v;
                    }
                }
                rebuild(b, &mut c, &mut cap, &mut mx, &mut se, &mut cm);
            }
            b += 1;
        }
        let r = rank[j];
        rebuild(r / s, &mut c, &mut cap, &mut mx, &mut se, &mut cm);
        c[r] = INF;
        rebuild(r / s, &mut c, &mut cap, &mut mx, &mut se, &mut cm);
    }
    count
}
