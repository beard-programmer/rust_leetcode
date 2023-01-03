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
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        brute_force(root)
    }
}

fn brute_force(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];

    fn traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match node {
            Some(node) => {
                traversal(node.borrow().left.clone(), res);
                res.push(node.borrow().val);
                traversal(node.borrow().right.clone(), res);
            }
            _ => (),
        }
    }

    traversal(root, &mut result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod brute_force {
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

            assert_eq!(brute_force(root), vec![1, 3, 2])
        }
    }
}
