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
        recursive(root)
    }
}

fn recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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

fn iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut current_node = root;
    let mut stack = Vec::<Rc<RefCell<TreeNode>>>::new();
    let mut result = vec![];

    loop {
        match (&current_node, &mut stack) {
            (Some(node), stack) => {
                stack.push(node.clone());
                let left_node = node.borrow_mut().left.take();
                current_node = left_node;
            }
            (None, stack) if !stack.is_empty() => {
                let node = stack.pop().unwrap();
                let mut node_ref = node.borrow_mut();
                result.push(node_ref.val);
                current_node = node_ref.right.take();
            }
            (None, _) => break,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod iterative {
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

            assert_eq!(iterative(root), vec![1, 3, 2])
        }
    }

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

            assert_eq!(recursive(root), vec![1, 3, 2])
        }
    }
}
