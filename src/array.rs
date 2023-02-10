#![allow(dead_code)]
extern crate test;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        match nums
            .iter()
            .enumerate()
            .try_fold(HashMap::new(), |mut hm, (i, &v)| {
                match hm.get(&(target - v)) {
                    Some(&i0) => Err(vec![i0, i as i32]),
                    None => {
                        hm.insert(v, i as i32);
                        Ok(hm)
                    }
                }
            }) {
            Err(v) => v,
            _ => vec![],
        }
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .skip(1)
            .fold((prices[0], 0), |(min, res), &v| {
                (min.min(v), res.max(v - min))
            })
            .1
    }
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        nums.iter()
            .try_fold(HashSet::new(), |mut hs, &v| match hs.get(&v) {
                None => {
                    hs.insert(v);
                    Some(hs)
                }
                _ => None,
            })
            .is_none()
    }
}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let (left, right) = (1..length).fold(
            (vec![1; length], vec![1; length]),
            |(mut left, mut right), i| {
                left[i] = left[i - 1] * nums[i - 1];
                right[length - i - 1] = right[length - i] * nums[length - i];
                (left, right)
            },
        );
        left.iter().zip(right).map(|(&a, b)| a * b).collect()
    }
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .skip(1)
            .fold((nums[0], nums[0]), |(mut pre, mut res), &v| {
                pre = pre.max(0);
                res = res.max(pre + v);
                pre += v;
                (pre, res)
            })
            .1
    }
}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        use std::mem;
        nums.iter()
            .skip(1)
            .fold(
                (nums[0], nums[0], nums[0]),
                |(mut max, mut min, mut res), &v| {
                    if v < 0 {
                        mem::swap(&mut max, &mut min)
                    }
                    max = (max * v).max(v);
                    min = (min * v).min(v);
                    res = res.max(max);
                    (max, min, res)
                },
            )
            .2
    }
}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        match nums
            .iter()
            .enumerate()
            .skip(1)
            .find(|(i, v)| nums[*i - 1] > **v)
        {
            Some(x) => *x.1,
            None => nums[0],
        }
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let offset = match nums
            .iter()
            .enumerate()
            .skip(1)
            .find(|(i, v)| nums[*i - 1] > **v)
        {
            Some(v) => v.0,
            _ => 0,
        };
        let length = nums.len();
        let new_nums = (0..length)
            .map(|i| nums[(i + offset) % length])
            .collect::<Vec<i32>>();

        match Self::binary_search(new_nums, target, -1, length as i32) {
            Some(v) => (v + offset as i32) % length as i32,
            None => -1,
        }
    }

    fn binary_search(nums: Vec<i32>, target: i32, left: i32, right: i32) -> Option<i32> {
        use std::cmp::Ordering::*;
        let mid = (left + right) / 2;
        match mid {
            x if x == left || x == right => None,
            _ => match nums[mid as usize].cmp(&target) {
                Equal => Some(mid as i32),
                Greater => Self::binary_search(nums, target, left, mid),
                Less => Self::binary_search(nums, target, mid, right),
            },
        }
    }
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering::*;
        use std::collections::HashSet;
        let mut nums = nums;
        nums.sort();
        let length = nums.len();
        let hs = (0..(length - 2)).fold(HashSet::new(), |mut hs, i| {
            let (mut left, mut right) = (i + 1, length - 1);
            while left < right {
                match (nums[left] + nums[right] + nums[i]).cmp(&0) {
                    Greater => right -= 1,
                    Less => left += 1,
                    _ => {
                        hs.insert(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                }
            }
            hs
        });

        hs.into_iter().map(|x| x).collect()
    }
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::Ordering::*;
        let (mut left, mut right, mut res) = (0, height.len() - 1, 0);
        while left < right {
            res = res.max((right - left) as i32 * height[right].min(height[left]));
            match height[right].cmp(&height[left]) {
                Greater => left += 1,
                _ => right -= 1,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        let mut data = vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];
        for (nums, target, want) in data.iter_mut() {
            let mut got = Solution::two_sum(nums.clone(), *target);
            got.sort();
            want.sort();
            assert_eq!(
                got,
                want.to_owned(),
                "nums:{:?}, got:{:?}, want:{:?}",
                nums,
                got,
                want
            )
        }
    }

    #[test]
    fn test_max_profit() {
        let data = vec![(vec![7, 1, 5, 3, 6, 4], 5), (vec![7, 6, 4, 3, 1], 0)];

        for (input, want) in data.iter() {
            assert_eq!(Solution::max_profit(input.clone()), *want)
        }
    }

    #[test]
    fn test_contains_duplicate() {
        let data = vec![
            (vec![1, 2, 3, 3], true),
            (vec![1, 2, 3, 4], false),
            (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
        ];
        for (input, want) in data {
            assert_eq!(Solution::contains_duplicate(input), want)
        }
    }

    #[test]
    fn test_product_except_self() {
        let data = vec![
            (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
            (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
        ];

        for (input, want) in data.iter() {
            assert_eq!(Solution::product_except_self(input.clone()), want.clone());
        }
    }

    #[test]
    fn test_max_sub_array() {
        let data = vec![
            (vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6),
            (vec![1], 1),
            (vec![5, 4, -1, 7, 8], 23),
        ];

        for (input, res) in data.iter() {
            assert_eq!(Solution::max_sub_array(input.clone()), *res)
        }
    }

    #[test]
    fn test_max_product() {
        let data = vec![(vec![2, 3, -2, 4], 6), (vec![-2, 0, -1], 0)];

        for (input, want) in data.iter() {
            assert_eq!(Solution::max_product(input.clone()), *want);
        }
    }

    #[test]
    fn test_find_min() {
        let data = vec![
            (vec![3, 4, 5, 1, 2], 1),
            (vec![4, 5, 6, 7, 0, 1, 2], 0),
            (vec![11, 13, 15, 17], 11),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::find_min(input), want)
        }
    }

    #[test]
    fn test_search() {
        let data = vec![
            (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
            (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
            (vec![1], 0, -1),
            (vec![1, 3], 3, 1),
        ];

        for (input, target, want) in data.into_iter() {
            assert_eq!(
                Solution::search(input.clone(), target),
                want,
                "data is {:?}",
                input
            )
        }
    }

    #[test]
    fn test_three_sum() {
        let data = vec![
            (vec![-1, 0, 1, 2, -1, -4], vec![[-1, -1, 2], [-1, 0, 1]]),
            (vec![0, 1, 1], vec![]),
            (vec![0, 0, 0], vec![[0, 0, 0]]),
        ];

        for (input, want) in data.into_iter() {
            let mut got = Solution::three_sum(input);
            got.sort();
            assert_eq!(got, want)
        }
    }

    #[test]
    fn test_max_area() {
        let data = vec![(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49), (vec![1, 1], 1)];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::max_area(input), want)
        }
    }
}
