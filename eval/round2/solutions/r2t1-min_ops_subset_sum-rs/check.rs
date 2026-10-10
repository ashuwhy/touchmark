include!("solution.rs");

use std::collections::VecDeque;
use std::time::Instant;

// Whole-state BFS over the tuple of current values, capped well above any value
// an optimal answer needs, so it does not share the (v >> k) << m argument.
fn brute(nums: &[i64], target: i64) -> i64 {
    let n = nums.len();
    let cap = 4 * nums.iter().copied().max().unwrap().max(target) as usize;
    let base = cap + 1;
    let total = base.pow(n as u32);
    let encode = |vals: &[usize]| vals.iter().fold(0usize, |acc, &x| acc * base + x);
    let decode = |mut code: usize| {
        let mut vals = vec![0usize; n];
        for i in (0..n).rev() {
            vals[i] = code % base;
            code /= base;
        }
        vals
    };
    let hits = |vals: &[usize]| {
        (1u32..(1 << n)).any(|mask| {
            (0..n).filter(|i| mask >> i & 1 == 1).map(|i| vals[i]).sum::<usize>() == target as usize
        })
    };
    let start: Vec<usize> = nums.iter().map(|&x| x as usize).collect();
    let mut dist = vec![u32::MAX; total];
    let s0 = encode(&start);
    dist[s0] = 0;
    let mut q = VecDeque::from([s0]);
    while let Some(code) = q.pop_front() {
        let vals = decode(code);
        if hits(&vals) {
            return dist[code] as i64;
        }
        for i in 0..n {
            for next in [vals[i] * 2, vals[i] / 2] {
                if next > cap {
                    continue;
                }
                let mut moved = vals.clone();
                moved[i] = next;
                let c = encode(&moved);
                if dist[c] == u32::MAX {
                    dist[c] = dist[code] + 1;
                    q.push_back(c);
                }
            }
        }
    }
    -1
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
    assert_eq!(min_ops_subset_sum(&[10, 2], 13), 3);
    assert_eq!(min_ops_subset_sum(&[6, 3], 8), 2);
    assert_eq!(min_ops_subset_sum(&[2, 2], 7), -1);
    assert_eq!(min_ops_subset_sum(&[1], 1), 0);
    assert_eq!(min_ops_subset_sum(&[500], 1), 8);
    assert_eq!(min_ops_subset_sum(&[1], 4096), 12);
    assert_eq!(min_ops_subset_sum(&[1], 5000), -1);
    assert_eq!(min_ops_subset_sum(&[500], 4000), 3);
    assert_eq!(min_ops_subset_sum(&[3], 5), -1);
    assert_eq!(min_ops_subset_sum(&[500, 500], 1000), 0);

    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut checked = 0;
    let mut neg = 0;
    for round in 0..4000 {
        let n = rng.range(1, 3) as usize;
        let vmax = if round % 2 == 0 { 12 } else { 30 };
        let nums: Vec<i64> = (0..n).map(|_| rng.range(1, vmax)).collect();
        let target = rng.range(1, 40);
        let want = brute(&nums, target);
        let got = min_ops_subset_sum(&nums, target);
        if want != got {
            panic!("mismatch nums={:?} target={} want={} got={}", nums, target, want, got);
        }
        if want == -1 {
            neg += 1;
        }
        checked += 1;
    }
    println!("random vs brute: {} cases ok ({} unreachable)", checked, neg);

    let mut worst = 0.0f64;
    for trial in 0..20 {
        let nums: Vec<i64> = match trial {
            0 => vec![500; 100],
            1 => vec![1; 100],
            2 => (1..=100).map(|i| i * 5).collect(),
            3 => (401..=500).collect(),
            _ => (0..100).map(|_| rng.range(1, 500)).collect(),
        };
        let target = if trial % 3 == 0 { 5000 } else { rng.range(4500, 5000) };
        let t0 = Instant::now();
        let ans = min_ops_subset_sum(&nums, target);
        let el = t0.elapsed().as_secs_f64();
        worst = worst.max(el);
        if trial < 4 {
            println!("trial {} target {} -> {} in {:.4}s", trial, target, ans, el);
        }
    }
    println!("worst max-size time {:.4}s", worst);
}
