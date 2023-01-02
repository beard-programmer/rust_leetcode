#[derive(Debug)]
pub enum RomanNumber {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<char> for RomanNumber {
    fn from(char: char) -> Self {
        match char {
            'I' => RomanNumber::I,
            'V' => RomanNumber::V,
            'X' => RomanNumber::X,
            'L' => RomanNumber::L,
            'C' => RomanNumber::C,
            'D' => RomanNumber::D,
            'M' => RomanNumber::M,
            _ => panic!("Invalid roman"),
        }
    }
}

impl RomanNumber {
    const fn value(&self) -> i32 {
        match &self {
            RomanNumber::I => 1,
            RomanNumber::V => 5,
            RomanNumber::X => 10,
            RomanNumber::L => 50,
            RomanNumber::C => 100,
            RomanNumber::D => 500,
            RomanNumber::M => 1000,
        }
    }
}

pub fn run(s: String) -> i32 {
    let (total, _) = s.chars().map(|char| RomanNumber::from(char)).fold(
        (0, None),
        |(previous_total, previous_roman), next_roman| match &previous_roman {
            None => (previous_total + next_roman.value(), Some(next_roman)),
            Some(previous_roman) => {
                let next_roman_value = match (&previous_roman, &next_roman) {
                    (RomanNumber::I, RomanNumber::V)
                    | (RomanNumber::I, RomanNumber::X)
                    | (RomanNumber::X, RomanNumber::L)
                    | (RomanNumber::X, RomanNumber::C)
                    | (RomanNumber::C, RomanNumber::D)
                    | (RomanNumber::C, RomanNumber::M) => {
                        let previous_roman_value = previous_roman.value();
                        next_roman.value() - previous_roman_value - previous_roman_value
                    }
                    _ => next_roman.value(),
                };
                (previous_total + next_roman_value, Some(next_roman))
            }
        },
    );

    total
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
                ("I", 1),
                ("II", 2),
                ("III", 3),
                ("IV", 4),
                ("V", 5),
                ("VI", 6),
                ("VII", 7),
                ("VIII", 8),
                ("IX", 9),
                ("XI", 11),
                ("XIII", 13),
                ("XIV", 14),
                ("XV", 15),
                ("XXIX", 29),
                ("XLVIII", 48),
                ("MCCXXXIV", 1234),
                ("MMXLVIII", 2048),
                ("MMMCDXXI", 3421),
            ]
            .iter()
            .for_each(|(roman_string, expected_result)| {
                assert_eq!(run(String::from(*roman_string)), *expected_result);
            });
        }
    }
}
