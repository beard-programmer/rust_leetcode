use super::ListNode;
use std::cmp::Ordering;

pub fn run(left: Option<Box<ListNode>>, right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (left, right) {
        (None, None) => None,
        (Some(list), None) | (None, Some(list)) => Some(list),
        (Some(left), Some(right)) => match left.val.cmp(&right.val) {
            Ordering::Less | Ordering::Equal => Some(Box::new(ListNode {
                val: left.val,
                next: run(left.next, Some(right)),
            })),
            Ordering::Greater => Some(Box::new(ListNode {
                val: right.val,
                next: run(Some(left), right.next),
            })),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Input: list1 = [1,2,4], list2 = [1,3,4]
    // Output: [1,1,2,3,4,4]
    #[test]
    fn example_1() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let result = run(list1, list2);
        let expected_result = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(result, expected_result);
    }

    // Input: list1 = [], list2 = []
    // Output: []
    #[test]
    fn example_2() {
        let list1 = None::<Box<ListNode>>;
        let list2 = None::<Box<ListNode>>;
        let result = run(list1, list2);
        assert_eq!(result, None::<Box<ListNode>>);
    }

    // Input: list1 = [], list2 = [0]
    // Output: [0]
    #[test]
    fn example_3() {
        let list1 = None::<Box<ListNode>>;
        let list2 = Some(Box::new(ListNode::new(0)));
        let result = run(list1, list2);
        assert_eq!(result, Some(Box::new(ListNode::new(0))));
    }
}
