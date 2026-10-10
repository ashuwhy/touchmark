#[path = "../solution.rs"]
mod solution;
use solution::distant_subarrays;

fn brute(nums: &[i64], goal: i64, k: i64) -> i64 {
    let mut c = 0;
    for i in 0..nums.len() {
        let mut s: i128 = 0;
        for j in i..nums.len() {
            s += nums[j] as i128;
            if (s - goal as i128).abs() >= k as i128 { c += 1; }
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
    assert_eq!(distant_subarrays(&[1, 2, 1], 4, 1), 5);
    assert_eq!(distant_subarrays(&[2, -1, 3], 2, 2), 2);
    assert_eq!(distant_subarrays(&[-3, 1, 2], 0, 3), 2);
    let mut r = Rng(88172645463325252);
    for it in 0..200000 {
        let n = r.range(1, 12) as usize;
        let (vlo, vhi, glim, klim) = match it % 3 {
            0 => (-5, 5, 10, 8),
            1 => (-1_000_000_000, 1_000_000_000, 1_000_000_000, 1_000_000_000),
            _ => (-3, 3, 4, 3),
        };
        let nums: Vec<i64> = (0..n).map(|_| r.range(vlo, vhi)).collect();
        let goal = r.range(-glim, glim);
        let k = r.range(0, klim);
        let (a, b) = (distant_subarrays(&nums, goal, k), brute(&nums, goal, k));
        if a != b { panic!("{:?} {} {} got {} want {}", nums, goal, k, a, b); }
    }
    for &(v, g, k) in &[(1_000_000_000i64, -1_000_000_000i64, 1_000_000_000i64), (-1_000_000_000, 1_000_000_000, 1_000_000_000), (0, 0, 1), (0, 0, 0), (1, 0, 1_000_000_000)] {
        let nums = vec![v; 100_000];
        let t = std::time::Instant::now();
        let a = distant_subarrays(&nums, g, k);
        println!("v={} g={} k={} -> {} in {:?}", v, g, k, a, t.elapsed());
    }
    let nums: Vec<i64> = (0..100_000).map(|_| r.range(-1_000_000_000, 1_000_000_000)).collect();
    let t = std::time::Instant::now();
    let a = distant_subarrays(&nums, 12345, 50_000_000);
    println!("random -> {} in {:?}", a, t.elapsed());
    let small: Vec<i64> = nums[..3000].to_vec();
    assert_eq!(distant_subarrays(&small, 12345, 50_000_000), brute(&small, 12345, 50_000_000));
    println!("ok");
}
