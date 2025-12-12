use std::{collections::HashSet, ops::RangeInclusive};

use winnow::{
    Parser, Result,
    ascii::digit1,
    combinator::{separated, separated_pair},
};

type Id = u64;
type IdRange = RangeInclusive<Id>;

fn input_parser(input: &mut &str) -> Result<Vec<IdRange>> {
    separated(
        1..,
        separated_pair(digit1, '-', digit1)
            .map(|(a, b): (&str, &str)| a.parse::<Id>().unwrap()..=b.parse::<Id>().unwrap()),
        ',',
    )
    .parse_next(input)
}

fn lshift(num: Id, n: u32) -> Id {
    num * Id::pow(10, n)
}

fn invalid_ids(range: IdRange, part_1: bool) -> Vec<Id> {
    let mut invalid_set = HashSet::new();

    let end = range.end();
    let max_len = end.ilog10();

    for lead_len in 1..=max_len / 2 + 1 {
        let lead_start = Id::pow(10, lead_len - 1);
        let lead_end = Id::pow(10, lead_len);
        for lead in lead_start..lead_end {
            // Can probably make it more efficient by making it the right lenght
            // from the start.
            let mut new_id = lead;
            loop {
                new_id = lshift(new_id, lead_len) + lead;
                if range.contains(&new_id) {
                    invalid_set.insert(new_id);
                } else if part_1 || new_id > *end {
                    break;
                }
            }
        }
    }
    invalid_set.into_iter().collect()
}

pub fn part1(input: &str) -> Id {
    let mut input = input;
    let input = input_parser.parse_next(&mut input).unwrap();
    let mut sum = 0;
    for range in input {
        let inv = invalid_ids(range, true);
        sum += inv.into_iter().sum::<Id>();
    }
    sum
}

pub fn part2(input: &str) -> Id {
    let mut input = input;
    let input = input_parser.parse_next(&mut input).unwrap();
    let mut sum = 0;
    for range in input {
        let inv = invalid_ids(range, false);
        sum += inv.into_iter().sum::<Id>();
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
    fn test_invalid_ids_part1(#[case] range: IdRange, #[case] expected: Vec<Id>) {
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
    fn test_invalid_ids_part2(#[case] range: IdRange, #[case] expected: Vec<Id>) {
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
