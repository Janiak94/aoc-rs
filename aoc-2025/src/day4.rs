use std::collections::{HashMap, HashSet};

type Loc<T> = (T, T);

struct Layout {
    items: HashSet<Loc<usize>>,
}

impl Layout {
    fn contains(&self, loc: &Loc<usize>) -> bool {
        self.items.contains(loc)
    }

    fn iter(&self) -> impl Iterator<Item = &Loc<usize>> {
        self.items.iter()
    }

    fn neighbors<'a>(&'a self, loc: &'a Loc<usize>) -> NeighborIter<'a> {
        NeighborIter {
            layout: self,
            loc,
            idx: 0,
        }
    }

    fn remove(&mut self, loc: &Loc<usize>) {
        self.items.remove(loc);
    }
}

impl IntoIterator for Layout {
    type Item = Loc<usize>;
    type IntoIter = std::collections::hash_set::IntoIter<Loc<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

struct NeighborIter<'a> {
    layout: &'a Layout,
    loc: &'a Loc<usize>,
    idx: usize,
}

impl<'a> NeighborIter<'a> {
    fn idx_to_loc(&self) -> Result<Loc<usize>, ()> {
        let (row_idx, col_idx) = (self.idx / 3, self.idx % 3);
        Ok((
            (self.loc.0 + row_idx).checked_sub(1).ok_or(())?,
            (self.loc.1 + col_idx).checked_sub(1).ok_or(())?,
        ))
    }
}

impl<'a> Iterator for NeighborIter<'a> {
    type Item = Loc<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < 9 {
            if let Ok(loc) = self.idx_to_loc() {
                if self.layout.contains(&loc) && self.loc != &loc {
                    self.idx += 1;
                    return Some(loc);
                }
            }
            self.idx += 1;
        }
        None
    }
}

fn parse_input(input: &str) -> Layout {
    let items = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '@' => Some((row, col)),
                    _ => None,
                })
        })
        .collect();
    Layout { items }
}

fn get_neighbor_counts(layout: &Layout) -> HashMap<Loc<usize>, i32> {
    let mut neighbor_counts: HashMap<Loc<usize>, i32> = HashMap::new();
    for loc in layout.iter() {
        for neighbor in layout.neighbors(loc) {
            *neighbor_counts.entry(neighbor).or_default() += 1;
        }
    }
    neighbor_counts
}

fn removable_locations(layout: &Layout) -> Layout {
    let neighbor_counts = get_neighbor_counts(layout);
    let removables = neighbor_counts
        .iter()
        .filter_map(|(&c, &d)| if d < 4 { Some(c) } else { None });
    Layout {
        items: HashSet::from_iter(removables),
    }
}

pub fn part1(input: &str) -> i32 {
    let layout = parse_input(input);

    let neighbor_counts = get_neighbor_counts(&layout);
    neighbor_counts.values().filter(|&&c| c < 4).count() as i32
}

pub fn part2(input: &str) -> i32 {
    let mut layout = parse_input(input);

    let mut num_removed = 0;
    loop {
        let removables = removable_locations(&layout);
        if removables.items.is_empty() {
            break;
        }
        num_removed += removables.items.len() as i32;
        for r in removables {
            layout.remove(&r);
        }
    }
    num_removed
}

mod parse {
    use winnow::Parser;
    use winnow::Result;
    use winnow::ascii::line_ending;
    use winnow::combinator::alt;
    use winnow::combinator::delimited;
    use winnow::combinator::repeat;
    use winnow::combinator::separated;
    use winnow::combinator::seq;
    use winnow::prelude::*;

    fn line_parser(input: &mut &str) -> Result<Vec<bool>> {
        repeat(1.., alt(('@'.value(true), '.'.value(false)))).parse_next(input)
    }

    fn input_parser(input: &mut &str) -> Result<Vec<Vec<bool>>> {
        separated(1.., line_parser, line_ending).parse_next(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        fn test_parse() {
            let input = "@.
.@

";
            let parsed = input_parser.parse(input).unwrap();
            assert_eq!(parsed, vec![vec![true, false], vec![false, true]]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let expected = 13;
        let actual = part1(input);
        assert_eq!(actual, expected, "expected={}, actual={}", expected, actual)
    }

    #[rstest]
    fn test_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let expected = 43;
        let actual = part2(input);
        assert_eq!(actual, expected, "expected={}, actual={}", expected, actual)
    }
}
