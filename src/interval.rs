#![allow(dead_code)]

extern crate test;

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.into_iter().fold(vec![new_interval], |mut dp, v| {
            let last = dp.last_mut().unwrap();
            if last[1] < v[0] {
                dp.push(v)
            } else if last[0] > v[1] {
                dp.insert(dp.len() - 1, v)
            } else {
                last[0] = last[0].min(v[0]);
                last[1] = last[1].max(v[1]);
            }
            dp
        })
    }
}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let f = intervals.first().unwrap().clone();
        intervals.into_iter().skip(1).fold(vec![f], |mut dp, v| {
            let last = dp.last_mut().unwrap();
            if last[1] < v[0] {
                dp.push(v)
            } else {
                last[0] = last[0].min(v[0]);
                last[1] = last[1].max(v[1]);
            }
            dp
        })
    }
}

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let l = intervals.last().unwrap().clone();
        let mut dp = vec![l];
        intervals.into_iter().rev().skip(1).fold(0, |mut count, v| {
            let first = dp.first().unwrap();
            if first[0] >= v[1] {
                dp.insert(0, v);
            } else {
                count += 1
            }
            count
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let data = vec![
            (
                vec![vec![1, 3], vec![6, 9]],
                vec![2, 5],
                vec![vec![1, 5], vec![6, 9]],
            ),
            (
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16],
                ],
                vec![4, 8],
                vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            ),
            (
                vec![vec![2, 6], vec![7, 9]],
                vec![15, 18],
                vec![vec![2, 6], vec![7, 9], vec![15, 18]],
            ),
        ];

        for (intervals, new_interval, want) in data.into_iter() {
            assert_eq!(Solution::insert(intervals, new_interval), want)
        }
    }

    #[test]
    fn test_merge() {
        let data = vec![
            (
                vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
                vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            ),
            (vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]),
            (
                vec![vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![1, 10]],
                vec![vec![1, 10]],
            ),
        ];
        for (intervals, want) in data.into_iter() {
            assert_eq!(Solution::merge(intervals), want);
        }
    }

    #[test]
    fn test_erase_overlap_intervals() {
        let data = vec![
            (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]], 1),
            (vec![vec![1, 2], vec![1, 2], vec![1, 2]], 2),
            (vec![vec![1, 2], vec![2, 3]], 0),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::erase_overlap_intervals(input), want);
        }
    }
}
