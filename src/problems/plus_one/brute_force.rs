pub fn run(mut digits: Vec<i32>) -> Vec<i32> {
    match digits
        .iter_mut()
        .rev()
        .fold(1, |remainder, element| match remainder {
            1 if 9 <= *element => {
                *element = 0;
                1
            }
            1 => {
                *element = *element + 1;
                0
            }
            0 => 0,
            _ => panic!("WTF?"),
        }) {
        0 => digits,
        1 => {
            digits.insert(0, 1);
            digits
        }
        _ => panic!("WTF?"),
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
            let mut digits = vec![1, 2, 3];
            assert_eq!(run(digits), vec![1, 2, 4]);
        }

        #[test]
        fn example_2() {
            let mut digits = vec![9];
            assert_eq!(run(digits), vec![1, 0]);
        }
    }
}
