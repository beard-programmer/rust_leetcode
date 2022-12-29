use std::{cmp::Ordering, vec};

pub fn run(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    match (nums1, nums2) {
        (other, empty) | (empty, other) if empty.is_empty() => find_median_in_sorted_array(other),
        (nums1, nums2) => {
            (find_median_in_sorted_array(nums1) + find_median_in_sorted_array(nums2)) / 2.0
        }
    }
}

fn merge_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let length = nums1.len();
    let middle = length / 2;
    nums1
}

fn add_element(numbers: Vec<i32>, number: i32) -> Vec<i32> {
    match numbers {
        numbers if numbers.is_empty() => vec![number],
        numbers if numbers.len() == 1 => vec![
            std::cmp::min(number, numbers[0]),
            std::cmp::max(number, numbers[0]),
        ],
        numbers => {
            let middle = numbers.len() / 2;
            match number.cmp(&numbers[middle]) {
                Ordering::Less => vec![
                    add_element(numbers[..middle].to_vec(), number).as_slice(),
                    &numbers[middle..],
                ]
                .concat(),
                Ordering::Equal => vec![&numbers[..middle], &[number], &numbers[middle..]].concat(),
                Ordering::Greater => {
                    // dbg!(&numbers[..=middle]);
                    vec![
                        &numbers[..=middle],
                        add_element(numbers[(middle + 1)..].to_vec(), number).as_slice(),
                    ]
                    .concat()
                }
            }
        }
    }
}

fn find_median_in_sorted_array(numbers: Vec<i32>) -> f64 {
    let length = numbers.len();
    let middle = length / 2;
    match length {
        0 => 0.0,
        1 => numbers[0] as f64,
        2 => (numbers[0] as f64 + numbers[1] as f64) / 2.0,
        _ => match length % 2 {
            0 => (numbers[middle - 1] as f64 + numbers[middle] as f64) / 2.0,
            _ => (numbers[middle]) as f64,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    mod run {
        use super::*;

        #[test]
        fn example_1() {
            let nums1 = vec![1, 3];
            let nums2 = vec![2];
            assert_eq!(run(nums1, nums2), 2.00);
        }

        #[test]
        fn example_2() {
            let nums1 = vec![1, 2];
            let nums2 = vec![3, 4];
            assert_eq!(run(nums1, nums2), 2.5);
        }

        #[test]
        fn example_3() {
            let nums1 = vec![1, 3];
            let nums2 = vec![2, 7];
            assert_eq!(run(nums1, nums2), 2.5);
        }
    }

    #[cfg(test)]
    mod add_element {
        use super::*;

        #[test]
        fn example_0() {
            assert_eq!(add_element(vec![], 1), vec![1]);
        }

        #[test]
        fn example_1() {
            vec![(vec![1, 2], 3), (vec![1, 3], 2), (vec![2, 3], 1)]
                .iter()
                .for_each(|(left, right)| {
                    assert_eq!(add_element(left.clone(), *right), vec![1, 2, 3])
                });
        }

        #[test]
        fn example_2() {
            vec![
                (vec![1, 2, 3], 4),
                (vec![1, 2, 4], 3),
                (vec![1, 3, 4], 2),
                (vec![2, 3, 4], 1),
            ]
            .iter()
            .for_each(|(left, right)| {
                assert_eq!(add_element(left.clone(), *right), vec![1, 2, 3, 4])
            });
        }
    }

    #[cfg(test)]
    mod find_median_in_sorted_array {
        use super::*;

        #[test]
        fn example_0() {
            let nums1 = vec![1];
            assert_eq!(find_median_in_sorted_array(nums1), 1.0);
        }

        #[test]
        fn example_1() {
            let nums1 = vec![1, 2];
            assert_eq!(find_median_in_sorted_array(nums1), 1.5);
        }

        #[test]
        fn example_2() {
            let nums1 = vec![1, 2, 3];
            assert_eq!(find_median_in_sorted_array(nums1), 2.0);
        }

        #[test]
        fn example_3() {
            let nums1 = vec![1, 2, 3, 4];
            assert_eq!(find_median_in_sorted_array(nums1), 2.5);
        }

        #[test]
        fn example_4() {
            let nums1 = vec![1, 2, 3, 4, 5];
            assert_eq!(find_median_in_sorted_array(nums1), 3.0);
        }
    }
}
