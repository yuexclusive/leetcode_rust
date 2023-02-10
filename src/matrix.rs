#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        use std::collections::HashSet;
        let (hs1, hs2) = matrix.iter().enumerate().fold(
            (HashSet::new(), HashSet::new()),
            |(mut hs1, mut hs2), (i, x)| {
                x.iter()
                    .enumerate()
                    .filter(|(_, &v)| v == 0)
                    .for_each(|(j, _)| {
                        hs1.insert(i);
                        hs2.insert(j);
                    });
                (hs1, hs2)
            },
        );
        let l = matrix[0].len();
        hs1.iter().for_each(|&i| {
            (0..l).for_each(|j| {
                matrix[i][j] = 0;
            });
        });
        let l = matrix.len();
        (0..l).for_each(|i| {
            hs2.iter().for_each(|&j| {
                matrix[i][j] = 0;
            });
        });
    }
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return vec![];
        }
        let mut total = matrix.len() * matrix[0].len();
        let steps = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut map = vec![vec![false; matrix[0].len()]; matrix.len()];
        let (mut i, mut j) = (0_i32, 0_i32);
        let mut step_index = 0;
        let mut res = vec![];
        loop {
            res.push(matrix[i as usize][j as usize]);
            map[i as usize][j as usize] = true;
            total -= 1;
            if total == 0 {
                break;
            }
            let [a, b] = steps[step_index];
            let (n_i, n_j) = (i + a, j + b);
            if n_i >= 0
                && n_j >= 0
                && (n_i as usize) < matrix.len()
                && (n_j as usize) < matrix[0].len()
                && !map[n_i as usize][n_j as usize]
            {
                i = n_i;
                j = n_j;
                continue;
            }
            step_index = (step_index + 1) % steps.len();
            let [a, b] = steps[step_index];
            i = i + a;
            j = j + b;
        }
        res
    }
}

impl Solution {
    // 3,0 -> 0,0
    // 3,1 -> 1,0
    // 3,2 -> 2,0
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let copy = matrix
            .iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .map(|(j, _)| matrix[matrix.len() - j - 1][i])
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>();

        matrix
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| x.iter_mut().enumerate().for_each(|(j, v)| *v = copy[i][j]));
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();

        board
            .iter()
            .enumerate()
            .try_fold(
                vec![vec![false; board[0].len()]; board.len()],
                |mut map, (i, x)| {
                    if let std::ops::ControlFlow::Break(()) =
                        x.iter().enumerate().try_for_each(|(j, _)| {
                            if Self::exist_walk(&board, &mut map, &word, 0, Some(i), Some(j)) {
                                return std::ops::ControlFlow::Break(());
                            }

                            std::ops::ControlFlow::Continue(())
                        })
                    {
                        return None;
                    }
                    Some(map)
                },
            )
            .is_none()
    }

    fn exist_walk(
        board: &Vec<Vec<char>>,
        map: &mut Vec<Vec<bool>>,
        word: &[char],
        index: usize,
        i: Option<usize>,
        j: Option<usize>,
    ) -> bool {
        if index == word.len() {
            return true;
        }
        if i.is_none() || j.is_none() {
            return false;
        }

        let (i, j) = (i.unwrap(), j.unwrap());

        if i == board.len() || j == board[0].len() || word[index] != board[i][j] || map[i][j] {
            return false;
        }

        map[i][j] = true;

        let index = index + 1;

        let res = Self::exist_walk(board, map, word, index, i.checked_add(1), Some(j))
            || Self::exist_walk(board, map, word, index, i.checked_sub(1), Some(j))
            || Self::exist_walk(board, map, word, index, Some(i), j.checked_add(1))
            || Self::exist_walk(board, map, word, index, Some(i), j.checked_sub(1));

        map[i][j] = false;

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_zeroes() {
        let data = vec![
            (
                vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
                vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]],
            ),
            (
                vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
                vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
            ),
        ];
        for (input, want) in data.into_iter() {
            let mut input = input;
            Solution::set_zeroes(&mut input);
            assert_eq!(input, want);
        }
    }

    #[test]
    fn test_spiral_order() {
        let data = vec![
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            ),
            (
                vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
                vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            ),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::spiral_order(input), want);
        }
    }

    #[test]
    fn test_rotate() {
        let data = vec![
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
            ),
            (
                vec![
                    vec![5, 1, 9, 11],
                    vec![2, 4, 8, 10],
                    vec![13, 3, 6, 7],
                    vec![15, 14, 12, 16],
                ],
                vec![
                    vec![15, 13, 2, 5],
                    vec![14, 3, 4, 1],
                    vec![12, 6, 8, 9],
                    vec![16, 7, 10, 11],
                ],
            ),
        ];

        for (input, want) in data.into_iter() {
            let mut input = input;
            Solution::rotate(&mut input);
            assert_eq!(input, want);
        }
    }

    #[test]
    fn test_exist() {
        let data = vec![
            (
                vec![
                    vec!["A", "B", "C", "E"],
                    vec!["S", "F", "C", "S"],
                    vec!["A", "D", "E", "E"],
                ],
                "ABCCED",
                true,
            ),
            (
                vec![
                    vec!["A", "B", "C", "E"],
                    vec!["S", "F", "C", "S"],
                    vec!["A", "D", "E", "E"],
                ],
                "SEE",
                true,
            ),
            (
                vec![
                    vec!["A", "B", "C", "E"],
                    vec!["S", "F", "C", "S"],
                    vec!["A", "D", "E", "E"],
                ],
                "ABCB",
                false,
            ),
            (
                vec![
                    vec!["C", "A", "A"],
                    vec!["A", "A", "A"],
                    vec!["B", "C", "D"],
                ],
                "AAB",
                true,
            ),
        ];

        for (board, word, want) in data.into_iter() {
            let board = board
                .into_iter()
                .map(|x| x.into_iter().map(|v| v.chars().next().unwrap()).collect())
                .collect::<Vec<Vec<char>>>();

            assert_eq!(Solution::exist(board, word.to_string()), want)
        }
    }
}
