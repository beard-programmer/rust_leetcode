// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

// Constraints:

// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.

// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        broot_force(nums, target)
    }
}

fn from_leetcode(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut expectations: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match expectations.get(num) {
            Some(&index) => return vec![index as i32, i as i32],
            None => expectations.insert(target - num, i),
        };
    }
    return vec![-1, -1];
}

fn with_hashing_algoritm(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_iter = nums
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, value)| (value, index));

    let hashed: HashMap<i32, HashSet<usize>> = HashMap::new();

    let hashed_numbers_indexes = nums_iter
        .clone()
        .fold(hashed, |mut result, (value, index)| {
            match result.get_key_value(&value) {
                Some((_, stored_indexes)) => {
                    let mut stored_indexes = stored_indexes.clone();
                    stored_indexes.insert(index);
                    result.insert(value, stored_indexes);
                    result
                }
                None => {
                    let mut init_indexes = HashSet::<usize>::new();
                    init_indexes.insert(index);
                    result.insert(value, init_indexes);
                    result
                }
            }
        });

    nums_iter
        .filter_map(|(value, seek_index)| {
            let seek_value = target - value;
            match hashed_numbers_indexes.get(&seek_value) {
                Some(indexes) => match indexes.len() {
                    0 => None,
                    1 if indexes.contains(&seek_index) => None,
                    _ => Some(seek_index as i32),
                },
                None => None,
            }
        })
        .collect()
}

// O(n^2) because of iter inside iter
fn broot_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .filter_map(|(seek_index, seek)| {
            let seek_value = nums
                .iter()
                .enumerate()
                .filter(|(index, _)| index != &seek_index)
                .find(|(_, value)| *value + seek == target);
            match seek_value {
                Some(_) => Some(seek_index as i32),
                None => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected_result = vec![0, 1];
        get_algoritms_helper().iter().for_each(|algoritm| {
            let result = algoritm(nums.clone(), target);
            assert_eq!(result, expected_result)
        });
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected_result = vec![1, 2];
        get_algoritms_helper().iter().for_each(|algoritm| {
            let result = algoritm(nums.clone(), target);
            assert_eq!(result, expected_result)
        });
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let expected_result = vec![0, 1];
        get_algoritms_helper().iter().for_each(|algoritm| {
            let result = algoritm(nums.clone(), target);
            assert_eq!(result, expected_result)
        });
    }

    fn get_algoritms_helper() -> [fn(Vec<i32>, i32) -> Vec<i32>; 3] {
        [broot_force, with_hashing_algoritm, from_leetcode]
    }
}
