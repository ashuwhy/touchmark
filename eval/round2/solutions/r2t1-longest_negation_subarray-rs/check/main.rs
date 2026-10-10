include!("../solution.rs");

use std::time::Instant;

fn brute(nums: &[i64], k: i64) -> i64 {
    let n = nums.len();
    let mut longest = 0;
    for l in 0..n {
        let mut s = 0i64;
        for r in l..n {
            s += nums[r];
            let ok = s.rem_euclid(k) == 0
                || (l..=r).any(|i| (s - 2 * nums[i]).rem_euclid(k) == 0);
            if ok {
                longest = longest.max((r - l + 1) as i64);
            }
        }
    }
    longest
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % ((hi - lo + 1) as u64)) as i64
    }
}

fn main() {
    assert_eq!(longest_negation_subarray(&[4, 1, 2], 3), 3);
    assert_eq!(longest_negation_subarray(&[5, 3, 4], 7), 2);
    assert_eq!(longest_negation_subarray(&[2, 2, 5], 6), 2);
    assert_eq!(longest_negation_subarray(&[1], 1), 1);
    assert_eq!(longest_negation_subarray(&[1], 2), 0);
    assert_eq!(longest_negation_subarray(&[0], 3000), 1);
    assert_eq!(longest_negation_subarray(&[-100000], 3000), 0);

    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut cases = 0;
    for _ in 0..200_000 {
        let n = rng.range(1, 14) as usize;
        let k = match rng.range(0, 3) {
            0 => rng.range(1, 6),
            1 => rng.range(1, 30),
            2 => rng.range(1, 3000),
            _ => rng.range(2990, 3000),
        };
        let span = match rng.range(0, 3) {
            0 => 3,
            1 => 20,
            2 => 1000,
            _ => 100_000,
        };
        let nums: Vec<i64> = (0..n).map(|_| rng.range(-span, span)).collect();
        let want = brute(&nums, k);
        let got = longest_negation_subarray(&nums, k);
        if want != got {
            panic!("mismatch nums={:?} k={} want={} got={}", nums, k, want, got);
        }
        cases += 1;
    }
    for _ in 0..3000 {
        let n = rng.range(30, 120) as usize;
        let k = rng.range(1, 200);
        let span = [5, 100, 100_000][rng.range(0, 2) as usize];
        let nums: Vec<i64> = (0..n).map(|_| rng.range(-span, span)).collect();
        assert_eq!(brute(&nums, k), longest_negation_subarray(&nums, k), "nums={:?} k={}", nums, k);
        cases += 1;
    }
    println!("random cases ok: {}", cases);

    let n = 100_000;
    let mut big: Vec<(String, Vec<i64>, i64)> = Vec::new();
    for &k in &[3000i64, 2999, 2048, 1] {
        big.push((format!("random k={}", k), (0..n).map(|_| rng.range(-100_000, 100_000)).collect(), k));
    }
    big.push(("all ones k=3000".into(), vec![1; n], 3000));
    big.push(("all max k=2999".into(), vec![100_000; n], 2999));
    big.push(("all min k=3000".into(), vec![-100_000; n], 3000));
    big.push(("zeros k=3000".into(), vec![0; n], 3000));
    // distinct prefix residues early, then cycle through every d
    let mut crafted = vec![1i64; 3000];
    for i in 0..(n - 3000) {
        crafted.push(((i * 1501) % 3000) as i64);
    }
    big.push(("crafted k=3000".into(), crafted, 3000));
    big.push(("alternating k=2999".into(), (0..n).map(|i| if i % 2 == 0 { 99_999 } else { -100_000 }).collect(), 2999));
    for (name, nums, k) in &big {
        let t = Instant::now();
        let got = longest_negation_subarray(nums, *k);
        println!("{:<22} -> {:>6} in {:?}", name, got, t.elapsed());
    }
}
