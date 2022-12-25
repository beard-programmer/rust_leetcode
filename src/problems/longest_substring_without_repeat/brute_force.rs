use std::cmp;

pub fn run(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let init = (s[0..1].to_string(), 1);

    let (_, max) = s
        .get(1..)
        .unwrap()
        .chars()
        .fold(init, |(substring, max), next_character| {
            match substring.find(next_character) {
                None => {
                    let mut new_s = substring.clone();
                    new_s.push(next_character);
                    let max = cmp::max(max, new_s.len());

                    (new_s, max)
                }
                Some(position) => {
                    let mut new_s = substring[(position + 1)..].to_string();
                    new_s.push(next_character);
                    (new_s, max)
                }
            }
        });
    max as i32
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
