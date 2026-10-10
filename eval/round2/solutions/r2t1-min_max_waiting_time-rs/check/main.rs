#[path = "../solution.rs"]
mod solution;
use solution::min_max_waiting_time;

// Literal event simulation over every choice sequence. Fuel counts only cars
// whose start time is <= the release time, to test that assumption in the DP.
fn brute(demand: &[i64], fuel: &[i64]) -> i64 {
    fn go(i: usize, demand: &[i64], fuel: &[i64], release: i64, starts: &mut Vec<(usize, i64)>,
          free: [i64; 2], worst: i64, out: &mut (usize, i64)) {
        let mut any = false;
        if i < demand.len() {
            for j in 0..2 {
                let used: i64 = starts.iter().enumerate()
                    .filter(|(_, &(dj, s))| dj == j && s <= release)
                    .map(|(k, _)| demand[k]).sum();
                assert_eq!(starts.iter().filter(|&&(_, s)| s > release).count(), 0);
                if fuel[j] - used >= demand[i] {
                    any = true;
                    let start = release.max(free[j]);
                    let mut nf = free;
                    nf[j] = start + demand[i];
                    starts.push((j, start));
                    go(i + 1, demand, fuel, start, starts, nf, worst.max(start - release), out);
                    starts.pop();
                }
            }
        }
        if !any {
            let w = if i == 0 { -1 } else { worst };
            if i > out.0 || (i == out.0 && w < out.1) {
                *out = (i, w);
            }
        }
    }
    let mut out = (0usize, i64::MAX);
    go(0, demand, fuel, 0, &mut Vec::new(), [0, 0], 0, &mut out);
    out.1
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; self.0
    }
    fn range(&mut self, lo: i64, hi: i64) -> i64 { lo + (self.next() % (hi - lo + 1) as u64) as i64 }
}

fn main() {
    assert_eq!(min_max_waiting_time(&[6, 8, 4, 6, 5], &[16, 13]), 6);
    assert_eq!(min_max_waiting_time(&[10, 15], &[12, 17]), 0);
    assert_eq!(min_max_waiting_time(&[10, 5], &[8, 8]), -1);
    assert_eq!(min_max_waiting_time(&[20], &[1, 1]), -1);
    assert_eq!(min_max_waiting_time(&[1], &[1, 1]), 0);
    assert_eq!(min_max_waiting_time(&[20; 50], &[50, 50]), 20);

    let mut rng = Rng(0x9e3779b97f4a7c15);
    for t in 0..200000 {
        let n = rng.range(1, if t % 3 == 0 { 14 } else { 9 }) as usize;
        let dmax = [3, 6, 20][t % 3];
        let demand: Vec<i64> = (0..n).map(|_| rng.range(1, dmax)).collect();
        let fmax = [10, 30, 50][(t / 3) % 3];
        let fuel = vec![rng.range(1, fmax), rng.range(1, fmax)];
        let want = brute(&demand, &fuel);
        let got = min_max_waiting_time(&demand, &fuel);
        assert_eq!(got, want, "{:?} {:?}", demand, fuel);
    }

    let start = std::time::Instant::now();
    for _ in 0..200 {
        let demand: Vec<i64> = (0..50).map(|_| rng.range(1, 20)).collect();
        std::hint::black_box(min_max_waiting_time(&demand, &[50, 50]));
        let demand: Vec<i64> = (0..50).map(|_| rng.range(1, 2)).collect();
        std::hint::black_box(min_max_waiting_time(&demand, &[50, 50]));
    }
    println!("ok, 400 max-size runs in {:?}", start.elapsed());
}
