include!("../solution.rs");

fn brute(nums: &[i64], a: i64, b: i64) -> i64 {
    let mut c = 0;
    for i in 0..nums.len() {
        let (mut x, mut y) = (0i128, 0i128);
        for j in i..nums.len() {
            if nums[j] % 2 == 0 { x += 1 } else { y += 1 }
            if y > 0 && x * (b as i128) <= (a as i128) * y { c += 1 }
        }
    }
    c
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; self.0 }
    fn range(&mut self, lo: i64, hi: i64) -> i64 { lo + (self.next() % ((hi - lo + 1) as u64)) as i64 }
}

fn main() {
    assert_eq!(count_ratio_subarrays(&[1, 2, 1, 2], 3, 2), 7);
    assert_eq!(count_ratio_subarrays(&[2, 2, 1], 2, 1), 3);
    assert_eq!(count_ratio_subarrays(&[2, 2, 2], 1, 1), 0);
    assert_eq!(count_ratio_subarrays(&[1], 1, 1), 1);
    assert_eq!(count_ratio_subarrays(&[2], 1_000_000_000, 1), 0);
    assert_eq!(count_ratio_subarrays(&[1_000_000_000], 1, 1_000_000_000), 0);
    assert_eq!(count_ratio_subarrays(&[999_999_999], 1, 1_000_000_000), 1);

    let mut rng = Rng(0x9E3779B97F4A7C15);
    for t in 0..30000 {
        let n = rng.range(1, 40) as usize;
        let bias = rng.range(0, 10);
        let nums: Vec<i64> = (0..n).map(|_| {
            let v = rng.range(1, 1_000_000_000);
            if rng.range(0, 9) < bias { v | 1 } else { if v % 2 == 1 { v + 1 - 2 * (v == 1_000_000_000) as i64 } else { v } }
        }).collect();
        let (a, b) = match t % 4 {
            0 => (rng.range(1, 5), rng.range(1, 5)),
            1 => (rng.range(1, 1_000_000_000), rng.range(1, 1_000_000_000)),
            2 => { let k = rng.range(1, 6); let m = rng.range(1, 200_000_000); (k * m, m) }
            _ => (rng.range(1, 3), rng.range(1, 1_000_000_000)),
        };
        let got = count_ratio_subarrays(&nums, a, b);
        let want = brute(&nums, a, b);
        assert_eq!(got, want, "nums={:?} a={} b={}", nums, a, b);
    }
    println!("random ok");

    let n = 100_000usize;
    let cases: Vec<(Vec<i64>, i64, i64, i64)> = vec![
        (vec![1; n], 1, 1_000_000_000, (n as i64) * (n as i64 + 1) / 2),
        (vec![2; n], 1_000_000_000, 1, 0),
        (vec![1_000_000_000; n], 1_000_000_000, 1_000_000_000, 0),
    ];
    for (nums, a, b, want) in &cases {
        let t0 = std::time::Instant::now();
        let got = count_ratio_subarrays(nums, *a, *b);
        println!("n={} got={} want={} {:?}", nums.len(), got, want, t0.elapsed());
        assert_eq!(got, *want);
    }
    let mixed: Vec<i64> = (0..n).map(|_| rng.range(1, 1_000_000_000)).collect();
    for (a, b) in [(1_000_000_000, 1), (1, 1_000_000_000), (1, 1), (999_999_937, 999_999_929)] {
        let t0 = std::time::Instant::now();
        let got = count_ratio_subarrays(&mixed, a, b);
        println!("mixed a={} b={} got={} {:?}", a, b, got, t0.elapsed());
    }
    // alternating 1,2 with a=b: every prefix value returns to 0, maximizing ties
    let alt: Vec<i64> = (0..n).map(|i| if i % 2 == 0 { 1 } else { 2 }).collect();
    let t0 = std::time::Instant::now();
    println!("alt got={} {:?}", count_ratio_subarrays(&alt, 1, 1), t0.elapsed());
}
