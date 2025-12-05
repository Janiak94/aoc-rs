type InputType = u128;

pub fn part1(input: &str) -> usize {
    let (ranges, ids) = parse::parse_input(input);

    ids.into_iter()
        .filter(|&id| ranges.iter().filter(|range| range.contains(&id)).count() > 0)
        .count()
}

pub fn part2(input: &str) -> InputType {
    let (ranges, _) = parse::parse_input(input);

    let ranges = {
        let mut ranges = ranges;
        ranges.sort_by_key(|range| *range.start());
        ranges
    };

    let mut total = 0;
    let mut current_start = ranges[0].start();
    let mut current_end = ranges[0].end();
    for r in &ranges[1..] {
        if *r.start() <= current_end + 1 {
            current_end = current_end.max(r.end());
        } else {
            total += current_end - current_start + 1;
            current_start = r.start();
            current_end = r.end();
        }
    }
    total += current_end - current_start + 1;
    total
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[rstest]
    fn test_part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let expected = 3;
        let actual = part1(input);
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let expected = 14;
        let actual = part2(input);
        assert_eq!(actual, expected);
    }
}

mod parse {
    use super::InputType;
    use std::ops::RangeInclusive;

    use winnow::{
        Parser, Result,
        ascii::{digit1, newline},
        combinator::{separated, separated_pair},
        error::{StrContext, StrContextValue},
    };

    fn create_ctx(desc: &'static str) -> StrContext {
        StrContext::Expected(StrContextValue::Description(desc))
    }

    fn ranges_parser(input: &mut &str) -> Result<Vec<RangeInclusive<InputType>>> {
        separated(
            1..,
            separated_pair(
                digit1.parse_to::<InputType>(),
                "-",
                digit1.parse_to::<InputType>(),
            )
            .map(|(start, end)| start..=end),
            newline,
        )
        .parse_next(input)
    }

    fn id_parser(input: &mut &str) -> Result<Vec<InputType>> {
        separated(
            1..,
            digit1.parse_to::<InputType>().context(create_ctx("id")),
            newline,
        )
        .context(create_ctx("all_ids"))
        .parse_next(input)
    }

    fn input_parser(input: &mut &str) -> Result<(Vec<RangeInclusive<InputType>>, Vec<InputType>)> {
        separated_pair(ranges_parser, "\n\n".context(create_ctx("del")), id_parser)
            .context(create_ctx("full_input"))
            .parse_next(input)
    }

    pub fn parse_input(input: &str) -> (Vec<RangeInclusive<InputType>>, Vec<InputType>) {
        // input_parser.parse(input).unwrap()
        let mut input = input;
        input_parser.parse_next(&mut input).unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::*;

        #[rstest]
        fn test_parse_input() {
            let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
            let expected = (
                vec![3..=5, 10..=14, 16..=20, 12..=18],
                vec![1, 5, 8, 11, 17, 32],
            );
            assert_eq!(input_parser.parse(input).unwrap(), expected);
        }

        #[rstest]
        fn test_parse_long() {
            let input = "299101822924925-300480478568268
538956871810902-540777983938739
486072692792085-486072692792085
499993691384229-500398599567638
134626100184815-140164095943839

67677576309617
33794763812662
168551711140598
68850974772815
377411025097359
144282091469283
294396730764118";
            assert!(input_parser.parse(input).is_ok());
        }
    }
}
