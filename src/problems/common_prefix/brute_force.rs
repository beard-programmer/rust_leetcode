use std::collections::HashSet;

pub fn run(strs: Vec<String>) -> String {
    let prefixes: Vec<HashSet<&str>> = strs.iter().map(|string| string_prefixes(&string)).collect();
    let string = strs.iter().next().unwrap();
    let biggest_prefix = string
        .char_indices()
        .fold("", move |current_bigest, (position, _)| {
            let possible_bigest = &string[..=position];
            match is_prefix_present(possible_bigest, &prefixes) {
                true => possible_bigest,
                false => current_bigest,
            }
        });

    biggest_prefix.to_string()
}

fn string_prefixes<'a>(string: &'a str) -> HashSet<&'a str> {
    let slice = &string[..];
    let asb = string
        .char_indices()
        .map(move |(pos, _)| &slice[..=pos])
        .collect::<HashSet<&str>>();
    asb
}

fn is_prefix_present(prefix: &str, collection: &Vec<HashSet<&str>>) -> bool {
    collection
        .iter()
        .find(|prefix_set| !prefix_set.contains(prefix))
        .is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let strs: Vec<String> = ["flower", "flow", "flight"].map(String::from).to_vec();
        let expected_result = "fl".to_string();
        let result = run(strs);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn example_2() {
        let strs: Vec<String> = ["dog", "racecar", "car"].map(String::from).to_vec();
        let expected_result = "".to_string();
        let result = run(strs);

        assert_eq!(result, expected_result);
    }

    mod private_functions {
        use super::*;

        #[test]
        fn is_prefix_present_test() {
            let collection: Vec<HashSet<&str>> = vec![
                HashSet::from_iter(["", "v", "vi", "vic", "vict", "victo", "victor"]),
                HashSet::from_iter(["v", "vi", "vic", "vico", "vicod", "vicodi", "vicodin"]),
            ];

            assert!(is_prefix_present("v", &collection));
            assert!(is_prefix_present("vi", &collection));
            assert!(is_prefix_present("vic", &collection));
            assert_eq!(is_prefix_present("vict", &collection), false);
            assert_eq!(is_prefix_present("", &collection), false);
        }

        #[test]
        fn string_prefixes_test() {
            assert_eq!(
                &string_prefixes("victor"),
                &HashSet::from_iter(["v", "vi", "vic", "vict", "victo", "victor"])
            );

            assert_eq!(&string_prefixes(""), &HashSet::from_iter([]));
            assert_eq!(&string_prefixes(" h"), &HashSet::from_iter([" ", " h"]));
        }
    }
}
