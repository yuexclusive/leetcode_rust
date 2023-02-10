#![allow(dead_code)]
extern crate test;

pub struct Solution;

impl Solution {
    pub fn bubble_sort(slice: &mut [i32]) {
        // let mut number = 0;
        let mut length = slice.len();
        let mut move_forward = true;
        while move_forward && length > 1 {
            move_forward = false;
            for i in 0..length - 1 {
                if slice[i] > slice[i + 1] {
                    move_forward = true;
                    slice.swap(i, i + 1);
                    // number += 1;
                }
            }
            length -= 1;
        }

        // println!("bubble sort number: {}", number)
    }

    pub fn select_sort(slice: &mut [i32]) {
        // let mut number = 0;
        let length = slice.len();
        if length <= 1 {
            return;
        }
        for i in 0..length - 1 {
            for j in i + 1..length {
                if slice[i] > slice[j] {
                    slice.swap(i, j);
                    // number += 1;
                }
            }
        }

        // println!("select sort number: {}", number)
    }

    pub fn insert_sort(slice: &mut [i32]) {
        // let mut number = 0;
        let length = slice.len();
        if length <= 1 {
            return;
        }
        for i in 1..length {
            for j in (1..=i).rev() {
                if slice[j] < slice[j - 1] {
                    slice.swap(j, j - 1);
                    // number += 1;
                } else {
                    break;
                }
            }
        }

        // println!("insert sort number: {}", number)
    }

    pub fn hill_sort(slice: &mut [i32]) {
        // let mut number = 0;
        let length = slice.len();
        if length <= 1 {
            return;
        }
        let mut i = length / 2;
        while i >= 1 {
            for j in 0..i {
                for k in j + i..length {
                    for l in (j + i..=k).rev() {
                        if slice[l] < slice[l - i] {
                            // number += 1;
                            slice.swap(l, l - i)
                        } else {
                            break;
                        }
                    }
                }
            }
            i /= 2;
        }

        // println!("hill sort number: {}", number)
    }

    pub fn quick_sort(slice: &mut [i32], left: usize, right: usize) {
        if left >= right {
            return;
        }
        let (mut start, mut end, mut flag) = (left, right, left);
        while start < end {
            while start < end {
                if slice[flag] > slice[end] {
                    slice.swap(flag, end);
                    flag = end;
                    break;
                } else {
                    end -= 1;
                }
            }
            while start < end {
                if slice[start] > slice[flag] {
                    slice.swap(flag, start);
                    flag = start;
                    break;
                } else {
                    start += 1;
                }
            }
        }
        if flag > 0 {
            Self::quick_sort(slice, left, flag - 1);
        }
        Self::quick_sort(slice, flag + 1, right);
    }

    pub fn merge_sort(slice: &mut [i32]) {
        let res = Self::merge_sort_r(slice);
        slice.copy_from_slice(&res);
    }

    fn merge_sort_r(slice: &[i32]) -> Vec<i32> {
        let length = slice.len();
        if length <= 1 {
            return slice.to_vec();
        }
        let mid = length / 2;
        Self::merge(
            &Self::merge_sort_r(&slice[..mid]),
            &Self::merge_sort_r(&slice[mid..]),
        )
    }
    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        let mut res = Vec::with_capacity(left.len() + right.len());
        let (mut p1, mut p2) = (0, 0);
        while p1 < left.len() && p2 < right.len() {
            if left[p1] < right[p2] {
                res.push(left[p1]);
                p1 += 1;
                continue;
            }
            res.push(right[p2]);
            p2 += 1;
        }
        res.extend(left[p1..].to_vec());
        res.extend(right[p2..].to_vec());
        res
    }

    fn heap_sort(slice: &mut [i32]) {
        let length = slice.len();
        if length <= 1 {
            return;
        }
        let mut end = length - 1;
        for k in (0..=(end / 2)).rev() {
            Self::heap_sort_rise(slice, k, end);
        }

        while end > 0 {
            slice.swap(0, end);
            end -= 1;
            Self::heap_sort_rise(slice, 0, end);
        }
    }

    fn heap_sort_rise(slice: &mut [i32], k: usize, end: usize) {
        let mut k = k;
        loop {
            let mut i = 2 * k + 1;
            if i > end {
                break;
            }
            if i < end && slice[i + 1] > slice[i] {
                i += 1
            }
            if slice[k] >= slice[i] {
                break;
            }
            slice.swap(i, k);
            k = i
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_data() -> Vec<(Vec<i32>, Vec<i32>)> {
        let vec = vec![
            (
                vec![4, 6, 3, 7, 2, 5, 1, 100, 0, 12, 13, 11],
                vec![0, 1, 2, 3, 4, 5, 6, 7, 11, 12, 13, 100],
            ),
            // (vec![1], vec![1]),
            // (vec![], vec![]),
            // (vec![2, 1], vec![1, 2]),
        ];
        vec
    }

    #[test]
    fn test_bubble_sort() {
        for (input, want) in get_data() {
            let mut input = input;
            Solution::bubble_sort(&mut input);
            assert_eq!(input, want);
        }
    }

    #[test]
    fn test_select_sort() {
        for (input, want) in get_data() {
            let mut input = input;
            Solution::select_sort(&mut input);
            assert_eq!(input, want);
        }
    }

    #[test]
    fn test_insert_sort() {
        for (input, want) in get_data() {
            let mut input = input;
            Solution::insert_sort(&mut input);
            assert_eq!(input, want);
        }
    }

    #[test]
    fn test_hill_sort() {
        for (input, want) in get_data() {
            let mut input = input;
            Solution::hill_sort(&mut input);
            assert_eq!(input, want);
        }
    }

    #[test]
    fn test_quick_sort() {
        for (input, want) in get_data() {
            let l = input.len();
            if l <= 1 {
                return;
            }
            let mut input = input;
            Solution::quick_sort(&mut input, 0, l - 1);
            assert_eq!(input, want);
        }
    }

    #[test]
    fn test_merge_sort() {
        for (input, want) in get_data() {
            let mut input = input;
            Solution::merge_sort(&mut input);
            assert_eq!(input, want);
        }
    }

    #[test]
    fn test_heap_sort() {
        for (input, want) in get_data() {
            let mut input = input;
            Solution::heap_sort(&mut input);
            assert_eq!(input, want);
        }
    }
}
