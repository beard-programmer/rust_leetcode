use std::cmp::Ordering;

use super::ListNode;

pub fn run(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(Box::new(ListNode {
            val: node.val,
            next: node.next,
        })),
        (Some(left), Some(right)) => match (left.val + right.val).cmp(&10) {
            Ordering::Less => Some(Box::new(ListNode {
                val: left.val + right.val,
                next: run(left.next, right.next),
            })),
            Ordering::Equal | Ordering::Greater => Some(Box::new(ListNode {
                val: left.val + right.val - 10,
                next: run(run(left.next, Some(Box::new(ListNode::new(1)))), right.next),
            })),
        },
    }
}

// Input: l1 = [0], l2 = [0]
// Output: [0]
#[cfg(test)]
mod tests {
    use super::*;

    // Input: l1 = [2,4,3], l2 = [5,6,4]
    // Output: [7,0,8]
    // Explanation: 342 + 465 = 807.
    #[test]
    fn example_1() {
        let list1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let result = run(list1, list2);
        let expected = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }));

        assert_eq!(&result, &expected);
    }

    // Input: l1 = [0], l2 = [0]
    // Output: [0]
    #[test]
    fn example_2() {
        let list1 = Some(Box::new(ListNode { val: 0, next: None }));
        let list2 = Some(Box::new(ListNode { val: 0, next: None }));
        let result = run(list1, list2);
        let expected = Some(Box::new(ListNode { val: 0, next: None }));
        assert_eq!(&result, &expected);
    }
    // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    // Output: [8,9,9,9,0,0,0,1]
    #[test]
    fn example_3() {
        let list1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));
        let result = run(list1, list2);
        let expected = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode { val: 1, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(&result, &expected);
    }
}
