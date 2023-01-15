use std::cmp::Ordering;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let result = with_iterators(&nums1, m, &nums2, n);
        let _ = std::mem::replace(nums1, result);
    }
}

fn with_iterators(nums1: &Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) -> Vec<i32> {
    let nums1s = nums1.clone();
    let mut left_iter = nums1s[0..m as usize].iter().rev();
    let mut right_iter = nums2.iter().rev();

    let mut left_item = left_iter.next();
    let mut right_item = right_iter.next();

    let mut result = VecDeque::<i32>::with_capacity((n + m) as usize);

    loop {
        match (left_item, right_item) {
            (Some(left_value), Some(right_value)) => match left_value.cmp(&right_value) {
                Ordering::Less | Ordering::Equal => {
                    result.push_front(*right_value);
                    right_item = right_iter.next();
                }
                Ordering::Greater => {
                    result.push_front(*left_value);
                    left_item = left_iter.next();
                }
            },
            (Some(value), None) => {
                result.push_front(*value);
                left_item = left_iter.next();
            }
            (None, Some(value)) => {
                result.push_front(*value);
                right_item = right_iter.next();
            }
            (None, None) => break,
        }
    }

    result.into()
}

fn brute_force(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut left_index, mut right_index) = (m, n);

    loop {
        let insert_number_position = (left_index + right_index - 1) as usize;

        match (
            nums1.get((left_index - 1) as usize),
            nums2.get((right_index - 1) as usize),
        ) {
            (Some(left_value), Some(right_value)) if right_value < left_value => {
                nums1[insert_number_position as usize] = *left_value;
                left_index -= 1;
            }
            (_, Some(right_value)) => {
                nums1[insert_number_position as usize] = *right_value;
                right_index -= 1;
            }
            (_, None) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod with_iterators {
        use super::*;

        #[test]
        fn example_1() {
            let mut nums1 = vec![1, 2, 3, 0, 0, 0];
            let mut nums2 = vec![2, 5, 6];
            let result = with_iterators(&mut nums1, 3, &mut nums2, 3);

            assert_eq!(result, vec![1, 2, 2, 3, 5, 6]);
        }

        #[test]
        fn example_2() {
            let mut nums1 = vec![0];
            let mut nums2 = vec![1];
            let result = with_iterators(&mut nums1, 0, &mut nums2, 1);
            assert_eq!(result, vec![1]);
        }
    }

    mod brute_force {
        use super::*;

        #[test]
        fn example_1() {
            let mut nums1 = vec![1, 2, 3, 0, 0, 0];
            let mut nums2 = vec![2, 5, 6];
            brute_force(&mut nums1, 3, &mut nums2, 3);
            assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
        }

        #[test]
        fn example_2() {
            let mut nums1 = vec![0];
            let mut nums2 = vec![1];
            brute_force(&mut nums1, 0, &mut nums2, 1);
            assert_eq!(nums1, vec![1]);
        }
    }
}
