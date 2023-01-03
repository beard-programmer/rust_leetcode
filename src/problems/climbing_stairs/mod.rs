use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache: HashMap<i32, i32> = HashMap::new();
        with_cache(n, &mut cache)
    }
}

fn brute_force(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 2,
        _ => brute_force(n - 1) + brute_force(n - 2),
    }
}

fn with_cache(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    match cache.get(&n) {
        Some(value) => *value,
        None => {
            let element = match n {
                1 => 1,
                2 => 2,
                _ => with_cache(n - 1, cache) + with_cache(n - 2, cache),
            };
            cache.insert(n, element);
            element
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    mod brute_force {
        use super::*;

        #[test]
        fn example_1() {
            assert_eq!(brute_force(22), 28657);
        }
    }

    #[cfg(test)]
    mod with_cache {
        use super::*;

        #[test]
        fn example_1() {
            let mut cache: HashMap<i32, i32> = HashMap::new();
            assert_eq!(with_cache(45, &mut cache), 1836311903);
        }
    }
}
