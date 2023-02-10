#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        match b {
            0 => a,
            _ => Self::get_sum(a ^ b, (a & b) << 1),
        }
    }
}

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        match n {
            0 => 0,
            _ => (n & 1) as i32 + Self::hamming_weight(n >> 1),
        }
    }
}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).fold(vec![], |mut dp, x| {
            dp.push(match x {
                0 => 0,
                1 => 1,
                _ => dp[(x & (x - 1)) as usize] + 1,
            });
            dp
        })
    }
}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.len() as i32
            ^ nums
                .into_iter()
                .enumerate()
                .fold(0, |r, (i, v)| r ^ v ^ i as i32)
    }
}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let (mut n, mut x, mut r) = (31, x, 0);
        while x > 0 {
            r += (x & 1) << n;
            n -= 1;
            x >>= 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_sum() {
        let data = vec![((1, 2), 3), ((2, 3), 5)];
        for ((a, b), want) in data.into_iter() {
            assert_eq!(Solution::get_sum(a, b), want)
        }
    }

    #[test]
    fn test_hamming_weight() {
        let data = vec![(11, 3), (128, 1), (((2_u64).pow(32) - 2) as u32, 31)];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::hamming_weight(input), want);
        }
    }

    #[test]
    fn test_count_bits() {
        let data = vec![(2, vec![0, 1, 1]), (5, vec![0, 1, 1, 2, 1, 2])];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::count_bits(input), want);
        }
    }

    #[test]
    fn test_missing_number() {
        let data = vec![
            (vec![3, 0, 1], 2),
            (vec![0, 1], 2),
            (vec![9, 6, 4, 2, 3, 5, 7, 0, 1], 8),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::missing_number(input), want);
        }
    }

    #[test]
    fn test_reverse_bits() {
        let data = vec![
            (0b00000010100101000001111010011100, 964176192),
            (0b11111111111111111111111111111101, 3221225471),
        ];
        for (input, want) in data.into_iter() {
            assert_eq!(Solution::reverse_bits(input), want);
        }
    }
}
