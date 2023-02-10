#![allow(dead_code)]

extern crate test;

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut ins = vec![0; num_courses as usize];
        let mut outs = vec![vec![]; num_courses as usize];
        for v in prerequisites.into_iter() {
            ins[v[0] as usize] += 1;
            outs[v[1] as usize].push(v[0]);
        }
        while let Some((i, _)) = ins.iter().enumerate().find(|(_, v)| **v == 0) {
            ins[i] = -1;
            for j in outs[i].iter() {
                ins[*j as usize] -= 1;
            }
        }
        ins.iter().all(|v| *v == -1)
    }
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = vec![vec![0; heights[0].len()]; heights.len()];
        map.iter_mut().enumerate().for_each(|(i, x)| {
            x.iter_mut().enumerate().for_each(|(j, v)| {
                if i == 0 || j == 0 {
                    *v += 1
                }
                if i == heights.len() - 1 || j == heights[0].len() - 1 {
                    *v += 2
                }
            })
        });

        (0..heights.len()).for_each(|i| {
            Self::pacific_atlantic_walk(&heights, &mut map, i, 0, Some(i), Some(0), 1);
            Self::pacific_atlantic_walk(
                &heights,
                &mut map,
                i,
                heights[0].len() - 1,
                Some(i),
                Some(heights[0].len() - 1),
                2,
            );
        });

        (0..heights[0].len()).for_each(|j| {
            Self::pacific_atlantic_walk(&heights, &mut map, 0, j, Some(0), Some(j), 1);
            Self::pacific_atlantic_walk(
                &heights,
                &mut map,
                heights.len() - 1,
                j,
                Some(heights.len() - 1),
                Some(j),
                2,
            );
        });

        map.iter()
            .enumerate()
            .flat_map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .filter(|(_, v)| **v == 3)
                    .map(move |(j, _)| vec![i as i32, j as i32])
            })
            .collect()
    }

    fn pacific_atlantic_walk(
        height: &Vec<Vec<i32>>,
        map: &mut Vec<Vec<i32>>,
        i0: usize,
        j0: usize,
        i: Option<usize>,
        j: Option<usize>,
        val: i32,
    ) {
        if i.is_none() || j.is_none() {
            return;
        }
        let (i, j) = (i.unwrap(), j.unwrap());

        if i >= height.len() || j >= height[i].len() {
            return;
        }

        if (i0 != i || j0 != j) && (map[i][j] & val != 0) {
            return;
        }

        if height[i][j] < height[i0][j0] {
            return;
        }

        map[i][j] |= val;

        Self::pacific_atlantic_walk(height, map, i, j, i.checked_add(1), Some(j), val);
        Self::pacific_atlantic_walk(height, map, i, j, i.checked_sub(1), Some(j), val);
        Self::pacific_atlantic_walk(height, map, i, j, Some(i), j.checked_add(1), val);
        Self::pacific_atlantic_walk(height, map, i, j, Some(i), j.checked_sub(1), val);
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut map = vec![vec![false; grid[0].len()]; grid.len()];
        grid.iter()
            .enumerate()
            .flat_map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, v)| {
                        if *v == char::from('1') && !map[i][j] {
                            Self::nums_islands_walk(&grid, &mut map, Some(i), Some(j));
                            return 1;
                        }
                        0
                    })
                    .collect::<Vec<i32>>()
            })
            .sum()
    }

    fn nums_islands_walk(
        grid: &Vec<Vec<char>>,
        map: &mut Vec<Vec<bool>>,
        i: Option<usize>,
        j: Option<usize>,
    ) {
        if i.is_none() || j.is_none() {
            return;
        }

        let (i, j) = (i.unwrap(), j.unwrap());

        if i >= grid.len() || j >= grid[i].len() {
            return;
        }

        if grid[i][j] != char::from('1') {
            return;
        }
        if map[i][j] {
            return;
        }
        map[i][j] = true;

        Self::nums_islands_walk(grid, map, i.checked_add(1), Some(j));
        Self::nums_islands_walk(grid, map, i.checked_sub(1), Some(j));
        Self::nums_islands_walk(grid, map, Some(i), j.checked_add(1));
        Self::nums_islands_walk(grid, map, Some(i), j.checked_sub(1));
    }
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = nums
            .iter()
            .map(|&x| (x, false))
            .collect::<HashMap<i32, bool>>();

        nums.iter()
            .map(|&v| {
                let mut s1 = v - 1;
                let mut s2 = v + 1;
                let mut c = 1;

                while let Some(mark) = map.get_mut(&s1) {
                    if *mark {
                        break;
                    }
                    *mark = true;
                    s1 -= 1;
                    c += 1;
                }

                while let Some(mark) = map.get_mut(&s2) {
                    if *mark {
                        break;
                    }
                    *mark = true;
                    s2 += 1;
                    c += 1;
                }
                c
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_finish() {
        let data = vec![
            (2, vec![vec![1, 0]], true),
            (2, vec![vec![1, 0], vec![0, 1]], false),
        ];

        for (num_courses, prerequisites, want) in data.into_iter() {
            assert_eq!(Solution::can_finish(num_courses, prerequisites), want);
        }
    }

    #[test]
    fn test_pacific_atlantic() {
        let data = vec![
            (
                vec![
                    vec![1, 2, 2, 3, 5],
                    vec![3, 2, 3, 4, 4],
                    vec![2, 4, 5, 3, 1],
                    vec![6, 7, 1, 4, 5],
                    vec![5, 1, 1, 2, 4],
                ],
                vec![
                    vec![0, 4],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 2],
                    vec![3, 0],
                    vec![3, 1],
                    vec![4, 0],
                ],
            ),
            (vec![vec![1]], vec![vec![0, 0]]),
        ];

        for (input, want) in data.into_iter() {
            let mut want = want.clone();
            want.iter_mut().for_each(|x| x.sort());
            want.sort();
            let mut got = Solution::pacific_atlantic(input);
            got.iter_mut().for_each(|x| x.sort());
            got.sort();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_num_islands() {
        let data = vec![
            (
                vec![
                    vec!["1", "1", "1", "1", "0"],
                    vec!["1", "1", "0", "1", "0"],
                    vec!["1", "1", "0", "0", "0"],
                    vec!["0", "0", "0", "0", "0"],
                ],
                1,
            ),
            (
                vec![
                    vec!["1", "1", "0", "0", "0"],
                    vec!["1", "1", "0", "0", "0"],
                    vec!["0", "0", "1", "0", "0"],
                    vec!["0", "0", "0", "1", "1"],
                ],
                3,
            ),
        ];

        let data = data
            .into_iter()
            .map(|(a, b)| {
                (
                    a.into_iter()
                        .map(|x| x.into_iter().map(|z| z.chars().next().unwrap()).collect())
                        .collect(),
                    b,
                )
            })
            .collect::<Vec<(Vec<Vec<char>>, i32)>>();

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::num_islands(input), want);
        }
    }

    #[test]
    fn test_longest_consecutive() {
        let data = vec![
            (vec![100, 4, 200, 1, 3, 2], 4),
            (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::longest_consecutive(input), want);
        }
    }
}
