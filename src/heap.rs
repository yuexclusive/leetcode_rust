#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl ListNode {
    fn add(&mut self, val: i32) -> &mut Self {
        match &mut self.next {
            Some(v) => {
                v.add(val);
            }
            None => self.next = Some(Box::new(Self::new(val))),
        };
        self
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists
            .into_iter()
            .filter(|x| !x.is_none())
            .map(|x| x.unwrap())
            .collect::<Vec<Box<ListNode>>>();

        let l = lists.len();

        if l == 0 {
            return None;
        }

        let mut end = l - 1;
        for k in (0..=(end / 2)).rev() {
            Self::merge_k_lists_rise(&mut lists, k, end);
        }

        let mut res = ListNode::new(0);

        loop {
            let head = lists.get_mut(0).unwrap();
            res.add(head.val);
            match &head.next {
                None => {
                    if end == 0 {
                        break;
                    }
                    lists.swap(0, end);
                    end -= 1;
                    Self::merge_k_lists_rise(&mut lists, 0, end);
                }
                Some(v) => {
                    *head = v.clone();
                    Self::merge_k_lists_rise(&mut lists, 0, end);
                }
            };
        }
        res.next
    }

    fn merge_k_lists_rise(slice: &mut Vec<Box<ListNode>>, k: usize, end: usize) {
        let mut k = k;
        loop {
            let mut i = 2 * k + 1;
            if i > end {
                break;
            }
            if i < end && slice[i + 1].val < slice[i].val {
                i += 1
            }

            if slice[k].val <= slice[i].val {
                break;
            }
            slice.swap(i, k);

            k = i;
        }
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let l = nums.len();
        if l == 0 || l == 1 {
            return nums;
        }
        let mut slice = nums
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut hm, v| {
                hm.entry(v).and_modify(|x| *x += 1).or_insert(1);
                hm
            })
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<Vec<(i32, i32)>>();

        let l = slice.len();
        let mut end = l - 1;
        let mut k = k;
        for k in (0..=(end / 2)).rev() {
            Self::top_k_frequent_rise(&mut slice, k, end);
        }
        let mut res = vec![];

        loop {
            res.push(slice[0].0);
            slice.swap(0, end);
            if k == 1 || end == 0 {
                break;
            }
            end -= 1;
            k -= 1;

            Self::top_k_frequent_rise(&mut slice, 0, end);
        }
        res
    }

    fn top_k_frequent_rise(slice: &mut [(i32, i32)], k: usize, end: usize) {
        let mut k = k;
        loop {
            let mut i = 2 * k + 1;
            if i > end {
                break;
            }
            if i < end && slice[i + 1].1 > slice[i].1 {
                i += 1
            }
            if slice[k].1 >= slice[i].1 {
                break;
            }
            slice.swap(i, k);
            k = i
        }
    }
}

struct MedianFinder {
    data: std::cell::RefCell<(Vec<i32>, Vec<i32>)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    fn rise(slice: &mut Vec<i32>, k: usize, end: usize, c: std::cmp::Ordering) {
        let mut k = k;
        loop {
            let mut i = 2 * k + 1;
            if i > end {
                break;
            }
            if i < end && slice[i + 1].cmp(&slice[i]) == c {
                i += 1
            }
            if slice[k] == slice[i] || slice[k].cmp(&slice[i]) == c {
                break;
            }
            slice.swap(k, i);
            k = i
        }
    }

    fn push(slice: &mut Vec<i32>, val: i32, c: std::cmp::Ordering) {
        slice.push(val);
        let end = slice.len() - 1;
        if end == 0 {
            return;
        }
        let mut k = (end - 1) / 2;
        loop {
            Self::rise(slice, k, end, c);
            if k == 0 {
                break;
            }
            k = (k - 1) / 2;
        }
    }

    fn pop(slice: &mut Vec<i32>, c: std::cmp::Ordering) -> i32 {
        let res = slice[0];
        let l = slice.len();
        if l > 1 {
            let end = l - 1;
            slice.swap(0, end);
            Self::rise(slice, 0, end - 1, c);
        }
        slice.pop().unwrap()
    }

    fn add_num(&self, num: i32) {
        let mut tuple = self.data.borrow_mut();
        let (l1, l2) = (tuple.0.len(), tuple.1.len());
        if l1 == 0 && l2 == 0 {
            Self::push(&mut tuple.0, num, std::cmp::Ordering::Greater);
        } else {
            if tuple.0[0] > num {
                Self::push(&mut tuple.0, num, std::cmp::Ordering::Greater);
            } else {
                Self::push(&mut tuple.1, num, std::cmp::Ordering::Less);
            }
        }
        let (l1, l2) = (tuple.0.len(), tuple.1.len());
        if l1.checked_sub(l2) == Some(2) {
            let val = Self::pop(&mut tuple.0, std::cmp::Ordering::Greater);
            Self::push(&mut tuple.1, val, std::cmp::Ordering::Less);
        } else if l2.checked_sub(l1) == Some(1) {
            let val = Self::pop(&mut tuple.1, std::cmp::Ordering::Less);
            Self::push(&mut tuple.0, val, std::cmp::Ordering::Greater)
        }
    }

    fn find_median(&self) -> f64 {
        let max = &self.data.borrow().0;
        let min = &self.data.borrow().1;
        if max.len() > min.len() {
            return max[0] as f64;
        }
        return (max[0] as f64 + min[0] as f64) / 2 as f64;
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! list {
        ($($val:expr),*) => {
            {
                let mut res = ListNode::new(0);
                $(
                    res.add($val);
                )*
                res.next
            }
        };
    }

    #[test]
    fn test_merge_k_lists() {
        let data = vec![
            (
                vec![list![1, 4, 5], list![1, 3, 4], list![2, 6]],
                list![1, 1, 2, 3, 4, 4, 5, 6],
            ),
            (vec![], list![]),
            (vec![list![]], list![]),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::merge_k_lists(input), want);
        }
    }

    #[test]
    fn test_top_k_frequent() {
        let data = vec![
            (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
            (vec![1], 1, vec![1]),
        ];
        for (input, k, want) in data.into_iter() {
            assert_eq!(Solution::top_k_frequent(input, k), want);
        }
    }

    #[test]
    fn test_find_median() {
        let mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);

        let data = vec![
            (
                vec![
                    vec![12],
                    vec![],
                    vec![10],
                    vec![],
                    vec![13],
                    vec![],
                    vec![11],
                    vec![],
                    vec![5],
                    vec![],
                    vec![15],
                    vec![],
                    vec![1],
                    vec![],
                    vec![11],
                    vec![],
                    vec![6],
                    vec![],
                    vec![17],
                    vec![],
                    vec![14],
                    vec![],
                    vec![8],
                    vec![],
                    vec![17],
                    vec![],
                    vec![6],
                    vec![],
                    vec![4],
                    vec![],
                    // vec![16],
                    // vec![],
                    // vec![8],
                    // vec![],
                    // vec![10],
                    // vec![],
                    // vec![2],
                    // vec![],
                    // vec![12],
                    // vec![],
                    // vec![0],
                    // vec![],
                ],
                vec![
                    12.00000, 11.00000, 12.00000, 11.50000, 11.00000, 11.50000, 11.00000, 11.00000,
                    11.00000, 11.00000, 11.00000, 11.00000, 11.00000, 11.00000, 11.00000,
                ],
            ),
            (
                vec![
                    vec![40],
                    vec![],
                    vec![12],
                    vec![],
                    vec![16],
                    vec![],
                    vec![14],
                    vec![],
                    vec![35],
                    vec![],
                    vec![19],
                    vec![],
                    vec![34],
                    vec![],
                    vec![35],
                    vec![],
                    vec![28],
                    vec![],
                    vec![35],
                    vec![],
                    vec![26],
                    vec![],
                    vec![6],
                    vec![],
                    vec![8],
                    vec![],
                    vec![2],
                    vec![],
                    vec![14],
                    vec![],
                    vec![25],
                    vec![],
                    vec![25],
                    vec![],
                    vec![4],
                    vec![],
                    vec![33],
                    vec![],
                    vec![18],
                    vec![],
                    vec![10],
                    vec![],
                    vec![14],
                    vec![],
                    vec![27],
                    vec![],
                    vec![3],
                    vec![],
                    vec![35],
                    vec![],
                    vec![13],
                    vec![],
                    vec![24],
                    vec![],
                    // vec![27],
                    // vec![],
                    // vec![14],
                    // vec![],
                    // vec![5],
                    // vec![],
                    // vec![0],
                    // vec![],
                    // vec![38],
                    // vec![],
                    // vec![19],
                    // vec![],
                    // vec![25],
                    // vec![],
                    // vec![11],
                    // vec![],
                    // vec![14],
                    // vec![],
                    // vec![31],
                    // vec![],
                    // vec![30],
                    // vec![],
                    // vec![11],
                    // vec![],
                    // vec![31],
                    // vec![],
                    // vec![0],
                    // vec![],
                ],
                vec![
                    40.00000, 26.00000, 16.00000, 15.00000, 16.00000, 17.50000, 19.00000, 26.50000,
                    28.00000, 31.00000, 28.00000, 27.00000, 26.00000, 22.50000, 19.00000, 22.00000,
                    25.00000, 22.00000, 25.00000, 22.00000, 19.00000, 18.50000, 19.00000, 18.50000,
                    19.00000, 18.50000,
                    19.00000,
                    // 21.50000, 19.00000, 18.50000, 18.00000, 18.50000,
                    // 19.00000, 19.00000, 19.00000, 18.50000, 19.00000, 19.00000, 19.00000, 19.00000,
                    // 19.00000,
                ],
            ),
        ];
        for (input, want) in data.into_iter() {
            let mut res = vec![];
            let mf = MedianFinder::new();
            for item in input {
                if item.len() == 1 {
                    mf.add_num(item[0]);
                    // println!("val: {}, {:?}", item[0], mf.data);
                } else {
                    res.push(mf.find_median())
                }
            }
            assert_eq!(res, want);
        }
    }
}
