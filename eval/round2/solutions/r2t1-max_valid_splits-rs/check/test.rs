#[path = "../solution.rs"]
mod solution;
use solution::max_valid_splits;
use std::time::Instant;

fn g(a: i64, b: i64) -> i64 { if b == 0 { a } else { g(b, a % b) } }

fn score(arr: &[i64]) -> i64 {
    let m = arr.len();
    let mut c = 0;
    for i in 0..m.saturating_sub(1) {
        let l = arr[..=i].iter().fold(0, |a, &x| g(a, x));
        let r = arr[i + 1..].iter().fold(0, |a, &x| g(a, x));
        if l == r { c += 1; }
    }
    c
}

fn brute(nums: &[i64]) -> i64 {
    let mut best = score(nums);
    for k in 0..nums.len() {
        let mut a = nums.to_vec();
        a.remove(k);
        best = best.max(score(&a));
    }
    best
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; self.0 }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}

fn main() {
    assert_eq!(max_valid_splits(&[10, 30, 15, 10]), 2);
    assert_eq!(max_valid_splits(&[2, 10, 14]), 1);
    assert_eq!(max_valid_splits(&[2, 4]), 0);
    assert_eq!(max_valid_splits(&[7, 7]), 1);
    assert_eq!(max_valid_splits(&[1_000_000_000, 1_000_000_000]), 1);

    let mut rng = Rng(0x9e3779b97f4a7c15);
    let primes = [2i64, 3, 5, 7];
    let mut cases = 0;
    for _ in 0..400_000 {
        let n = 2 + rng.below(9) as usize;
        let mode = rng.below(5);
        let nums: Vec<i64> = (0..n).map(|_| match mode {
            0 => 1 + rng.below(12) as i64,
            1 => {
                let mut v = 1i64;
                for &p in &primes { for _ in 0..rng.below(3) { v *= p; } }
                v
            }
            2 => [6i64, 10, 15, 30, 2, 3, 5][rng.below(7) as usize],
            3 => 1 + rng.below(1_000_000_000) as i64,
            _ => {
                let base = [1i64, 999_999_937, 536_870_912, 1_000_000_000][rng.below(4) as usize];
                if rng.below(2) == 0 { base } else { 1 + rng.below(1_000_000_000) as i64 }
            }
        }).collect();
        let want = brute(&nums);
        let got = max_valid_splits(&nums);
        if want != got { panic!("mismatch {:?}: want {} got {}", nums, want, got); }
        cases += 1;
    }
    println!("random small: {} cases ok", cases);

    // medium n, brute is O(n^3) so keep n modest
    for _ in 0..300 {
        let n = 2 + rng.below(60) as usize;
        let nums: Vec<i64> = (0..n).map(|_| {
            let mut v = 1i64;
            for &p in &primes { for _ in 0..rng.below(2) { v *= p; } }
            v * 7
        }).collect();
        assert_eq!(max_valid_splits(&nums), brute(&nums), "{:?}", nums);
    }
    println!("medium ok");

    let n = 100_000;
    let mut fib = vec![1i64, 2];
    while *fib.last().unwrap() < 1_000_000_000 { let l = fib.len(); fib.push(fib[l - 1] + fib[l - 2]); }
    fib.pop();
    let big: Vec<(&str, Vec<i64>)> = vec![
        ("all equal max", vec![1_000_000_000; n]),
        ("all ones", vec![1; n]),
        ("random", (0..n).map(|_| 1 + rng.below(1_000_000_000) as i64).collect()),
        ("fib alternating", (0..n).map(|i| fib[fib.len() - 1 - (i % 2)]).collect()),
        ("fib cycle", (0..n).map(|i| fib[fib.len() - 1 - (i % 10)]).collect()),
        ("powers of two desc", (0..n).map(|i| 1i64 << (29 - (i * 30 / n))).collect()),
        ("one odd in middle", (0..n).map(|i| if i == n / 2 { 3 } else { 6 }).collect()),
        ("multiples of large prime", (0..n).map(|_| 999_983 * (1 + rng.below(1000) as i64)).collect()),
        ("2^29 except ends", (0..n).map(|i| if i == 0 || i == n - 1 { 3 } else { 1 << 29 }).collect()),
    ];
    for (name, nums) in &big {
        let t = Instant::now();
        let r = max_valid_splits(nums);
        println!("{:28} n={} -> {:6} in {:?}", name, nums.len(), r, t.elapsed());
    }
    assert_eq!(max_valid_splits(&vec![1_000_000_000; n]), (n - 1) as i64);
    let mut odd_mid: Vec<i64> = vec![6; n];
    odd_mid[n / 2] = 3;
    assert_eq!(max_valid_splits(&odd_mid), (n - 2) as i64);
    println!("all ok");
}
