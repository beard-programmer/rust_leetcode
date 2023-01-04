use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        recursive(root)
    }
}

fn recursive(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn compare(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(left), Some(right)) if left.borrow().val == right.borrow().val => {
                let (left_left, left_right) =
                    (left.borrow().left.clone(), left.borrow().right.clone());
                let (right_right, right_left) =
                    (right.borrow().right.clone(), right.borrow().left.clone());
                let (left_left, right_right) =
                    (left.borrow().left.clone(), right.borrow().right.clone());
                let (left_right, right_left) =
                    (left.borrow().right.clone(), right.borrow().left.clone());

                compare(left_left, right_right) && compare(left_right, right_left)
            }
            _ => false,
        }
    }
    match root {
        Some(root) => compare(root.borrow().left.clone(), root.borrow().right.clone()),
        None => true,
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
            let root = Some(Rc::new(RefCell::new(TreeNode {
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
            })));

            assert_eq!(recursive(root), false)
        }

        #[test]
        fn example_2() {
            let root = Some(Rc::new(RefCell::new(TreeNode {
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
            })));

            assert_eq!(recursive(root), true)
        }
    }
}
