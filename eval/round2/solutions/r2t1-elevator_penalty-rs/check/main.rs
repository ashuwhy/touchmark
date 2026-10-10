#[path = "../solution.rs"]
mod solution;
use solution::elevator_penalty;
use std::time::Instant;

// Follows the spec literally: each second the elevator moves +1 or -1 inside
// [0, n-1]; a request costs the first time the elevator stands on it.
fn simulate(n: i64, start: i64, requests: &[i64]) -> i64 {
    let m = requests.len();
    let full = (1usize << m) - 1;
    let mask_at = |pos: i64| -> usize {
        requests.iter().enumerate().filter(|(_, &f)| f == pos).map(|(i, _)| 1 << i).sum()
    };
    const INF: i64 = i64::MAX / 4;
    let nu = n as usize;
    let mut cur = vec![vec![INF; full + 1]; nu];
    cur[start as usize][mask_at(start)] = 0;
    let mut answer = INF;
    let horizon = 4 * n + 4;
    for t in 0..=horizon {
        for p in 0..nu {
            if cur[p][full] < answer {
                answer = cur[p][full];
            }
        }
        if t == horizon {
            break;
        }
        let mut next = vec![vec![INF; full + 1]; nu];
        for p in 0..nu {
            for mask in 0..full {
                let c = cur[p][mask];
                if c >= INF {
                    continue;
                }
                for q in [p as i64 - 1, p as i64 + 1] {
                    if q < 0 || q >= n {
                        continue;
                    }
                    let gained = mask_at(q) & !mask;
                    let nm = mask | gained;
                    let nc = c + (t + 1) * gained.count_ones() as i64;
                    if nc < next[q as usize][nm] {
                        next[q as usize][nm] = nc;
                    }
                }
            }
        }
        cur = next;
    }
    answer
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, b: u64) -> u64 {
        self.next() % b
    }
}

fn distinct(rng: &mut Rng, n: i64, m: usize) -> Vec<i64> {
    let mut seen = std::collections::HashSet::new();
    let mut v = Vec::new();
    while v.len() < m {
        let f = rng.below(n as u64) as i64;
        if seen.insert(f) {
            v.push(f);
        }
    }
    v
}

fn main() {
    assert_eq!(elevator_penalty(6, 4, &[1, 5]), 6);
    assert_eq!(elevator_penalty(8, 3, &[3, 7, 1]), 10);
    assert_eq!(elevator_penalty(10, 5, &[0, 2, 9]), 22);
    assert_eq!(elevator_penalty(1, 0, &[0]), 0);
    assert_eq!(elevator_penalty(1_000_000_000, 0, &[999_999_999]), 999_999_999);
    assert_eq!(elevator_penalty(1_000_000_000, 999_999_999, &[0]), 999_999_999);
    assert_eq!(elevator_penalty(1_000_000_000, 7, &[7]), 0);

    let mut rng = Rng(0x9e3779b97f4a7c15);
    let mut cases = 0;
    for n in 1..=9i64 {
        for _ in 0..400 {
            let m = 1 + rng.below(n.min(6) as u64) as usize;
            let start = rng.below(n as u64) as i64;
            let req = distinct(&mut rng, n, m);
            let got = elevator_penalty(n, start, &req);
            let want = simulate(n, start, &req);
            assert_eq!(got, want, "n={} start={} req={:?}", n, start, req);
            cases += 1;
        }
    }
    println!("brute force agreed on {} cases", cases);

    // Largest sizes: 1500 requests over 1e9 floors, several start positions.
    for &(start, spread) in &[(0i64, 1_000_000_000i64), (999_999_999, 1_000_000_000), (500_000_000, 1_000_000_000), (123, 1_000_000_000), (500, 1500)] {
        let n = 1_000_000_000i64;
        let mut req = distinct(&mut rng, spread, 1500);
        if spread == 1500 {
            for f in req.iter_mut() { *f += 0; }
        }
        let t = Instant::now();
        let v = elevator_penalty(n, start, &req);
        println!("start={} spread={} -> {} in {:?}", start, spread, v, t.elapsed());
        assert!(v >= 0);
    }
    // Worst-ish magnitude: requests packed at both ends, start in the middle.
    let mut req: Vec<i64> = (0..750).collect();
    req.extend((0..750).map(|i| 999_999_999 - i));
    let t = Instant::now();
    let v = elevator_penalty(1_000_000_000, 500_000_000, &req);
    println!("two clusters -> {} in {:?}", v, t.elapsed());
    // Go to 0 cluster first: 750 requests at ~5e8, then 750 at ~1.5e9 + i.
    // Answer must be close to 750*5e8 + 750*1.5e9 = 1.5e12.
    assert!(v > 1_400_000_000_000 && v < 1_600_000_000_000);
    // Start is a request and at the edge, all floors 0..1499.
    let req: Vec<i64> = (0..1500).rev().collect();
    let v = elevator_penalty(1_000_000_000, 0, &req);
    assert_eq!(v, (0..1500i64).sum::<i64>());
    println!("ok");
}
