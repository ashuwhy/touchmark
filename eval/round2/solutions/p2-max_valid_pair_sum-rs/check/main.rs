include!("../solution.rs");

fn brute(nums: &[i64], k: i64) -> i64 {
    let n = nums.len();
    let mut best = i64::MIN;
    for i in 0..n {
        for j in i + 1..n {
            if (j - i) as i64 >= k {
                best = best.max(nums[i] + nums[j]);
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
    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next() % (hi - lo + 1)
    }
}

fn main() {
    assert_eq!(max_valid_pair_sum(&[1, 3, 5, 2, 8], 2), 13);
    assert_eq!(max_valid_pair_sum(&[5, 1, 9], 1), 14);
    assert_eq!(max_valid_pair_sum(&[7, 4], 1), 11);
    assert_eq!(max_valid_pair_sum(&[1_000_000_000, 1_000_000_000], 1), 2_000_000_000);
    assert_eq!(max_valid_pair_sum(&[9, 100, 100, 1], 3), 10);

    let mut rng = Rng(0x9e3779b97f4a7c15);
    for _ in 0..200_000 {
        let n = rng.range(2, 12) as usize;
        let hi = if rng.next() % 2 == 0 { 10 } else { 1_000_000_000 };
        let nums: Vec<i64> = (0..n).map(|_| rng.range(1, hi) as i64).collect();
        let k = rng.range(1, n as u64 - 1) as i64;
        assert_eq!(max_valid_pair_sum(&nums, k), brute(&nums, k), "{:?} k={}", nums, k);
    }

    let n = 100_000;
    let nums: Vec<i64> = (0..n).map(|_| rng.range(1, 1_000_000_000) as i64).collect();
    let t = std::time::Instant::now();
    for k in [1i64, 2, 50_000, 99_998, 99_999] {
        let got = max_valid_pair_sum(&nums, k);
        if k == 99_999 {
            assert_eq!(got, nums[0] + nums[n - 1]);
        }
    }
    let maxed = vec![1_000_000_000i64; n];
    assert_eq!(max_valid_pair_sum(&maxed, 99_999), 2_000_000_000);
    assert_eq!(max_valid_pair_sum(&maxed, 1), 2_000_000_000);
    println!("ok, large cases {:?}", t.elapsed());
}
