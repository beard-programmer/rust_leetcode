use std::{cmp, collections::HashMap};

pub fn run(s: String) -> i32 {
    let mut chars_indexes: HashMap<char, usize> = HashMap::new();

    s.chars()
        .enumerate()
        .fold(0, move |max, (position, character)| {
            match chars_indexes.insert(character, position) {
                None => cmp::max(max, chars_indexes.len()),
                Some(old_position) => {
                    chars_indexes.retain(|_, position| match old_position.cmp(position) {
                        cmp::Ordering::Less | cmp::Ordering::Equal => true,
                        _ => false,
                    });
                    max
                }
            }
        }) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    // Input: s = "abcabcbb"
    // Output: 3
    // Explanation: The answer is "abc", with the length of 3.
    #[test]
    fn example_1() {
        let s = String::from("abcabcbb");
        let result = run(s);
        assert_eq!(result, 3);
    }

    // Input: s = "bbbbb"
    // Output: 1
    // Explanation: The answer is "b", with the length of 1.
    #[test]
    fn example_2() {
        let s = String::from("bbbbb");
        let result = run(s);
        assert_eq!(result, 1);
    }
    // Input: s = "pwwkew"
    // Output: 3
    // Explanation: The answer is "wke", with the length of 3.
    // Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
    #[test]
    fn example_3() {
        let s = String::from("pwwkew");
        let result = run(s);
        assert_eq!(result, 3);
    }

    // Input: s = ""
    // Output: 0
    #[test]
    fn example_4() {
        let s = String::from("");
        let result = run(s);
        assert_eq!(result, 0);
    }

    // Input: s = "a"
    // Output: 1
    #[test]
    fn example_5() {
        let s = String::from("a");
        let result = run(s);
        assert_eq!(result, 1);
    }

    // Input: s = "abcadefgg"
    // Output:
    #[test]
    fn example_6() {
        let s = String::from("abcadefgg");
        let result = run(s);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_7() {
        let s = String::from("bbtablud");
        let result = run(s);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_8() {
        let s = String::from(
            "plbvuntohelijzsxtdinazvjmiafpkufxxaskvgrctcnuukozvbcuylghnowyidxfgprdykfrmu",
        );
        let result = run(s);
        assert_eq!(result, 14);
    }

    #[test]
    fn example_9() {
        let s = String::from("aabaab!bb");
        let result = run(s);
        assert_eq!(result, 3);
    }
}
