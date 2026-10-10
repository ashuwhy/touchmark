/// Counts pairs i < j with nums[i] < nums[j] and no k in (i, j) whose value
/// lies strictly between them.
///
/// Each index becomes a point (i, rank). A pair counts when the open rectangle
/// spanned by its two points holds no other point. Splitting the ranks in half
/// leaves same-half pairs to the recursion, since anything strictly between two
/// values in one half lies in that half too. Cross pairs are counted in one sweep
/// by index. O(n log^2 n) time, O(n) extra space.
pub fn shadow_pairs(nums: &[i64]) -> i64 {
    let n = nums.len();
    // An equal value must neither pair up nor block. Breaking ties by
    // descending index gives distinct ranks that keep both of those true.
    let mut by_value: Vec<u32> = (0..n as u32).collect();
    by_value.sort_unstable_by(|&a, &b| {
        nums[a as usize]
            .cmp(&nums[b as usize])
            .then(b.cmp(&a))
    });
    let mut rank = vec![0u32; n];
    for (r, &i) in by_value.iter().enumerate() {
        rank[i as usize] = r as u32;
    }

    let mut xs: Vec<u32> = (0..n as u32).collect();
    let mut spare = vec![0u32; n];
    let mut stacks = Stacks { lower: Vec::new(), upper: Vec::new() };
    count_band(&mut xs, &mut spare, &rank, 0, &mut stacks) as i64
}

struct Stacks {
    lower: Vec<u32>,
    upper: Vec<u32>,
}

/// `xs` holds, in ascending order, every index whose rank is in
/// [lo, lo + xs.len()). Counts the shadow pairs among them.
fn count_band(xs: &mut [u32], spare: &mut [u32], rank: &[u32], lo: u32, stacks: &mut Stacks) -> u64 {
    let len = xs.len();
    if len < 2 {
        return 0;
    }
    let split = len / 2;
    let mid = lo + split as u32;

    // Sweep by index. `lower` keeps the lower-half points not yet blocked from
    // the right by a higher lower-half point: ascending index, descending rank.
    // `upper` is an ascending-rank stack, so after popping, its top is the
    // nearest earlier upper-half point below the current one. That point is a
    // wall: a partner left of it would have it inside the rectangle.
    let Stacks { lower, upper } = stacks;
    lower.clear();
    upper.clear();
    let mut pairs = 0u64;
    for &x in xs.iter() {
        let r = rank[x as usize];
        if r < mid {
            while lower.last().map_or(false, |&t| rank[t as usize] < r) {
                lower.pop();
            }
            lower.push(x);
        } else {
            while upper.last().map_or(false, |&t| rank[t as usize] > r) {
                upper.pop();
            }
            let open = match upper.last() {
                Some(&wall) => lower.len() - lower.partition_point(|&p| p < wall),
                None => lower.len(),
            };
            pairs += open as u64;
            upper.push(x);
        }
    }

    let (mut a, mut b) = (0, split);
    for &x in xs.iter() {
        if rank[x as usize] < mid {
            spare[a] = x;
            a += 1;
        } else {
            spare[b] = x;
            b += 1;
        }
    }
    xs.copy_from_slice(&spare[..len]);

    let (below, above) = xs.split_at_mut(split);
    pairs
        + count_band(below, spare, rank, lo, stacks)
        + count_band(above, spare, rank, mid, stacks)
}
