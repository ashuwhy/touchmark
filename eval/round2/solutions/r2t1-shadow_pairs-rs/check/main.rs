#[path = "../solution.rs"]
mod solution;
use solution::shadow_pairs;
use std::time::Instant;

fn by_spec(nums: &[i64]) -> i64 {
    let n = nums.len();
    let mut c = 0;
    for i in 0..n {
        for j in i + 1..n {
            if nums[i] < nums[j] && !(i + 1..j).any(|k| nums[i] < nums[k] && nums[k] < nums[j]) {
                c += 1;
            }
        }
    }
    c
}

fn by_scan(nums: &[i64]) -> i64 {
    let n = nums.len();
    let mut c = 0;
    for i in 0..n {
        let mut cap = i64::MAX;
        for j in i + 1..n {
            let v = nums[j];
            if v > nums[i] && v <= cap {
                c += 1;
                cap = v;
            }
        }
    }
    c
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
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

fn main() {
    assert_eq!(shadow_pairs(&[3, 1, 4, 2, 5]), 5);
    assert_eq!(shadow_pairs(&[6, 7, 8, 9]), 3);
    assert_eq!(shadow_pairs(&[1, 1, 1]), 0);
    assert_eq!(shadow_pairs(&[3, 2, 1]), 0);
    assert_eq!(shadow_pairs(&[1, 2, 1, 2]), 3);
    assert_eq!(shadow_pairs(&[1, 3, 2]), 2);
    assert_eq!(shadow_pairs(&[1, 2, 3]), 2);

    let mut rng = Rng(0x9e3779b97f4a7c15);
    for _ in 0..3000 {
        let n = rng.range(3, 40) as usize;
        let hi = [1, 2, 3, 5, 10, 40, 1_000_000_000][rng.range(0, 6) as usize];
        let nums: Vec<i64> = (0..n).map(|_| rng.range(1, hi)).collect();
        let want = by_spec(&nums);
        assert_eq!(by_scan(&nums), want, "scan {:?}", nums);
        assert_eq!(shadow_pairs(&nums), want, "{:?}", nums);
    }
    for _ in 0..30 {
        let n = rng.range(3, 2000) as usize;
        let hi = [2, 7, 100, 1000, 1_000_000_000][rng.range(0, 4) as usize];
        let nums: Vec<i64> = (0..n).map(|_| rng.range(1, hi)).collect();
        assert_eq!(shadow_pairs(&nums), by_scan(&nums));
    }
    println!("random ok");

    let n = 50_000usize;
    let m = (n / 2) as i64;
    let mut cases: Vec<(&str, Vec<i64>)> = vec![
        ("random", (0..n).map(|_| rng.range(1, 1_000_000_000)).collect()),
        ("random small", (0..n).map(|_| rng.range(1, 50)).collect()),
        ("increasing", (1..=n as i64).collect()),
        ("decreasing", (1..=n as i64).rev().collect()),
        ("all max", vec![1_000_000_000; n]),
        ("ones then twos", (0..n).map(|i| if i < n / 2 { 1 } else { 2 }).collect()),
        ("two descents", (0..n as i64).map(|i| if i < m { m - i } else { 2 * m - (i - m) }).collect()),
        ("zigzag", (0..n as i64).map(|i| if i % 2 == 0 { i + 1 } else { 1_000_000_000 - i }).collect()),
        ("alternating 1 1e9", (0..n).map(|i| if i % 2 == 0 { 1 } else { 1_000_000_000 }).collect()),
    ];
    cases.push(("sawtooth", (0..n as i64).map(|i| 1 + i % 223).collect()));
    for (name, nums) in &cases {
        let t = Instant::now();
        let got = shadow_pairs(nums);
        let el = t.elapsed();
        let want = by_scan(nums);
        assert_eq!(got, want, "{}", name);
        println!("{:<18} {:>12} {:?}", name, got, el);
    }
}
