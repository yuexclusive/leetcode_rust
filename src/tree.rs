#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn add_left(&mut self, val: i32) {
        self.left = Some(Rc::new(RefCell::new(Self::new(val))))
    }

    fn add_right(&mut self, val: i32) {
        self.right = Some(Rc::new(RefCell::new(Self::new(val))))
    }

    fn pre_order(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match node {
            None => vec![],
            Some(v) => {
                let mut res = vec![v.borrow().val];
                res.extend(Self::pre_order(v.borrow().left.clone()));
                res.extend(Self::pre_order(v.borrow().right.clone()));
                res
            }
        }
    }

    fn pre_order_non_recursive(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut node = node;
        while node.is_some() || !stack.is_empty() {
            while let Some(v) = node {
                res.push(v.borrow().val);
                stack.push(v.clone());
                node = v.borrow().left.clone();
            }

            let tail = stack.last().map(|x| x.clone());
            node = tail.as_deref().unwrap().borrow().right.clone();
            stack.pop();
        }
        res
    }

    fn in_order(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match node {
            None => vec![],
            Some(v) => {
                let mut res = vec![];
                res.extend(Self::in_order(v.borrow().left.clone()));
                res.push(v.borrow().val);
                res.extend(Self::in_order(v.borrow().right.clone()));
                res
            }
        }
    }

    fn in_order_non_recursive(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut node = node;
        while node.is_some() || !stack.is_empty() {
            while let Some(v) = node {
                stack.push(v.clone());
                node = v.borrow().left.clone();
            }

            let tail = stack.last().map(|x| x.clone());
            res.push(tail.as_deref().unwrap().borrow().val);
            node = tail.as_deref().unwrap().borrow().right.clone();
            stack.pop();
        }
        res
    }

    fn post_order(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match node {
            None => vec![],
            Some(v) => {
                let mut res = vec![];
                res.extend(Self::post_order(v.borrow().left.clone()));
                res.extend(Self::post_order(v.borrow().right.clone()));
                res.push(v.borrow().val);
                res
            }
        }
    }

    fn post_order_non_recursive(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut node = node;
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;
        while node.is_some() || !stack.is_empty() {
            while let Some(v) = node {
                stack.push(v.clone());
                node = v.borrow().left.clone();
            }

            let tail = stack.last().map(|x| x.clone());

            if (tail.as_deref().unwrap().borrow().left.is_none()
                && tail.as_deref().unwrap().borrow().right.is_none())
                || (tail.as_deref().unwrap().borrow().right.is_none()
                    && tail.as_deref().unwrap().borrow().left == pre)
                || tail.as_deref().unwrap().borrow().right == pre
            {
                pre = tail.as_ref().map(|x| x.clone());
                res.push(tail.as_deref().unwrap().borrow().val);
                stack.pop();
            } else {
                node = tail.as_deref().unwrap().borrow().right.clone();
            }
        }
        res
    }

    fn level_order(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut queue = vec![node];
        while !queue.is_empty() {
            let head = queue.first().unwrap().as_deref().map(|x| x.clone());
            match head {
                Some(v) => {
                    res.push(v.borrow().val);
                    queue.push(v.borrow().left.clone());
                    queue.push(v.borrow().right.clone());
                }
                None => {}
            }
            queue.remove(0);
        }

        res
    }

    fn level_order_2(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut res = vec![];
        let mut queue = vec![node];
        while !queue.is_empty() {
            let head = queue.first().unwrap().as_deref().map(|x| x.clone());
            match head {
                Some(v) => {
                    res.push(Some(v.borrow().val));
                    queue.push(v.borrow().left.clone());
                    queue.push(v.borrow().right.clone());
                }
                None => {
                    res.push(None);
                }
            }
            queue.remove(0);
        }

        res
    }

    fn from_level_order(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() || vec.first().is_none() {
            return None;
        }
        let vec = vec
            .iter()
            .map(|x| match x {
                Some(v) => Some(Rc::new(RefCell::new(Self::new(*v)))),
                None => None,
            })
            .collect::<Vec<_>>();

        if vec.len() == 1 {
            return vec[0].clone();
        }

        let (mut a, mut b, mut c) = (0, 1, 2);

        while b < vec.len() {
            vec[a].as_deref().unwrap().borrow_mut().left = vec[b].as_ref().map(|x| x.clone());
            if c < vec.len() {
                vec[a].as_deref().unwrap().borrow_mut().right = vec[c].as_ref().map(|x| x.clone());
            }
            b += 2;
            c += 2;
            a += 1;
            while a < vec.len() && vec[a].is_none() {
                a += 1;
            }
        }

        vec[0].clone()
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                Self::max_depth(n.borrow().left.clone())
                    .max(Self::max_depth(n.borrow().right.clone()))
                    + 1
            }
        }
    }
}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (&p, &q) {
            (Some(a), Some(b)) => {
                a.borrow().val == b.borrow().val
                    && Self::is_same_tree(a.borrow().left.clone(), b.borrow().left.clone())
                    && Self::is_same_tree(a.borrow().right.clone(), b.borrow().right.clone())
            }
            _ => p == q,
        }
    }
}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(n) => {
                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();
                n.borrow_mut().right = Self::invert_tree(left);
                n.borrow_mut().left = Self::invert_tree(right);
                Some(n)
            }
        }
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = i32::MIN;
        Self::max_sum(root, &mut res);
        res
    }

    fn max_sum(node: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        match node {
            None => 0,
            Some(v) => {
                let v = v.borrow();
                let left = Self::max_sum(v.left.clone(), res).max(0);
                let right = Self::max_sum(v.right.clone(), res).max(0);
                *res = (*res).max(v.val + left + right);
                v.val + left.max(right)
            }
        }
    }
}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut level_index = 0_usize;
        let mut level_capacity = 1;
        let mut level_capacity_next = 0;
        let mut index = 0;
        let mut queue = vec![root];
        while !queue.is_empty() {
            let head = queue.first().unwrap().as_ref().map(|x| x.clone());
            if let Some(v) = head {
                if res.len() == level_index {
                    res.push(vec![]);
                }
                res[level_index].push(v.borrow().val);
                queue.push(v.borrow().left.clone());
                queue.push(v.borrow().right.clone());
                level_capacity_next += 2;
            }
            index += 1;
            if level_capacity == index {
                index = 0;
                level_index += 1;
                level_capacity = level_capacity_next;
                level_capacity_next = 0;
            }
            queue.remove(0);
        }
        res
    }
}

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut queue = vec![root];
        let mut vec = vec![];
        while !queue.is_empty() {
            let head = queue.first().unwrap().as_ref().map(|x| x.clone());
            match head {
                Some(v) => {
                    vec.push(v.borrow().val.to_string());
                    queue.push(v.borrow().left.clone());
                    queue.push(v.borrow().right.clone());
                }
                None => {
                    vec.push("null".to_string());
                }
            };
            queue.remove(0);
        }
        vec.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let vec = data
            .split(",")
            .map(|x| match x {
                "null" => None,
                _ => Some(Rc::new(RefCell::new(TreeNode::new(x.parse().unwrap())))),
            })
            .collect::<Vec<_>>();

        let (mut a, mut b, mut c) = (0, 1, 2);
        while b < vec.len() {
            vec[a].as_deref().unwrap().borrow_mut().left = vec[b].as_ref().map(|x| x.clone());
            if c < vec.len() {
                vec[a].as_deref().unwrap().borrow_mut().right = vec[c].as_ref().map(|x| x.clone());
            }
            b += 2;
            c += 2;
            a += 1;
            while a < vec.len() && vec[a].is_none() {
                a += 1;
            }
        }

        vec[0].clone()
    }
}

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (_, None) => true,
            (None, _) => false,
            (Some(a), Some(b)) => {
                Self::is_same_node(Some(a.clone()), Some(b.clone()))
                    || Self::is_subtree(a.borrow().left.clone(), Some(b.clone()))
                    || Self::is_subtree(a.borrow().right.clone(), Some(b.clone()))
            }
        }
    }
    fn is_same_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(t1), Some(t2)) => {
                t1.borrow().val == t2.borrow().val
                    && Self::is_same_node(t1.borrow().left.clone(), t2.borrow().left.clone())
                    && Self::is_same_node(t1.borrow().right.clone(), t2.borrow().right.clone())
            }
            _ => false,
        }
    }
}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let val = preorder[0];
        let mut res = TreeNode::new(val);

        let mid = inorder
            .iter()
            .enumerate()
            .find(|(_, &v)| v == val)
            .map(|x| x.0)
            .unwrap();

        res.left = Self::build_tree(preorder[1..(mid + 1)].to_vec(), inorder[..mid].to_vec());
        res.right = Self::build_tree(
            preorder[(mid + 1)..].to_vec(),
            inorder[(mid + 1)..].to_vec(),
        );

        Some(Rc::new(RefCell::new(res)))
    }
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut root = root;
        let mut stack = vec![];
        let mut pre: Option<i32> = None;
        while root.is_some() || !stack.is_empty() {
            while let Some(v) = root {
                stack.push(v.clone());
                root = v.borrow().left.clone();
            }
            let tail = stack.last().as_deref().map(|x| x.clone());
            let val = tail.as_deref().unwrap().borrow().val;
            if let Some(p) = pre {
                if p >= val {
                    return false;
                }
            }
            pre = Some(val);
            root = tail.as_deref().unwrap().borrow().right.clone();
            stack.pop();
        }
        true
    }
}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut root = root;
        let mut stack = vec![];
        let mut vec = vec![];
        while root.is_some() || !stack.is_empty() {
            while let Some(v) = root {
                vec.push(v.borrow().val);
                let end = vec.len() - 1;
                let mut k = end / 2;
                loop {
                    Self::rise(&mut vec, k, end);
                    if k == 0 {
                        break;
                    }
                    k = k / 2;
                }
                stack.push(v.clone());
                root = v.borrow().left.clone();
            }
            let tail = stack.last().as_deref().map(|x| x.clone());
            root = tail.as_deref().unwrap().borrow().right.clone();
            stack.pop();
        }
        let mut k = k;
        let mut end = vec.len() - 1;
        let mut res = 0;
        while k > 0 {
            res = vec[0];
            vec.swap(0, end);
            k -= 1;
            match end.checked_sub(1) {
                Some(v) => {
                    end = v;
                }
                None => {
                    break;
                }
            }
            Self::rise(&mut vec, 0, end);
        }
        res
    }

    fn rise(slice: &mut [i32], k: usize, end: usize) {
        let mut k = k;
        loop {
            let mut i = 2 * k + 1;
            if i > end {
                break;
            }
            if i < end && slice[i] > slice[i + 1] {
                i += 1;
            }
            if slice[k] <= slice[i] {
                break;
            }
            slice.swap(i, k);
            k = i;
        }
    }
}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        use std::cmp::Ordering;
        let (mut p, mut q) = (p, q);
        let (mut l, mut r) = (
            p.as_deref().unwrap().borrow().val,
            q.as_deref().unwrap().borrow().val,
        );
        if l > r {
            std::mem::swap(&mut p, &mut q);
            std::mem::swap(&mut l, &mut r);
        }
        let v = root.as_deref().unwrap().borrow().val;
        match (v.cmp(&l), v.cmp(&r)) {
            (Ordering::Greater, Ordering::Less) | (Ordering::Equal, _) | (_, Ordering::Equal) => {
                root
            }
            (Ordering::Less, _) => {
                Self::lowest_common_ancestor(root.as_deref().unwrap().borrow().right.clone(), p, q)
            }

            (_, Ordering::Greater) => {
                Self::lowest_common_ancestor(root.as_deref().unwrap().borrow().left.clone(), p, q)
            }
        }
    }
}

struct Trie {
    end: std::cell::RefCell<bool>,
    m: std::cell::RefCell<std::collections::HashMap<u8, Self>>,
}

impl Trie {
    fn new() -> Self {
        Self {
            end: Default::default(),
            m: Default::default(),
        }
    }

    fn insert(&self, word: String) {
        if word.is_empty() {
            *self.end.borrow_mut() = true;
            return;
        }
        let bytes = word.as_bytes();
        self.m
            .borrow_mut()
            .entry(bytes[0])
            .or_insert(Self::new())
            .insert(String::from_utf8_lossy(&bytes[1..]).to_string());
    }

    fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return *self.end.borrow();
        }
        let bytes = word.as_bytes();
        match self.m.borrow().get(&bytes[0]) {
            None => false,
            Some(v) => v.search(String::from_utf8_lossy(&bytes[1..]).to_string()),
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        if prefix.is_empty() {
            return true;
        }
        let bytes = prefix.as_bytes();
        match self.m.borrow().get(&bytes[0]) {
            None => false,
            Some(v) => v.starts_with(String::from_utf8_lossy(&bytes[1..]).to_string()),
        }
    }
}

struct WordDictionary {
    end: std::cell::RefCell<bool>,
    m: std::cell::RefCell<std::collections::HashMap<u8, Self>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            end: Default::default(),
            m: Default::default(),
        }
    }

    fn add_word(&self, word: String) {
        if word.is_empty() {
            *self.end.borrow_mut() = true;
            return;
        }
        let bytes = word.as_bytes();
        self.m
            .borrow_mut()
            .entry(bytes[0])
            .or_insert(Self::new())
            .add_word(String::from_utf8_lossy(&bytes[1..]).to_string());
    }

    fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return *self.end.borrow();
        }
        let bytes = word.as_bytes();
        let str = String::from_utf8_lossy(&bytes[1..]).to_string();
        if bytes[0] == b'.' {
            for (_, x) in self.m.borrow().iter() {
                if x.search(str.clone()) {
                    return true;
                }
            }
        }
        match self.m.borrow().get(&bytes[0]) {
            None => false,
            Some(v) => v.search(str),
        }
    }
}

use std::collections::HashMap;

#[derive(Default, Debug)]
struct WordTrie {
    end: RefCell<bool>,
    m: RefCell<HashMap<u8, Self>>,
}

impl WordTrie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&self, str: &str) {
        if str.is_empty() {
            *self.end.borrow_mut() = true;
            return;
        }
        let bs = str.as_bytes();
        self.m
            .borrow_mut()
            .entry(bs[0])
            .or_insert(Self::new())
            .insert(&String::from_utf8_lossy(&bs[1..]))
    }

    fn search(
        &self,
        board: &Vec<Vec<u8>>,
        m: &mut Vec<Vec<bool>>,
        i: Option<usize>,
        j: Option<usize>,
        path: &mut Vec<u8>,
        res: &mut Vec<String>,
    ) {
        if *self.end.borrow() {
            *self.end.borrow_mut() = false;
            res.push(String::from_utf8_lossy(path).to_string());
        }
        if i.is_none() || j.is_none() {
            return;
        }
        let (i, j) = (i.unwrap(), j.unwrap());
        if i == board.len() || j == board[0].len() {
            return;
        }
        if m[i][j] {
            return;
        }
        match self.m.borrow().get(&board[i][j]) {
            None => {
                return;
            }
            Some(v) => {
                m[i][j] = true;
                path.push(board[i][j]);
                v.search(board, m, i.checked_add(1), Some(j), path, res);
                v.search(board, m, i.checked_sub(1), Some(j), path, res);
                v.search(board, m, Some(i), j.checked_add(1), path, res);
                v.search(board, m, Some(i), j.checked_sub(1), path, res);
                path.pop();
                m[i][j] = false;
            }
        };
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let board: Vec<Vec<u8>> = board
            .into_iter()
            .map(|x| x.into_iter().map(|v| v as u8).collect())
            .collect();
        let trie = WordTrie::new();
        for word in words {
            trie.insert(&word);
        }
        let mut res = vec![];
        let mut path = vec![];
        let (x, y) = (board.len(), board[0].len());
        let mut m = vec![vec![false; y]; x];
        for i in 0..x {
            for j in 0..y {
                trie.search(&board, &mut m, Some(i), Some(j), &mut path, &mut res);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_mut)]
    use super::*;
    macro_rules! tree {
        ($($val: expr),*) => {
            {
                let mut vec = vec![];
                $(
                    vec.push(match stringify!($val){
                        "null" | "\"null\"" | "nil" | "\"nil\"" => None,
                        x => Some(x.parse::<i32>().unwrap()),
                    });
                )*
                TreeNode::from_level_order(vec)
            }
        };
    }

    fn get_root() -> TreeNode {
        let mut root = TreeNode::new(4);
        root.add_left(2);
        root.left.as_deref().unwrap().borrow_mut().add_left(1);
        root.left.as_deref().unwrap().borrow_mut().add_right(3);
        root.add_right(6);
        root.right.as_deref().unwrap().borrow_mut().add_left(5);
        root.right.as_deref().unwrap().borrow_mut().add_right(7);
        root
    }

    #[test]
    fn test_pre_order() {
        let root = get_root();

        let res = TreeNode::pre_order(Some(Rc::new(RefCell::new(root.clone()))));

        let res_non_recursive =
            TreeNode::pre_order_non_recursive(Some(Rc::new(RefCell::new(root))));

        assert_eq!(res, res_non_recursive)
    }
    #[test]
    fn test_in_order() {
        let root = get_root();

        let res = TreeNode::in_order(Some(Rc::new(RefCell::new(root.clone()))));

        let res_non_recursive = TreeNode::in_order_non_recursive(Some(Rc::new(RefCell::new(root))));

        assert_eq!(res, res_non_recursive)
    }

    #[test]
    fn test_post_order() {
        let root = get_root();

        let res = TreeNode::post_order(Some(Rc::new(RefCell::new(root.clone()))));

        let res_non_recursive =
            TreeNode::post_order_non_recursive(Some(Rc::new(RefCell::new(root))));

        assert_eq!(res, res_non_recursive)
    }

    #[test]
    fn test_level_order_self() {
        let root = get_root();

        let res = TreeNode::level_order(Some(Rc::new(RefCell::new(root.clone()))));

        assert_eq!(res, vec![4, 2, 6, 1, 3, 5, 7])
    }

    #[test]
    fn test_from_level_order() {
        let root = get_root();

        let res = tree![4, 2, 6, 1, 3, 5, 7];

        assert_eq!(res.unwrap().borrow().clone(), root);

        let res2 = tree![1, 2, null, 3, null, 4, null, 5];

        let mut root2 = TreeNode::new(1);
        root2.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root2.left.as_deref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root2
            .left
            .as_deref()
            .unwrap()
            .borrow_mut()
            .left
            .as_deref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(4))));

        root2
            .left
            .as_deref()
            .unwrap()
            .borrow_mut()
            .left
            .as_deref()
            .unwrap()
            .borrow_mut()
            .left
            .as_deref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        assert_eq!(Some(Rc::new(RefCell::new(root2))), res2);
    }

    #[test]
    fn test_max_depth() {
        let data = vec![
            (tree![3, 9, 20, null, null, 15, 7], 3),
            (tree![1, null, 2], 2),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::max_depth(input), want);
        }
    }

    #[test]
    fn test_is_same_tree() {
        let data = vec![
            (tree![1, 2, 3], tree![1, 2, 3], true),
            (tree![1, 2], tree![1, null, 2], false),
            (tree![1, 2, 1], tree![1, 1, 2], false),
        ];
        for (a, b, want) in data.into_iter() {
            assert_eq!(Solution::is_same_tree(a, b), want);
        }
    }

    #[test]
    fn test_inverse_tree() {
        let data = vec![
            (tree![4, 2, 7, 1, 3, 6, 9], tree![4, 7, 2, 9, 6, 3, 1]),
            (tree![2, 1, 3], tree![2, 3, 1]),
            (tree![], tree![]),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::invert_tree(input), want)
        }
    }

    #[test]
    fn test_max_path_sum() {
        let data = vec![
            (tree![1, 2, 3], 6),
            (tree![-10, 9, 20, null, null, 15, 7], 42),
            (
                tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                48,
            ),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::max_path_sum(input), want)
        }
    }

    #[test]
    fn test_level_order() {
        let data = vec![
            (
                tree![3, 9, 20, null, null, 15, 7],
                vec![vec![3], vec![9, 20], vec![15, 7]],
            ),
            (tree![1], vec![vec![1]]),
            (tree![], vec![]),
            (
                tree![1, 2, null, 3, null, 4, null, 5],
                vec![vec![1], vec![2], vec![3], vec![4], vec![5]],
            ),
        ];

        for (input, want) in data.into_iter() {
            assert_eq!(Solution::level_order(input), want)
        }
    }

    #[test]
    fn test_codec() {
        let c = Codec::new();
        let data = vec![tree![1, 2, 3, null, null, 4, 5], tree![]];
        for input in data.into_iter() {
            let r1 = c.serialize(input.clone());
            let r2 = c.deserialize(r1);
            assert_eq!(r2, input);
        }
    }

    #[test]
    fn test_is_subtree() {
        let data = vec![
            (tree![3, 4, 5, 1, 2], tree![4, 1, 2], true),
            (
                tree![3, 4, 5, 1, 2, null, null, null, null, 0],
                tree![4, 1, 2],
                false,
            ),
        ];

        for (t1, t2, want) in data.into_iter() {
            assert_eq!(Solution::is_subtree(t1, t2), want)
        }
    }

    #[test]
    fn test_build_tree() {
        let data = vec![
            (
                vec![3, 9, 20, 15, 7],
                vec![9, 3, 15, 20, 7],
                tree![3, 9, 20, null, null, 15, 7],
            ),
            (vec![-1], vec![-1], tree![-1]),
        ];
        for (a, b, want) in data.into_iter() {
            assert_eq!(Solution::build_tree(a, b), want)
        }
    }

    #[test]
    fn test_is_valid_bst() {
        let data = vec![
            (tree![2, 1, 3], true),
            (tree![5, 1, 4, null, null, 3, 6], false),
        ];

        for (input, want) in data {
            assert_eq!(Solution::is_valid_bst(input), want)
        }
    }

    #[test]
    fn test_kth_smallest() {
        let data = vec![
            (tree![3, 1, 4, null, 2], 1, 1),
            (tree![5, 3, 6, 2, 4, null, null, 1], 3, 3),
            (tree![1], 1, 1),
        ];

        for (input, k, want) in data.into_iter() {
            assert_eq!(Solution::kth_smallest(input, k), want)
        }
    }

    #[test]
    fn test_lowest_common_ancestor() {
        let data = vec![
            (tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5], 2, 8, 6),
            (tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5], 2, 4, 2),
            (tree![2, 1], 2, 1, 2),
        ];
        for (root, p, q, want) in data.into_iter() {
            let res = Solution::lowest_common_ancestor(
                root,
                Some(Rc::new(RefCell::new(TreeNode::new(p)))),
                Some(Rc::new(RefCell::new(TreeNode::new(q)))),
            );
            assert_eq!(res.as_deref().unwrap().borrow().val, want);
        }
    }

    #[test]
    fn test_trie() {
        let trie = Trie::new();

        trie.insert("apple".to_string());

        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);

        trie.insert("app".to_string());

        assert_eq!(trie.search("app".to_string()), true);
    }

    #[test]
    fn test_word_dictionary() {
        let dic = WordDictionary::new();

        dic.add_word("bad".to_string());
        dic.add_word("dad".to_string());
        dic.add_word("mad".to_string());

        assert_eq!(dic.search("pad".to_string()), false);
        assert_eq!(dic.search("bad".to_string()), true);
        assert_eq!(dic.search(".ad".to_string()), true);
        assert_eq!(dic.search("b..".to_string()), true);
    }
    #[test]
    fn test_find_words() {
        let data = vec![
            (
                vec![
                    vec!["o", "a", "a", "n"],
                    vec!["e", "t", "a", "e"],
                    vec!["i", "h", "k", "r"],
                    vec!["i", "f", "l", "v"],
                ],
                vec!["oath", "pea", "eat", "rain"],
                vec!["oath", "eat"],
            ),
            (vec![vec!["a", "b"], vec!["c", "d"]], vec!["abcb"], vec![]),
            (vec![vec!["a", "b"]], vec!["ba"], vec!["ba"]),
            (
                vec![
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                    vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a", "a"],
                ],
                vec![
                    "a",
                    "aa",
                    "aaa",
                    "aaaa",
                    "aaaaa",
                    "aaaaaa",
                    "aaaaaaa",
                    "aaaaaaaa",
                    "aaaaaaaaa",
                    "aaaaaaaaaa",
                ],
                vec![
                    "a",
                    "aa",
                    "aaa",
                    "aaaa",
                    "aaaaa",
                    "aaaaaa",
                    "aaaaaaa",
                    "aaaaaaaa",
                    "aaaaaaaaa",
                    "aaaaaaaaaa",
                ],
            ),
        ];
        for (input, strs, want) in data.into_iter() {
            let input = input
                .into_iter()
                .map(|x| x.into_iter().map(|v| v.chars().next().unwrap()).collect())
                .collect::<Vec<Vec<char>>>();

            let strs = strs.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();

            let mut want = want
                .into_iter()
                .map(|x: &str| x.to_string())
                .collect::<Vec<_>>();

            let mut res = Solution::find_words(input, strs);
            want.sort();
            res.sort();

            assert_eq!(res, want);
        }
    }
}
