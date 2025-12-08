use itertools::Itertools;

type Input = (u32, u32);

#[cfg(test)]
const LIMIT: usize = 10;

#[cfg(not(test))]
const LIMIT: usize = 1000;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Junction(u32, u32, u32);

#[derive(Debug)]
struct Edge<'j>(&'j Junction, &'j Junction);

impl Junction {
    #[inline]
    fn distance(&self, other: &Self) -> u64 {
        (self.0 as u64).abs_diff(other.0 as u64).pow(2)
            + (self.1 as u64).abs_diff(other.1 as u64).pow(2)
            + (self.2 as u64).abs_diff(other.2 as u64).pow(2)
    }
}

impl Edge<'_> {
    #[inline]
    fn distance(&self) -> u64 {
        self.0.distance(self.1)
    }
}

type Circuit = Vec<Junction>;

struct Circuits {
    circuits: Vec<Circuit>,
}

impl Circuits {
    fn new(junctions: &[Junction]) -> Self {
        let circuits = junctions.iter().map(|j| vec![*j]).collect();
        Self { circuits }
    }

    fn add_connection(&mut self, edge: &Edge) {
        let circuits = self.circuits.drain(..);
        let (to_merge, rest): (Vec<_>, Vec<_>) =
            circuits.partition(|c| c.contains(edge.0) || c.contains(edge.1));
        assert!(to_merge.len() <= 2);

        let merged = to_merge.into_iter().reduce(|mut acc, c| {
            acc.extend(c);
            acc
        });
        self.circuits.extend(rest.into_iter().chain(merged));
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<Junction>> {
        self.circuits.iter()
    }

    #[inline]
    fn len(&self) -> usize {
        self.circuits.len()
    }
}

fn edges_sorted(junctions: &[Junction]) -> impl Iterator<Item = Edge<'_>> {
    let len = junctions.len();
    let num_connections = len * (len - 1) / 2;
    let mut edges = Vec::with_capacity(num_connections);
    for (i, j1) in junctions.iter().enumerate().skip(1) {
        for j2 in junctions.iter().take(i) {
            edges.push(Edge(j1, j2));
        }
    }
    assert_eq!(edges.len(), num_connections);
    edges
        .into_iter()
        .sorted_unstable_by(|e1, e2| e1.distance().partial_cmp(&e2.distance()).unwrap())
}

pub fn process(input: &str) -> Input {
    let junctions = parse::parse_input(input);

    let sorted = edges_sorted(&junctions);

    let mut connections = sorted;
    let mut circuits = Circuits::new(&junctions);
    for edge in connections.by_ref().take(LIMIT) {
        circuits.add_connection(&edge);
    }
    let part1 = circuits
        .iter()
        .map(|c| c.len() as u32)
        .sorted_unstable()
        .rev()
        .take(3)
        .product();

    for edge in connections {
        circuits.add_connection(&edge);
        if circuits.len() == 1 {
            let part2 = edge.0.0 * edge.1.0;
            return (part1, part2);
        }
    }
    unreachable!()
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const SAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[rstest]
    fn test_process() {
        process(SAMPLE_INPUT);
    }

    #[rstest]
    fn test_part1() {
        let actual = part1(&process(SAMPLE_INPUT));
        let expected = 40;
        assert_eq!(expected, actual);
    }

    #[rstest]
    fn test_part2() {
        let actual = part2(&process(SAMPLE_INPUT));
        let expected = 25272;
        assert_eq!(expected, actual);
    }
}

mod parse {

    use winnow::{
        ModalResult, Parser,
        ascii::{digit1, newline},
        combinator::separated,
        token::rest,
    };

    use crate::day8::Junction;

    fn junction(input: &mut &str) -> ModalResult<Junction> {
        let numbers: Vec<_> = separated(3, digit1.parse_to::<u32>(), ',').parse_next(input)?;
        Ok(Junction(numbers[0], numbers[1], numbers[2]))
    }

    fn parser(input: &mut &str) -> ModalResult<Vec<Junction>> {
        let junctions: Vec<_> = separated(1.., junction, newline).parse_next(input)?;
        rest.parse_next(input)?;
        Ok(junctions)
    }

    pub fn parse_input(input: &str) -> Vec<Junction> {
        parser.parse(input).unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        fn test_parse_input() {
            let input = "162,817,812
57,618,57
906,360,560

";
            let actual = parse_input(input);
            let expected = vec![
                Junction(162, 817, 812),
                Junction(57, 618, 57),
                Junction(906, 360, 560),
            ];
            assert_eq!(actual, expected);
        }
    }
}
