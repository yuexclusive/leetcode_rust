#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (1..=n).fold((1, 1), |(a, b), _| (b, a + b)).0
    }
}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        match amount {
            0 => 0,
            _ => {
                let mut dp = vec![-1; amount as usize + 1];
                dp[0] = 0;
                (1..=amount).for_each(|i| {
                    let min = coins
                        .iter()
                        .filter(|&&c| c <= i && dp[(i - c) as usize] != -1)
                        .map(|&c| dp[(i - c) as usize] + 1)
                        .min();
                    dp[i as usize] = min.unwrap_or(-1);
                });
                dp[amount as usize]
            }
        }
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold((1, vec![1; nums.len()]), |(res, mut dp), (i, &v)| {
                (0..i).filter(|&i0| nums[i0] < v).for_each(|i0| {
                    dp[i] = dp[i].max(dp[i0] + 1);
                });
                (res.max(dp[i]), dp)
            })
            .0
    }
}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (l1, l2) = (text1.len(), text2.len());
        text1.as_bytes().iter().enumerate().fold(
            vec![vec![0; l2 + 1]; l1 + 1],
            |mut dp, (i1, c1)| {
                text2.as_bytes().iter().enumerate().for_each(|(i2, c2)| {
                    dp[i1 + 1][i2 + 1] = match c1.cmp(c2) {
                        std::cmp::Ordering::Equal => dp[i1][i2] + 1,
                        _ => dp[i1 + 1][i2].max(dp[i1][i2 + 1]),
                    }
                });
                dp
            },
        )[l1][l2]
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        (1..=s.len()).for_each(|i| {
            dp[i] = word_dict
                .iter()
                .find(|w| w.len() <= i && *w == &s[i - w.len()..i] && dp[i - w.len()] == true)
                .is_some()
        });

        dp[s.len()]
    }
}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        if target == 0 {
            return 0;
        }
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        (1..=target).for_each(|i| {
            nums.iter()
                .filter(|&&n| n <= i)
                .for_each(|&n| dp[i as usize] += dp[(i - n) as usize])
        });
        dp[target as usize]
    }
}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold((0, 0, 0), |(a, b, c), (i, &v)| {
                let (a, b) = match i {
                    0 => (v, 0),
                    1 => (a, a.max(v)),
                    _ => (b, (a + v).max(b)),
                };
                (a, b, c.max(a).max(b))
            })
            .2
    }
}

impl Solution {
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            _ => Self::rob((&nums[1..]).to_vec())
                .max(Self::rob((&nums[..(nums.len() - 1)]).to_vec())),
        }
    }
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bs = s.as_bytes();
        bs.iter()
            .enumerate()
            .fold(vec![0; s.len()], |mut dp, (i, &v)| {
                dp[i] = match i {
                    0 => match v {
                        b'0' => 0,
                        _ => 1,
                    },
                    1 => match bs[i - 1] {
                        b'0' => 0,
                        b'1' => match v {
                            b'0' => 1,
                            _ => 2,
                        },
                        b'2' => match v {
                            b'0' => 1,
                            b'1'..=b'6' => 2,
                            _ => 1,
                        },
                        _ => match v {
                            b'0' => 0,
                            _ => 1,
                        },
                    },
                    _ => match bs[i - 1] {
                        b'0' => match v {
                            b'0' => 0,
                            _ => dp[i - 1],
                        },
                        b'1' => match v {
                            b'0' => dp[i - 2],
                            _ => dp[i - 1] + dp[i - 2],
                        },
                        b'2' => match v {
                            b'0' => dp[i - 2],
                            b'1'..=b'6' => dp[i - 1] + dp[i - 2],
                            _ => dp[i - 1],
                        },
                        _ => match v {
                            b'0' => 0,
                            _ => dp[i - 1],
                        },
                    },
                };
                dp
            })[s.len() - 1]
    }
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp[0][1] = 1;
        (1..=m).fold(dp, |mut dp, i| {
            (1..=n).for_each(|j| dp[i][j] = dp[i - 1][j] + dp[i][j - 1]);
            dp
        })[m][n]
    }
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        match nums.iter().enumerate().try_fold(0, |max, (i, &v)| {
            if max < i {
                return Err(-1);
            }
            let max = (i + v as usize).max(max);
            if max >= nums.len() - 1 {
                return Err(1);
            }
            Ok(max)
        }) {
            Err(-1) => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_climb_stairs() {
        let data = vec![(2, 2), (3, 3)];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::climb_stairs(input), want)
        }
    }

    #[test]
    fn test_coin_change() {
        let data = vec![(vec![1, 2, 5], 11, 3), (vec![2], 3, -1), (vec![1], 0, 0)];
        for (coins, acount, want) in data.into_iter() {
            assert_eq!(Solution::coin_change(coins, acount), want);
        }
    }

    #[test]
    fn test_length_of_lis() {
        let data = vec![
            (vec![10, 9, 2, 5, 3, 7, 101, 18], 4),
            (vec![0, 1, 0, 3, 2, 3], 4),
            (vec![7, 7, 7, 7, 7, 7, 7], 1),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::length_of_lis(input), want)
        }
    }

    #[test]
    fn test_longest_common_subsequence() {
        let data = vec![("abcde", "ace", 3), ("abc", "abc", 3), ("abc", "def", 0)];

        for (t1, t2, want) in data.into_iter() {
            assert_eq!(
                Solution::longest_common_subsequence(t1.to_string(), t2.to_string()),
                want
            )
        }
    }

    #[test]
    fn test_word_break() {
        let data = vec![
            ("leetcode", vec!["leet", "code"], true),
            ("applepenapple", vec!["apple", "pen"], true),
            (
                "catsandog",
                vec!["cats", "dog", "sand", "and", "cat"],
                false,
            ),
        ];

        for (s, dict, want) in data.into_iter() {
            assert_eq!(
                Solution::word_break(s.to_string(), dict.iter().map(|x| x.to_string()).collect()),
                want
            )
        }
    }

    #[test]
    fn test_combination_sum4() {
        let data = vec![(vec![1, 2, 3], 4, 7), (vec![9], 3, 0)];

        for (nums, target, want) in data.into_iter() {
            assert_eq!(Solution::combination_sum4(nums, target), want);
        }
    }

    #[test]
    fn test_rob() {
        let data = vec![
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 12),
            (vec![2, 1, 1, 2], 4),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::rob(input.clone()), want, "input: {:?}", input)
        }
    }

    #[test]
    fn test_rob_2() {
        let data = vec![
            (vec![2, 3, 2], 3),
            (vec![1, 2, 3, 1], 4),
            (vec![1, 2, 3], 3),
            (vec![1, 2, 3, 1], 4),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::rob_2(input), want);
        }
    }

    #[test]
    fn test_num_decodings() {
        let data = vec![("12", 2), ("226", 3), ("06", 0), ("10011", 0)];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::num_decodings(input.to_string()), want);
        }
    }

    #[test]
    fn test_unique_paths() {
        let data = vec![(3, 7, 28), (3, 2, 3)];

        for (m, n, want) in data.into_iter() {
            assert_eq!(Solution::unique_paths(m, n), want);
        }
    }

    #[test]
    fn test_can_jump() {
        let data = vec![(vec![2, 3, 1, 1, 4], true), (vec![3, 2, 1, 0, 4], false)];
        for (input, want) in data.into_iter() {
            assert_eq!(
                Solution::can_jump(input.clone()),
                want,
                "input: {:?}",
                input
            );
        }
    }
}
