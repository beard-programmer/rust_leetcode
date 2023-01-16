// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        recursive(root.as_ref())
    }
}

pub fn recursive(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            let left_depth = recursive(node.left.as_ref());
            let right_depth = recursive(node.right.as_ref());

            cmp::max(left_depth, right_depth) + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod recursive {
        use super::*;

        #[test]
        fn example_1() {
            let root = build_tree_helper(false);

            assert_eq!(recursive(root.as_ref()), 3)
        }

        #[test]
        fn example_2() {
            let root = build_tree_helper(true);
            assert_eq!(recursive(root.as_ref()), 3)
        }
    }

    fn build_tree_helper(symmetric: bool) -> Option<Rc<RefCell<TreeNode>>> {
        match symmetric {
            true => Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            false => Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        }
    }
}
