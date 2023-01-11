use std::cell::RefCell;
use std::rc::Rc;

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
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                is_symmetric(left.left.as_ref(), right.right.as_ref())
                    && is_symmetric(left.right.as_ref(), right.left.as_ref())
            }
            (None, None) => true,
        }
    }
    match root {
        Some(root) => is_symmetric(root.borrow().left.as_ref(), root.borrow().right.as_ref()),
        None => true,
    }
}

fn iterative(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn depth_first_traversal(
        root: Option<Rc<RefCell<TreeNode>>>,
        direction: TraverseDirection,
    ) -> Vec<Option<i32>> {
        let mut values: Vec<Option<i32>> = Vec::new();
        let mut nodes_to_visit = vec![root];

        loop {
            match nodes_to_visit.pop() {
                Some(maybe_node) => match maybe_node {
                    Some(node_rc) => {
                        let node = &*node_rc.borrow();
                        let TreeNode { val, left, right } = node;
                        values.push(Some(*val));

                        match direction {
                            TraverseDirection::Left => {
                                nodes_to_visit.push(right.clone());
                                nodes_to_visit.push(left.clone());
                            }
                            TraverseDirection::Right => {
                                nodes_to_visit.push(left.clone());
                                nodes_to_visit.push(right.clone());
                            }
                        }
                    }
                    None => values.push(None),
                },
                None => break,
            };
        }

        values
    }

    enum TraverseDirection {
        Left,
        Right,
    }

    let root_ref = root.unwrap();
    let root_node = &*root_ref.borrow();

    let left_values = depth_first_traversal(root_node.left.clone(), TraverseDirection::Left);
    let right_values = depth_first_traversal(root_node.right.clone(), TraverseDirection::Right);
    left_values == right_values
}

#[cfg(test)]
mod tests {
    use super::*;

    mod iterative {
        use super::*;

        #[test]
        fn example_1() {
            let root = build_tree_helper(false);

            assert_eq!(iterative(root), false)
        }

        #[test]
        fn example_2() {
            let root = build_tree_helper(true);
            assert_eq!(iterative(root), true)
        }
    }

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
