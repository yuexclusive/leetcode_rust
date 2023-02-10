#![allow(dead_code)]
#![allow(unused_macros)]

extern crate test;

struct Solution;

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

    fn add(&mut self, val: i32) -> &mut Self {
        if let Some(ref mut v) = self.next {
            return v.add(val);
        }
        self.next = Some(Box::new(Self::new(val)));
        self
    }

    fn from_slice(slice: &[i32]) -> Self {
        *Self::from_slice_r(slice).unwrap()
    }

    fn from_slice_r(slice: &[i32]) -> Option<Box<Self>> {
        if slice.is_empty() {
            return None;
        }
        Some(Box::new(Self {
            val: slice[0],
            next: Self::from_slice_r(&slice[1..]),
        }))
    }

    fn to_vec(head: &mut Option<Box<Self>>) -> Vec<i32> {
        let mut res = vec![];
        Self::to_vec_r(head, &mut res);
        res
    }
    fn to_vec_r(head: &mut Option<Box<Self>>, vec: &mut Vec<i32>) {
        match head {
            None => (),
            Some(node) => {
                vec.push(node.val);
                Self::to_vec_r(&mut node.next, vec)
            }
        }
    }

    fn reverse(&mut self, pre: Option<Box<Self>>) -> Self {
        let pre = Self {
            val: self.val,
            next: pre,
        };
        if let Some(ref mut v) = self.next {
            return v.reverse(Some(Box::new(pre)));
        }
        pre
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        match head.as_deref_mut() {
            None => None,
            Some(v) => Some(Box::new(v.reverse(None))),
        }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (Some(l1), Some(l2)) => {
                if l1.val < l2.val {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Self::merge_two_lists(l1.next, Some(l2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Self::merge_two_lists(Some(l1), l2.next),
                    }))
                }
            }
        }
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists
            .into_iter()
            .filter(|x| !x.is_none())
            .map(|x| x.unwrap())
            .collect::<Vec<Box<ListNode>>>();

        if lists.is_empty() {
            return None;
        }

        let mut end = lists.len() - 1;

        for k in (0..=(end / 2)).rev() {
            Self::rise(&mut lists, k, end)
        }

        let mut res = ListNode::new(0);

        let mut res_ref = &mut res;

        loop {
            let target = lists[0].clone();
            res_ref.next = Some(Box::new(ListNode {
                val: target.val,
                next: None,
            }));
            res_ref = res_ref.next.as_deref_mut().unwrap();
            match target.next {
                Some(v) => {
                    lists[0] = v;
                }
                None => {
                    if end.checked_sub(1).is_none() {
                        break;
                    }
                    lists.swap(0, end);
                    end -= 1;
                }
            }
            Self::rise(&mut lists, 0, end)
        }

        res.next
    }

    fn rise(lists: &mut Vec<Box<ListNode>>, k: usize, end: usize) {
        let mut k = k;
        loop {
            let mut i = 2 * k + 1;
            if i > end {
                break;
            }
            if i < end && lists[i].val > lists[i + 1].val {
                i += 1
            }
            if lists[k].val <= lists[i].val {
                break;
            }
            lists.swap(k, i);
            k = i;
        }
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut vec = ListNode::to_vec(&mut head);
        vec.remove(vec.len() - n as usize);
        ListNode::from_slice_r(&vec)
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        use std::cmp::Ordering;
        let vec = ListNode::to_vec(head);
        if vec.is_empty() {
            return;
        }
        let mut i = 0;
        let mut x = head;
        'a: loop {
            x.as_deref_mut().unwrap().val = vec[i];
            x = &mut x.as_deref_mut().unwrap().next;
            i = match i.cmp(&(vec.len() / 2)) {
                Ordering::Equal => break 'a,
                Ordering::Greater => vec.len() - i,
                Ordering::Less => vec.len() - i - 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! list {
    ($($item:expr),*) => {
        {
            let mut vec = Vec::new();
            $(
               vec.push($item);
            )*
            ListNode::from_slice(&vec)
        }
    };
}
    use super::*;
    #[test]
    fn test_reverse_list() {
        let data = vec![
            (
                Some(Box::new(list![1, 2, 3, 4, 5])),
                Some(Box::new(list![5, 4, 3, 2, 1])),
            ),
            (Some(Box::new(list![1, 2])), Some(Box::new(list![2, 1]))),
            (None, None),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::reverse_list(input), want)
        }
    }

    #[test]
    fn test_merge_two_lists() {
        let data = vec![
            (
                Some(Box::new(list![1, 2, 4])),
                Some(Box::new(list![1, 3, 4])),
                Some(Box::new(list![1, 1, 2, 3, 4, 4])),
            ),
            (None, None, None),
            (None, Some(Box::new(list![1])), Some(Box::new(list![1]))),
        ];

        for (l1, l2, want) in data.into_iter() {
            assert_eq!(Solution::merge_two_lists(l1, l2), want)
        }
    }

    #[test]
    fn test_merge_k_lists() {
        let data = vec![
            (
                vec![
                    Some(Box::new(list![1, 4, 5])),
                    Some(Box::new(list![1, 3, 4])),
                    Some(Box::new(list![2, 6])),
                ],
                Some(Box::new(list![1, 1, 2, 3, 4, 4, 5, 6])),
            ),
            (vec![], None),
            (vec![None], None),
            (
                vec![
                    Some(Box::new(list![-8, -7, -7, -5, 1, 1, 3, 4])),
                    Some(Box::new(list![-2])),
                    Some(Box::new(list![-10, -10, -7, 0, 1, 3])),
                    Some(Box::new(list![2])),
                ],
                Some(Box::new(list![
                    -10, -10, -8, -7, -7, -7, -5, -2, 0, 1, 1, 1, 2, 3, 3, 4
                ])),
            ),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::merge_k_lists(input), want)
        }
    }

    #[test]
    fn test_remove_nth_from_end() {
        let data = vec![
            (
                Some(Box::new(list![1, 2, 3, 4, 5])),
                2,
                Some(Box::new(list![1, 2, 3, 5])),
            ),
            (Some(Box::new(list![1])), 1, None),
            (Some(Box::new(list![1, 2])), 1, Some(Box::new(list![1]))),
        ];

        for (input, n, want) in data.into_iter() {
            assert_eq!(Solution::remove_nth_from_end(input, n), want)
        }
    }

    #[test]
    fn test_reorder_list() {
        let data = vec![
            (list![1, 2, 3, 4], list![1, 4, 2, 3]),
            (list![1, 2, 3, 4, 5], list![1, 5, 2, 4, 3]),
        ];

        for (input, want) in data.into_iter() {
            let mut input = Some(Box::new(input));
            Solution::reorder_list(&mut input);
            assert_eq!(input, Some(Box::new(want)));
        }
    }
}
