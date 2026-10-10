#[path = "solution.rs"]
mod solution;
use solution::palindromic_subarray_sum;
use std::time::Instant;

fn brute(a: &[i64]) -> i64 {
    let n = a.len();
    let mut best = i64::MIN;
    for l in 0..n {
        for r in l..n {
            let s = &a[l..=r];
            if s.iter().eq(s.iter().rev()) {
                best = best.max(s.iter().sum());
            }
        }
    }
    best
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn main() {
    assert_eq!(palindromic_subarray_sum(&[1, 2, 3, 2, 1, 5, 6]), 9);
    assert_eq!(palindromic_subarray_sum(&[1, 2, 3, 4, 5]), 5);
    assert_eq!(palindromic_subarray_sum(&[10, 10]), 20);
    assert_eq!(palindromic_subarray_sum(&[7]), 7);
    assert_eq!(palindromic_subarray_sum(&[1_000_000_000]), 1_000_000_000);
    // a short palindrome of big values beats a long one of small values
    assert_eq!(palindromic_subarray_sum(&[1, 1, 1, 1, 100]), 100);
    assert_eq!(palindromic_subarray_sum(&[50, 1, 50, 2, 2, 2, 2]), 101);

    let mut rng = Rng(0x9e3779b97f4a7c15);
    for iter in 0..200_000 {
        let n = (rng.next() % 14 + 1) as usize;
        let alpha = [1u64, 2, 3, 5, 1_000_000_000][iter % 5];
        let a: Vec<i64> = (0..n)
            .map(|_| if alpha == 1_000_000_000 { (rng.next() % 3) as i64 * 499_999_999 + 1 } else { (rng.next() % alpha + 1) as i64 })
            .collect();
        let got = palindromic_subarray_sum(&a);
        let want = brute(&a);
        assert_eq!(got, want, "{:?}", a);
    }
    println!("random ok");

    let n = 100_000;
    let cases: Vec<(&str, Vec<i64>, i64)> = vec![
        ("all max", vec![1_000_000_000; n], 100_000_000_000_000),
        ("all one", vec![1; n], n as i64),
        ("alternating", (0..n).map(|i| (i % 2) as i64 + 1).collect(), -1),
        ("increasing", (0..n).map(|i| i as i64 + 1).collect(), n as i64),
        ("mountain", (0..n).map(|i| (i.min(n - 1 - i)) as i64 + 1).collect(), -1),
        ("random", (0..n).map(|_| (rng.next() % 1_000_000_000) as i64 + 1).collect(), -1),
        ("random2", (0..n).map(|_| (rng.next() % 2) as i64 + 1).collect(), -1),
    ];
    for (name, a, want) in cases {
        let t = Instant::now();
        let got = palindromic_subarray_sum(&a);
        let dt = t.elapsed();
        if want >= 0 {
            assert_eq!(got, want, "{}", name);
        }
        println!("{name}: {got} in {:?}", dt);
    }
}
