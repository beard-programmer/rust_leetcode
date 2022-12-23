pub fn run(nums: &mut Vec<i32>) -> i32 {
    let mut tail_index = nums.len();
    let mut count = tail_index as i32;

    while 1 <= tail_index {
        let (left, right) = (nums.get(tail_index - 1), nums.get(tail_index));

        match (left, right) {
            (Some(left), Some(right)) if left == right => {
                nums.remove(tail_index);
                count -= 1;
            }
            _ => (),
        };
        tail_index -= 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // Input: nums = [1,1,2]
    // Output: 2, nums = [1,2,_]
    // Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
    // It does not matter what you leave beyond the returned k (hence they are underscores).
    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];
        let result = run(&mut nums);
        assert_eq!(result, 2);
        assert_eq!(nums, vec![1, 2]);
    }

    // Input: nums = [0,0,1,1,1,2,2,3,3,4]
    // Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
    // Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
    // It does not matter what you leave beyond the returned k (hence they are underscores).
    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = run(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}
