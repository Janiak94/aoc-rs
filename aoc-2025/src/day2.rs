use std::collections::HashSet;

use winnow::{
    ascii::digit1,
    combinator::{separated, separated_pair},
    Parser,
};

fn parse_input(input: &str) -> Result<Vec<(&str, &str)>, ()> {
    let mut input = input;
    separated(1.., separated_pair(digit1, '-', digit1), ',').parse_next(&mut input)
}

fn is_invalid(id: &str) -> bool {
    if id.len() % 2 == 1 {
        return false;
    }

    id[0..id.len() / 2] == id[id.len() / 2..]
}

fn contains(id: &str, (start, end): (&str, &str)) -> bool {
    id.parse::<u128>().unwrap() >= start.parse::<u128>().unwrap()
        && id.parse::<u128>().unwrap() <= end.parse::<u128>().unwrap()
}

fn invalid_ids((start, end): (&str, &str), part_1: bool) -> Vec<String> {
    let mut invalid_set = HashSet::new();

    if is_invalid(start) {
        invalid_set.insert(start.to_string());
    }

    for lead_len in 1..=end.len() / 2 + 1 {
        for lead in 1..10u128.pow(lead_len as u32) {
            let lead = lead.to_string();

            let mut new_id = lead.to_string().repeat(2);
            loop {
                if contains(&new_id, (start, end)) {
                    invalid_set.insert(new_id.clone());
                }
                new_id = format!("{}{}", &lead, &new_id);
                if part_1 || new_id.len() > end.len() {
                    break;
                }
            }
        }
    }
    invalid_set.into_iter().collect()
}

pub fn part1(input: &str) -> u128 {
    let input = parse_input(input).unwrap();
    let mut sum = 0;
    for range in input {
        let inv = invalid_ids(range, true);
        sum += inv
            .iter()
            .map(|id| id.parse::<u128>().unwrap())
            .sum::<u128>();
    }
    sum
}

pub fn part2(input: &str) -> u128 {
    let input = parse_input(input).unwrap();
    let mut sum = 0;
    for range in input {
        let inv = invalid_ids(range, false);
        sum += inv
            .iter()
            .map(|id| id.parse::<u128>().unwrap())
            .sum::<u128>();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rstest::*;

    use crate::day2::parse_input;

    #[rstest]
    fn test_parse_input() {
        let input = "11-22,95-115";
        let parsed = parse_input(input).unwrap();
        assert_eq!(parsed, vec![("11", "22"), ("95", "115")]);
    }

    #[rstest]
    #[case("11", true)]
    #[case("12", false)]
    #[case("1211", false)]
    #[case("1212", true)]
    fn test_is_invalid(#[case] id: &str, #[case] expected: bool) {
        assert_eq!(is_invalid(id), expected);
    }

    #[rstest]
    #[case(("11", "22"), vec!["11".to_string(), "22".to_string()])]
    #[case(("95", "115"), vec!["99".to_string()])]
    #[case(("998", "1012"), vec!["1010".to_string()])]
    #[case(("1188511880", "1188511890"), vec!["1188511885".to_string()])]
    #[case(("222220", "222224"), vec!["222222".to_string()])]
    #[case(("1698522", "1698528"), vec![])]
    #[case(("446443", "446449"), vec!["446446".to_string()])]
    #[case(("38593856","38593862"), vec!["38593859".to_string()])]
    fn test_invalid_ids_part1(#[case] range: (&str, &str), #[case] expected: Vec<String>) {
        let actual = invalid_ids(range, true);
        assert!(
            actual.iter().sorted().eq(expected.iter().sorted()),
            "Expected: {:?}, Actual: {:?}",
            expected,
            actual
        );
    }

    #[rstest]
    #[case(("11", "22"), vec!["11".to_string(), "22".to_string()])]
    #[case(("95", "115"), vec!["99".to_string(), 111.to_string()])]
    #[case(("998", "1012"), vec![999.to_string(), "1010".to_string()])]
    #[case(("1188511880", "1188511890"), vec!["1188511885".to_string()])]
    #[case(("222220", "222224"), vec!["222222".to_string()])]
    #[case(("1698522", "1698528"), vec![])]
    #[case(("446443", "446449"), vec!["446446".to_string()])]
    #[case(("38593856","38593862"), vec!["38593859".to_string()])]
    #[case(("565653","565659"), vec![565656.to_string()])]
    #[case(("824824821","824824827"), vec![824824824.to_string()])]
    #[case(("2121212118","2121212124"), vec![2121212121.to_string()])]
    fn test_invalid_ids_part2(#[case] range: (&str, &str), #[case] expected: Vec<String>) {
        let actual = invalid_ids(range, false);
        assert!(
            actual.iter().sorted().eq(expected.iter().sorted()),
            "Expected: {:?}, Actual: {:?}",
            expected,
            actual
        );
    }

    #[rstest]
    fn test_part_1() {
        let expected = 1227775554;
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let actual = part1(input);
        assert_eq!(actual, expected);
    }
}
