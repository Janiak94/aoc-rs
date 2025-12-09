use std::collections::VecDeque;

use itertools::Itertools;

type Tile = (u64, u64);

type Input = Vec<Tile>;

pub fn process(input: &str) -> Input {
    parse::parse_input(input)
}

pub fn part1(input: &Input) -> u64 {
    let mut max_area = 0;
    for (i, (x1, y1)) in input.iter().enumerate() {
        for (x2, y2) in input.iter().skip(i + 1) {
            let dx = x1.abs_diff(*x2) + 1;
            let dy = y1.abs_diff(*y2) + 1;
            max_area = max_area.max(dx * dy);
        }
    }
    max_area
}

struct CompressedTile((usize, usize), (u64, u64));

impl CompressedTile {
    fn compressed(&self) -> &(usize, usize) {
        &self.0
    }

    fn tile(&self) -> &Tile {
        &self.1
    }
}

const OUTSIDE: i32 = 0;
const INSIDE: i32 = 1;
const UNKNOWN: i32 = 2;

pub fn part2(input: &Input) -> u64 {
    let num_corners = input.len();

    let x_sorted = input
        .iter()
        .map(|c| c.0)
        .chain([0, u64::MAX])
        .sorted_unstable()
        .dedup();
    let y_sorted = input
        .iter()
        .map(|c| c.1)
        .chain([0, u64::MAX])
        .sorted_unstable()
        .dedup();

    let x_compressed = x_sorted.collect_vec();
    let y_compressed = y_sorted.collect_vec();

    let compressed = input
        .iter()
        .map(|c| {
            let x_idx = x_compressed.binary_search(&c.0).expect("could not find x");
            let y_idx = y_compressed.binary_search(&c.1).expect("could not find y");
            CompressedTile((x_idx, y_idx), *c)
        })
        .collect_vec();

    let mut grid = vec![vec![UNKNOWN; y_compressed.len()]; x_compressed.len()];
    let mut to_check: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);

    for i in 0..num_corners {
        let (x1, y1) = *compressed[i].compressed();
        let (x2, y2) = *compressed[(i + 1) % num_corners].compressed();

        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[x][y] = INSIDE;
            }
        }
    }

    let w = grid.len();
    let h = grid[0].len();
    while let Some(point) = to_check.pop_front() {
        let offsets: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
        let neighbors = offsets.iter().filter_map(|offset| {
            let x = point.0.checked_add_signed(offset.0)?;
            let y = point.1.checked_add_signed(offset.1)?;
            if x >= w || y >= h {
                return None;
            }
            Some((x, y))
        });

        // TODO: Assert inside grid
        for n in neighbors {
            if grid[n.0][n.1] == UNKNOWN {
                grid[n.0][n.1] = OUTSIDE;
                to_check.push_back(n);
            }
        }
    }

    for y in 1..h {
        for x in 1..w {
            let v = i32::from(grid[x][y] != OUTSIDE);
            let new_v =
                v + grid[x][y - 1] as i32 + grid[x - 1][y] as i32 - grid[x - 1][y - 1] as i32;
            grid[x][y] = new_v;
        }
    }

    let mut max_area = 0;
    for i in 0..num_corners {
        for j in i + 1..num_corners {
            let (x1, y1) = *compressed[i].compressed();
            let (x2, y2) = *compressed[j].compressed();

            let x3 = x1.min(x2);
            let x4 = x1.max(x2);
            let y3 = y1.min(y2);
            let y4 = y1.max(y2);

            let expected = (x4 - x3 + 1) as i32 * (y4 - y3 + 1) as i32;
            let actual = grid[x4][y4] - grid[x3 - 1][y4] - grid[x4][y3 - 1] + grid[x3 - 1][y3 - 1];

            if expected == actual {
                let (x1, y1) = compressed[i].tile();
                let (x2, y2) = compressed[j].tile();
                let dx = x1.abs_diff(*x2) + 1;
                let dy = y1.abs_diff(*y2) + 1;
                max_area = max_area.max(dx * dy);
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_process() {
        let expected = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        let actual = process(SAMPLE_INPUT);
        assert_eq!(expected, actual);
    }

    #[rstest]
    fn test_part1() {
        let input = process(SAMPLE_INPUT);
        let expected = 50;
        let actual = part1(&input);
        assert_eq!(expected, actual);
    }

    #[rstest]
    fn test_part2() {
        let input = process(SAMPLE_INPUT);
        let expected = 24;
        let actual = part2(&input);
        assert_eq!(expected, actual);
    }
}

mod parse {
    use winnow::{
        Parser, Result,
        ascii::{digit1, multispace0, newline},
        combinator::{separated, separated_pair, terminated},
    };

    use crate::day9::Tile;

    #[inline]
    fn corner(input: &mut &str) -> Result<Tile> {
        separated_pair(digit1.parse_to(), ',', digit1.parse_to()).parse_next(input)
    }

    fn parser(input: &mut &str) -> Result<Vec<Tile>> {
        terminated(separated(1.., corner, newline), multispace0).parse_next(input)
    }

    pub fn parse_input(input: &str) -> Vec<Tile> {
        parser.parse(input).expect("could not parse input")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parser() {
            let input = "123,123
123,123
";
            let expected = vec![(123, 123), (123, 123)];
            let actual = parser.parse(input).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
