include!("../solution.rs");

fn power_of(p: &[i64]) -> Vec<i64> {
    (0..15)
        .map(|i| {
            let mask = 1i64 << (14 - i);
            p.iter().take_while(|&&x| x & mask != 0).count() as i64
        })
        .collect()
}

fn brute(nums: &[i64]) -> Vec<i64> {
    let mut p = nums.to_vec();
    let mut best = power_of(&p);
    // Heap's algorithm
    let n = p.len();
    let mut c = vec![0usize; n];
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 { p.swap(0, i); } else { p.swap(c[i], i); }
            let pw = power_of(&p);
            if pw > best { best = pw; }
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
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
    assert_eq!(largest_power(&[7, 5]), vec![0,0,0,0,0,0,0,0,0,0,0,0,2,1,2]);
    assert_eq!(largest_power(&[3, 1, 7]), vec![0,0,0,0,0,0,0,0,0,0,0,0,1,2,3]);
    assert_eq!(largest_power(&[0]), vec![0; 15]);
    assert_eq!(largest_power(&[32767]), vec![1; 15]);

    let mut rng = Rng(0x9e3779b97f4a7c15);
    for t in 0..20000 {
        let n = 1 + (rng.next() % 7) as usize;
        // vary which bits are live so splits happen at high and low bits
        let live = match t % 4 {
            0 => 0x7fff,
            1 => (rng.next() & 0x7fff) as i64,
            2 => 0x7000 | 0x7,
            _ => 0x4000 | 0x0400 | 0x0010 | 0x0001,
        };
        let base = if t % 3 == 0 { (rng.next() & 0x7fff) as i64 } else { 0 };
        let nums: Vec<i64> = (0..n)
            .map(|_| ((rng.next() & 0x7fff) as i64 & live) | (base & !live))
            .collect();
        let got = largest_power(&nums);
        let want = brute(&nums);
        assert_eq!(got, want, "nums={:?}", nums);
    }
    println!("brute ok");

    let big: Vec<i64> = (0..50_000).map(|_| (rng.next() & 0x7fff) as i64).collect();
    let start = std::time::Instant::now();
    let r = largest_power(&big);
    println!("random 50k {:?} in {:?}", r, start.elapsed());
    let all_max = vec![32767i64; 50_000];
    let start = std::time::Instant::now();
    let r = largest_power(&all_max);
    println!("all max {:?} in {:?}", r, start.elapsed());
    let mix: Vec<i64> = (0..50_000).map(|i| if i % 2 == 0 { 32767 } else { 0 }).collect();
    println!("mix {:?}", largest_power(&mix));
}
