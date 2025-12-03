use std::{collections::HashSet, ops::RangeInclusive};

use winnow::{
    ascii::digit1,
    combinator::{separated, separated_pair},
    Parser, Result,
};

type IdRange = RangeInclusive<u128>;

fn input_parser<'a>(input: &mut &'a str) -> Result<Vec<IdRange>> {
    separated(
        1..,
        separated_pair(digit1, '-', digit1)
            .map(|(a, b): (&str, &str)| a.parse::<u128>().unwrap()..=b.parse::<u128>().unwrap()),
        ',',
    )
    .parse_next(input)
}

fn append_leader(num: u128, lead: u128) -> u128 {
    let lead_len = lead.ilog10() + 1;
    num * 10u128.pow(lead_len) + lead
}

fn invalid_ids(range: IdRange, part_1: bool) -> Vec<u128> {
    let mut invalid_set = HashSet::new();

    let end = range.end();
    let max_len = end.ilog10();

    for lead_len in 1..=max_len / 2 + 1 {
        for lead in 1..10u128.pow(lead_len as u32) {
            let mut new_id = lead;
            loop {
                new_id = append_leader(new_id, lead);
                if range.contains(&new_id) {
                    invalid_set.insert(new_id);
                } else if part_1 || new_id.ilog10() > end.ilog10() {
                    break;
                }
            }
        }
    }
    invalid_set.into_iter().collect()
}

pub fn part1(input: &str) -> u128 {
    let mut input = input;
    let input = input_parser.parse_next(&mut input).unwrap();
    let mut sum = 0;
    for range in input {
        let inv = invalid_ids(range, true);
        sum += inv.into_iter().sum::<u128>();
    }
    sum
}

pub fn part2(input: &str) -> u128 {
    let mut input = input;
    let input = input_parser.parse_next(&mut input).unwrap();
    let mut sum = 0;
    for range in input {
        let inv = invalid_ids(range, false);
        sum += inv.into_iter().sum::<u128>();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rstest::*;

    #[rstest]
    fn test_parse_input() {
        let mut input = "11-22,95-115\n\n";
        let parsed = input_parser.parse_next(&mut input).unwrap();
        assert_eq!(parsed, vec![11..=22, 95..=115]);
    }

    #[rstest]
    #[case(11..=22, vec![11, 22])]
    #[case(95..=115, vec![99])]
    #[case(998..=1012, vec![1010])]
    #[case(1188511880..=1188511890, vec![1188511885])]
    #[case(222220..=222224, vec![222222])]
    #[case(1698522..=1698528, vec![])]
    #[case(446443..=446449, vec![446446])]
    #[case(38593856..=38593862, vec![38593859])]
    fn test_invalid_ids_part1(#[case] range: IdRange, #[case] expected: Vec<u128>) {
        let actual = invalid_ids(range, true);
        assert!(
            actual.iter().sorted().eq(expected.iter().sorted()),
            "Expected: {:?}, Actual: {:?}",
            expected,
            actual
        );
    }

    #[rstest]
    #[case(11..=22, vec![11, 22])]
    #[case(95..=115, vec![99, 111])]
    #[case(998..=1012, vec![999, 1010])]
    #[case(1188511880..=1188511890, vec![1188511885])]
    #[case(222220..=222224, vec![222222])]
    #[case(1698522..=1698528, vec![])]
    #[case(446443..=446449, vec![446446])]
    #[case(38593856..=38593862, vec![38593859])]
    #[case(565653..=565659, vec![565656])]
    #[case(824824821..=824824827, vec![824824824])]
    #[case(2121212118..=2121212124, vec![2121212121])]
    fn test_invalid_ids_part2(#[case] range: IdRange, #[case] expected: Vec<u128>) {
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
