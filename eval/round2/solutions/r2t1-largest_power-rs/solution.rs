// The orderings that keep every earlier power[] entry at its maximum form an
// ordered sequence of blocks: blocks must appear in order, and elements inside
// a block may be permuted freely. For the next bit, the prefix runs through
// every block whose elements all have that bit, then stops inside the first
// block that does not, after that block's elements with the bit (placed first).
// The only orderings reaching that maximum are the ones that split this block
// into [has bit][lacks bit], so the block sequence is refined and we move on.
// At most one split per bit keeps the block count at 16 or fewer.
pub fn largest_power(nums: &[i64]) -> Vec<i64> {
    let mut order: Vec<i64> = nums.to_vec();
    // Block k covers order[bounds[k]..bounds[k + 1]].
    let mut bounds: Vec<usize> = vec![0, order.len()];
    let mut power = Vec::with_capacity(15);

    for i in 0..15 {
        let mask = 1i64 << (14 - i);
        let mut prefix = order.len();
        for k in 0..bounds.len() - 1 {
            let (lo, hi) = (bounds[k], bounds[k + 1]);
            let block = &mut order[lo..hi];
            let with_bit = block.iter().filter(|&&x| x & mask != 0).count();
            if with_bit == block.len() {
                continue;
            }
            prefix = lo + with_bit;
            if with_bit > 0 {
                block.sort_unstable_by_key(|&x| x & mask == 0);
                bounds.insert(k + 1, lo + with_bit);
            }
            break;
        }
        power.push(prefix as i64);
    }
    power
}
