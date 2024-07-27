#![allow(unused)]
use std::cell::RefCell;
use std::rc::Rc;

// Binary tree node definition.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/*

    Sample [input] -> [5, 1, 2, 3, null, 6, 4]
    startValue = 3
    destValue = 6

*/

pub struct Solution;

impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let mut nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        for i in 0..4 {
            unimplemented!()
        }

        "".to_owned()
    }
}

fn main() {}

// let root = vec![Some(5), Some(1), Some(2), Some(3), None, Some(6), Some(4)];
//     Solution::get_directions(root, 3, 6);
