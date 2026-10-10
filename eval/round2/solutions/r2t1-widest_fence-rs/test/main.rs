#[path = "../solution.rs"]
mod solution;
use solution::widest_fence;
use std::time::Instant;

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

// Every plank is unused, a single, or paired with a later unused plank.
fn brute(p: &[i64]) -> i64 {
    fn go(p: &[i64], used: &mut Vec<bool>, i: usize, pieces: &mut Vec<i64>, best: &mut i64) {
        if i == p.len() {
            if let Some(&h) = pieces.first() {
                if pieces.iter().all(|&x| x == h) {
                    *best = (*best).max(pieces.len() as i64);
                }
            }
            return;
        }
        if used[i] {
            return go(p, used, i + 1, pieces, best);
        }
        go(p, used, i + 1, pieces, best);
        pieces.push(p[i]);
        go(p, used, i + 1, pieces, best);
        pieces.pop();
        used[i] = true;
        for j in i + 1..p.len() {
            if !used[j] {
                used[j] = true;
                pieces.push(p[i] + p[j]);
                go(p, used, i + 1, pieces, best);
                pieces.pop();
                used[j] = false;
            }
        }
        used[i] = false;
    }
    let mut best = 0;
    go(p, &mut vec![false; p.len()], 0, &mut Vec::new(), &mut best);
    best
}

fn main() {
    assert_eq!(widest_fence(&[1, 3, 2, 5, 7, 5, 4, 2, 1]), 4);
    assert_eq!(widest_fence(&[2, 3, 7]), 1);
    assert_eq!(widest_fence(&[1]), 1);
    assert_eq!(widest_fence(&[1_000_000_000]), 1);
    assert_eq!(widest_fence(&[1_000_000_000, 1_000_000_000]), 2);
    assert_eq!(widest_fence(&[1, 1]), 2);
    assert_eq!(widest_fence(&[1, 1, 2]), 2);
    assert_eq!(widest_fence(&[1, 1, 1]), 3);
    assert_eq!(widest_fence(&vec![7; 1000]), 1000);
    let half: Vec<i64> = (0..1000).map(|i| if i % 2 == 0 { 1 } else { 999_999_999 }).collect();
    assert_eq!(widest_fence(&half), 500);

    let mut rng = Rng(0x9e3779b97f4a7c15);
    for iter in 0..30000 {
        let n = rng.range(1, 8) as usize;
        let hi = match iter % 3 { 0 => 3, 1 => 8, _ => 1_000_000_000 };
        let lo = if iter % 5 == 0 { (hi - 4).max(1) } else { 1 };
        let p: Vec<i64> = (0..n).map(|_| rng.range(lo, hi)).collect();
        let (a, b) = (widest_fence(&p), brute(&p));
        assert_eq!(a, b, "{:?}", p);
    }
    println!("random brute ok");

    for case in 0..4 {
        let p: Vec<i64> = (0..1000)
            .map(|i| match case {
                0 => rng.range(1, 1_000_000_000),
                1 => 1_000_000_000 - i,
                2 => i + 1,
                _ => rng.range(1, 50),
            })
            .collect();
        let t = Instant::now();
        let r = widest_fence(&p);
        println!("case {} -> {} in {:?}", case, r, t.elapsed());
    }
}
