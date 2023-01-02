#[derive(Debug)]
struct Sqrt {
    value: i32,
    square: i32,
    distance: i32,
}

pub fn run(x: i32) -> i32 {
    match x {
        0 => 0,
        1..=3 => 1,
        4..=8 => 2,
        9..=15 => 3,
        16..=24 => 4,
        25..=35 => 5,
        36..=48 => 6,
        49..=63 => 7,
        64..=80 => 8,
        _ => {
            let treshhold: i32 = x / 2;

            let mut closest = Sqrt {
                value: 1,
                square: 1,
                distance: x - 1 * 1,
            };
            let mut index = 2;
            loop {
                match index.cmp(&treshhold) {
                    std::cmp::Ordering::Greater => break,
                    _ => {
                        let square = index * index;
                        let candidate = Sqrt {
                            value: index,
                            square: square,
                            distance: x - square,
                        };

                        index += 1;
                        match candidate.distance {
                            distance if distance < 0 => {
                                break;
                            }
                            distance if distance == 0 => {
                                closest = candidate;
                                break;
                            }
                            distance if distance < closest.distance => closest = candidate,
                            _ => (),
                        };
                    }
                }
            }

            closest.value
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
            vec![
                (0, 0),
                (1, 1),
                (4, 2),
                (5, 2),
                (6, 2),
                (7, 2),
                (15, 3),
                (63, 7),
                (2_147_395_599, 46339),
            ]
            .iter()
            .for_each(|(input, expected_result)| assert_eq!(run(*input), *expected_result));
        }
    }
}
