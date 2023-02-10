#![allow(dead_code)]

extern crate test;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bs = s.as_bytes();
        let l = bs.len();
        if l == 0 {
            return 0;
        }
        let (mut left, mut right) = (0, 1);
        let mut res = 1;
        while left < right {
            res = res.max((right - left) as i32);
            right += 1;
            if right > l {
                break;
            }
            for i in (left..(right - 1)).rev() {
                if bs[i] == bs[right - 1] {
                    left = i + 1;
                }
            }
        }
        res
    }
}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bs = s.as_bytes();
        let l = bs.len();
        if l == 0 {
            return 0;
        }
        let (mut left, mut right) = (0, 1);
        let mut res = 1;
        let mut hm = std::collections::HashMap::new();
        hm.insert(bs[0], 1);
        while left < right {
            res = res.max((right - left) as i32);
            right += 1;
            if right > l {
                break;
            }
            hm.entry(bs[right - 1])
                .and_modify(|x| *x += 1)
                .or_insert(1_i32);
            while left < right {
                let limit = (right - left) as i32 - hm.iter().map(|(_, v)| *v).max().unwrap_or(0);
                if limit <= k {
                    break;
                }
                hm.entry(bs[left]).and_modify(|x| *x -= 1);
                left += 1
            }
        }
        res
    }
}

impl Solution {
    // ADOBECODEBANC
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;
        let mut res = String::new();
        let mut total = 0;
        let mut hm = HashMap::new();
        if s.is_empty() || t.is_empty() || t.len() > s.len() {
            return res;
        }

        for &v in t.as_bytes() {
            hm.entry(v).and_modify(|v| *v += 1).or_insert(1);
            total += 1;
        }

        let (mut left, mut right) = (0, 1);

        let bs = s.as_bytes();

        hm.entry(bs[left]).and_modify(|x| {
            *x -= 1;
            total -= 1;
        });

        let l = s.len();

        while left < right {
            if total == 0 {
                if res.is_empty() || res.len() > (right - left) {
                    res = s[left..right].to_string()
                }
                hm.entry(bs[left]).and_modify(|x| {
                    *x += 1;
                    if *x > 0 {
                        total += 1;
                    }
                });
                left += 1;
                continue;
            } else {
                right += 1;
                if right > l {
                    break;
                }
                hm.entry(bs[right - 1]).and_modify(|x| {
                    *x -= 1;
                    if *x >= 0 {
                        total -= 1
                    }
                });
            }
        }

        res
    }
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        s.as_bytes().iter().for_each(|&b| {
            hm.entry(b).and_modify(|x| *x += 1).or_insert(1);
        });
        match t.as_bytes().iter().try_for_each(|&b| match hm.get_mut(&b) {
            None => std::ops::ControlFlow::Break(()),
            Some(v) => {
                *v -= 1;
                std::ops::ControlFlow::Continue(())
            }
        }) {
            std::ops::ControlFlow::Break(_) => false,
            _ => hm.iter().all(|(_, &v)| v == 0),
        }
    }
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let array = (0..26)
            .fold(([0; 26], 3), |(mut dp, mut start), i| {
                'a: loop {
                    for i in 2..=(start / 2) {
                        if start % i == 0 {
                            start += 1;
                            continue 'a;
                        }
                    }
                    break;
                }
                dp[i] = start;
                start += 1;
                (dp, start)
            })
            .0;

        let mut hm = HashMap::<usize, Vec<String>>::new();

        strs.iter().for_each(|x| {
            let val = x
                .as_bytes()
                .iter()
                .fold(1, |acc, &v| acc * array[(v - b'a') as usize]);
            hm.entry(val)
                .and_modify(|vec| vec.push(x.to_string()))
                .or_insert(vec![x.to_string()]);
        });

        hm.into_iter().map(|x| x.1).collect()
    }
}

macro_rules! hm {
    ($($key:expr=>$val:expr),*) => {
        {
            let mut res = std::collections::HashMap::new();
            $(
                res.insert($key,$val);
            )*
            res
        }
    };
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let hs = hm![b']'=>b'[',b'}'=> b'{',b')'=> b'('];
        let mut stack = vec![];
        for &c in s.as_bytes() {
            match c {
                b']' | b'}' | b')' => {
                    let pre = *hs.get(&c).unwrap();
                    if stack.len() > 0 && stack[stack.len() - 1] == pre {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => stack.push(c),
            }
        }
        stack.len() == 0
    }
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let vec = s
            .as_bytes()
            .iter()
            .filter(|&&c| c.is_ascii_alphanumeric())
            .map(|&c| c.to_ascii_lowercase())
            .collect::<Vec<u8>>();
        for i in 0..vec.len() / 2 {
            if vec[i] != vec[vec.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let length = s.len();
        let bs = s.as_bytes();
        let (mut left, mut right) = (0, 1);

        for i in 0..s.len() {
            let vec = vec![(Some(i), i + 1), (Some(i), i + 2)];
            for (mut l, mut r) in vec {
                while l.is_some() && r <= length && bs[l.unwrap()] == bs[r - 1] {
                    if r - l.unwrap() > right - left {
                        right = r;
                        left = l.unwrap();
                    }
                    l = l.unwrap().checked_sub(1);
                    r += 1;
                }
            }
        }

        s[left..right].to_string()
    }
}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let length = s.len();
        let bs = s.as_bytes();
        let mut res = 0;
        for i in 0..s.len() {
            let vec = vec![(Some(i), i + 1), (Some(i), i + 2)];
            for (mut l, mut r) in vec {
                while l.is_some() && r <= length && bs[l.unwrap()] == bs[r - 1] {
                    res += 1;
                    l = l.unwrap().checked_sub(1);
                    r += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        let data = vec![("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3)];
        for (input, want) in data.into_iter() {
            assert_eq!(
                Solution::length_of_longest_substring(input.to_string()),
                want
            );
        }
    }

    #[test]
    fn test_character_replacement() {
        let data = vec![("ABAB", 2, 4), ("AABABBA", 1, 4), ("AAAA", 0, 4)];
        for (s, k, want) in data.into_iter() {
            assert_eq!(Solution::character_replacement(s.to_string(), k), want)
        }
    }

    #[test]
    fn test_min_window() {
        let data = vec![
            ("ADOBECODEBANC", "ABC", "BANC"),
            ("a", "a", "a"),
            ("a", "aa", ""),
            ("cabwefgewcwaefgcf", "cae", "cwae"),
            ("ab", "b", "b"),
        ];

        for (s, t, want) in data.into_iter() {
            assert_eq!(
                Solution::min_window(s.to_string(), t.to_string()),
                want.to_string()
            );
        }
    }

    #[test]
    fn test_is_anagram() {
        let data = vec![("anagram", "nagaram", true), ("rat", "car", false)];
        for (a, b, want) in data.into_iter() {
            assert_eq!(Solution::is_anagram(a.to_string(), b.to_string()), want)
        }
    }

    #[test]
    fn test_group_anagram() {
        let data = vec![
            (
                vec!["eat", "tea", "tan", "ate", "nat", "bat"],
                vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
            ),
            (vec![""], vec![vec![""]]),
            (vec!["a"], vec![vec!["a"]]),
        ];

        for (input, want) in data {
            let input = input.into_iter().map(|x| x.to_string()).collect();
            let mut want = want
                .into_iter()
                .map(|x| x.into_iter().map(|a| a.to_string()).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            want.iter_mut().for_each(|x| x.sort());

            want.sort();

            let mut got = Solution::group_anagrams(input);

            got.iter_mut().for_each(|x| x.sort());

            got.sort();

            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_is_valid() {
        let data = vec![("", true), ("()[]{}", true), ("(]", false)];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::is_valid(input.to_string()), want);
        }
    }

    #[test]
    fn test_is_palindrome() {
        let data = vec![
            ("A man, a plan, a canal: Panama", true),
            ("race a car", false),
            (" ", true),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::is_palindrome(input.to_string()), want)
        }
    }

    #[test]
    fn test_longest_palindrome() {
        let data = vec![("babad", "bab"), ("cbbd", "bb")];

        for (input, want) in data.into_iter() {
            assert_eq!(
                Solution::longest_palindrome(input.to_string()),
                want.to_string()
            )
        }
    }
    #[test]
    fn test_count_substrings() {
        let data = vec![("abc", 3), ("aaa", 6)];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::count_substrings(input.to_string()), want)
        }
    }
}
