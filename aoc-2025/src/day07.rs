use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    None,
    Start,
    Splitter,
}

struct Grid {
    items: Vec<Vec<Cell>>,
}

type Pos = (usize, usize);

impl Grid {
    fn start(&self) -> Pos {
        let r = 0;
        let c = self.items[0]
            .iter()
            .position(|item| *item == Cell::Start)
            .expect("could not find start");
        (r, c)
    }

    fn size(&self) -> (usize, usize) {
        (self.items.len(), self.items[0].len())
    }
}

fn init_ray_counts(start: usize, width: usize) -> Vec<u64> {
    let mut ray_counts = vec![0u64; width];
    ray_counts[start] = 1;
    ray_counts
}

pub fn part1(input: &str) -> u32 {
    let grid = parse::parse_input(input);
    let (_, w) = grid.size();

    let start = grid.start().1;

    let ray_counts = init_ray_counts(start, w);

    let mut split_count = 0;
    grid.items[1..]
        .iter()
        .fold(ray_counts, |mut ray_counts, row| {
            for idx in row.iter().positions(|c| matches!(c, Cell::Splitter)) {
                if ray_counts[idx] > 0 {
                    let tmp_count = std::mem::replace(&mut ray_counts[idx], 0);
                    for offset in [-1, 1] {
                        if let Some(column) = idx.checked_add_signed(offset as isize)
                            && column < w
                            && row[column] == Cell::None
                        {
                            ray_counts[column] += tmp_count;
                        }
                    }
                    split_count += 1;
                }
            }
            ray_counts
        })
        .len();
    split_count
}

pub fn part2(input: &str) -> u64 {
    let grid = parse::parse_input(input);
    let (_, w) = grid.size();

    let start = grid.start();

    let ray_counts = init_ray_counts(start.1, w);

    grid.items[1..]
        .iter()
        .fold(ray_counts, |mut ray_counts, row| {
            for (idx, count) in ray_counts
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(_, c)| *c > 0)
            {
                if row[idx] == Cell::Splitter {
                    for offset in [-1, 1] {
                        if let Some(column) = idx.checked_add_signed(offset as isize)
                            && column < w
                            && row[column] == Cell::None
                        {
                            ray_counts[column] += count;
                        }
                    }
                    ray_counts[idx] = 0;
                }
            }

            ray_counts
        })
        .iter()
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    const SAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[rstest]
    fn test_part1() {
        let actual = part1(SAMPLE_INPUT);
        let expected = 21;
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_part2() {
        let actual = part2(SAMPLE_INPUT);
        let expected = 40;
        assert_eq!(actual, expected);
    }
}

mod parse {
    use super::{Cell, Grid};
    use winnow::ascii::newline;
    use winnow::combinator::{alt, repeat, separated};
    use winnow::token::rest;
    use winnow::{Parser, Result};

    fn cell(input: &mut &str) -> Result<Cell> {
        alt((
            '.'.value(Cell::None),
            'S'.value(Cell::Start),
            '^'.value(Cell::Splitter),
        ))
        .parse_next(input)
    }

    fn grid_parser(input: &mut &str) -> Result<Vec<Vec<Cell>>> {
        let grid: Vec<Vec<Cell>> = separated(
            1..,
            repeat::<&str, Cell, Vec<Cell>, _, _>(1.., cell),
            newline,
        )
        .parse_next(input)?;

        let _ = rest.parse_next(input)?;

        Ok(grid)
    }

    pub(crate) fn parse_input(input: &str) -> Grid {
        let items = grid_parser.parse(input).expect("could not read input");
        Grid { items }
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        use rstest::rstest;

        const SAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

        #[rstest]
        fn test_parse_input() {
            let grid = parse_input(SAMPLE_INPUT);

            let start = grid.start();
            assert_eq!(start, (0, 7));
        }
    }
}
