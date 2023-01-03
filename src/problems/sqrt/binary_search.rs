use std::cmp::{self, Ordering};

pub fn run(x: i32) -> i32 {
    match x {
        0 => 0,
        1 | 2 | 3 => 1,
        _ => {
            //

            let (mut left, mut right) = (3, cmp::min(46340, x / 2));
            while left <= right {
                let middle = left + (right - left) / 2;
                if x / middle < middle {
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            }
            right
        }
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
            (0..=100_500).for_each(|input| {
                assert_eq!(run(input), f32::sqrt(input as f32).floor() as i32);
            });
        }
    }
}
