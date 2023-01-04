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
    fn is_symmetric(
        left: Option<&Rc<RefCell<TreeNode>>>,
        right: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, Some(_)) | (Some(_), None) => false,
            (Some(l), Some(r)) if l.borrow().val != r.borrow().val => false,
            (None, None) => true,
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                is_symmetric(left.left.as_ref(), right.right.as_ref())
                    && is_symmetric(left.right.as_ref(), right.left.as_ref())
            }
        }
    }
    match root {
        Some(root) => is_symmetric(root.borrow().left.as_ref(), root.borrow().right.as_ref()),
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
            let root = build_tree_helper(false);

            assert_eq!(recursive(root), false)
        }

        #[test]
        fn example_2() {
            let root = build_tree_helper(true);
            assert_eq!(recursive(root), true)
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
