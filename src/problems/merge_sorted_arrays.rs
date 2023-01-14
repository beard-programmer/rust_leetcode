struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        brute_force(nums1, m, nums2, n);
    }
}

fn brute_force(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut left_index, mut right_index) = (m, n);

    loop {
        let insert_number_position = left_index + right_index - 1;

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
