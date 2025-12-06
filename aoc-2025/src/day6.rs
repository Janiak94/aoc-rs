#[derive(Eq, PartialEq, Debug)]
enum Operation {
    Add,
    Mul,
}

#[derive(Eq, PartialEq, Debug)]
struct Column {
    nums: Vec<String>,
    op: Operation,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => Err("Unexpected operation".to_owned()),
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let cols = parse::parse_input(input);

    cols.into_iter()
        .map(|col| {
            let nums = col
                .nums
                .into_iter()
                .map(|num| num.trim().parse::<u64>().unwrap());
            match col.op {
                Operation::Add => nums.sum::<u64>(),
                Operation::Mul => nums.product(),
            }
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let cols = parse::parse_input(input);

    cols.into_iter()
        .map(|col| {
            let term_count = col.nums[0].len();
            let mut nums = vec![0; term_count];
            for num in col.nums.iter() {
                for (j, c) in num.chars().enumerate() {
                    if c.is_numeric() {
                        nums[j] = 10 * nums[j] + c.to_digit(10).unwrap() as u64;
                    }
                }
            }
            match col.op {
                Operation::Add => nums.iter().sum::<u64>(),
                Operation::Mul => nums.iter().product(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  

";
        let actual = part1(input);
        let expected = 4277556;
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_part2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  

";
        let actual = part2(input);
        let expected = 3263827;
        assert_eq!(actual, expected);
    }
}

mod parse {

    use super::{Column, Operation};
    use winnow::{
        Parser, Result,
        ascii::{newline, space1},
        combinator::separated,
        token::{one_of, rest, take_while},
    };

    fn input_parser(input: &mut &str) -> Result<Vec<Column>> {
        let (number_lines, _, operations): (Vec<&str>, (), Vec<Operation>) = (
            separated(
                1..,
                take_while(1.., |c: char| c.is_numeric() || c == ' '),
                newline,
            ),
            newline.void(),
            separated(
                1..,
                one_of(['+', '*']).map(|c: char| c.try_into().unwrap()),
                space1,
            ),
        )
            .parse_next(input)?;

        let _ = rest.parse_next(input)?;

        let num_chars = number_lines[0].len();

        let is_separator = number_lines
            .iter()
            .fold(vec![true; num_chars], |mut acc, line| {
                for (v, c) in acc.iter_mut().zip(line.chars()) {
                    *v = *v && c == ' ';
                }
                acc
            });
        let mut col_nums = vec![vec![]; operations.len()];
        for line in number_lines {
            let mut num = String::new();
            let mut col_nums_iter = col_nums.iter_mut();
            for (char, is_sep) in line.chars().zip(is_separator.iter()) {
                if !is_sep {
                    num.push(char);
                } else {
                    if let Some(l) = col_nums_iter.next() {
                        l.push(num.clone())
                    }
                    num.clear();
                }
            }
            if let Some(l) = col_nums_iter.next() {
                l.push(num);
            }
        }

        Ok(col_nums
            .into_iter()
            .zip(operations)
            .map(|(nums, op)| Column { nums, op })
            .collect())
    }

    pub(crate) fn parse_input(input: &str) -> Vec<Column> {
        input_parser.parse(input).expect("invalid input")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::*;

        #[rstest]
        fn test_parse() {
            let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
            let actual = parse_input(input);
            let expected = vec![
                Column {
                    nums: vec!["123".into(), " 45".into(), "  6".into()],
                    op: Operation::Mul,
                },
                Column {
                    nums: vec!["328".into(), "64 ".into(), "98 ".into()],
                    op: Operation::Add,
                },
                Column {
                    nums: vec![" 51".into(), "387".into(), "215".into()],
                    op: Operation::Mul,
                },
                Column {
                    nums: vec!["64 ".into(), "23 ".into(), "314".into()],
                    op: Operation::Add,
                },
            ];
            assert_eq!(expected, actual);
        }
    }
}
