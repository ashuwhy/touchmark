/// Largest sum of a palindromic subarray of `nums`.
///
/// Every element is positive, so at any fixed center the longest palindrome
/// also has the largest sum. Manacher gives the longest palindrome at every
/// center in O(n), and prefix sums price each one in O(1).
///
/// Worst-case sum is 1e5 * 1e9 = 1e14, well inside i64.
pub fn palindromic_subarray_sum(nums: &[i64]) -> i64 {
    let n = nums.len();

    let mut prefix = vec![0i64; n + 1];
    for (i, &x) in nums.iter().enumerate() {
        prefix[i + 1] = prefix[i] + x;
    }

    // Interleave separators so odd and even centers become one case:
    // t = [sep, a0, sep, a1, ..., sep]. A palindrome in t compares positions of
    // equal parity only, so separators never meet real values and need no
    // special value.
    let m = 2 * n + 1;
    let at = |j: usize| -> Option<i64> {
        if j % 2 == 0 {
            None
        } else {
            Some(nums[j / 2])
        }
    };

    let mut radius = vec![0usize; m];
    let mut best = i64::MIN;
    let (mut center, mut reach) = (0usize, 0usize);
    for i in 0..m {
        let mut k = if i < reach {
            radius[2 * center - i].min(reach - i)
        } else {
            0
        };
        while k < i && i + k + 1 < m && at(i - k - 1) == at(i + k + 1) {
            k += 1;
        }
        radius[i] = k;
        if i + k > reach {
            center = i;
            reach = i + k;
        }

        // Radius k in t around i covers original indices [(i-k)/2, (i-k)/2 + k).
        if k > 0 {
            let start = (i - k) / 2;
            best = best.max(prefix[start + k] - prefix[start]);
        }
    }

    best
}
