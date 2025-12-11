use std::collections::HashMap;

type Input = Graph;

struct NodeIndexer<'i> {
    mapping: HashMap<&'i str, usize>,
}

impl<'i> NodeIndexer<'i> {
    fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    fn index_of(&mut self, node: &'i str) -> usize {
        let size = self.mapping.len();
        *self.mapping.entry(node).or_insert(size)
    }
}

type Mapping = Vec<Vec<usize>>;

#[derive(Debug)]
pub struct Graph {
    mapping: Mapping,
    start: usize,
    end: usize,

    dac: usize,
    fft: usize,
    svr: usize,
}

pub fn process(input: &str) -> Graph {
    let lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();

    let mut start = None;
    let mut end = None;
    let mut dac = None;
    let mut fft = None;
    let mut svr = None;
    let mut mapping = vec![vec![]; lines.len() + 1];
    let mut indexer = NodeIndexer::new();
    for line in lines {
        let (from_str, to_str) = line.split_once(": ").expect("");

        let from = indexer.index_of(from_str);

        match from_str {
            "you" => start = Some(from),
            "fft" => fft = Some(from),
            "dac" => dac = Some(from),
            "svr" => svr = Some(from),
            _ => {}
        }

        let to: Vec<_> = to_str
            .split(" ")
            .map(|node_str| {
                let node = indexer.index_of(node_str);
                if node_str == "out" {
                    end = Some(node);
                }
                node
            })
            .collect();
        mapping[from] = to;
    }

    Graph {
        mapping,
        start: start.expect("start not found"),
        end: end.expect("end not found"),
        dac: dac.expect("dac not found"),
        fft: fft.expect("fft not found"),
        svr: svr.expect("svr not found"),
    }
}

fn walk(start: usize, graph: &Graph) -> u64 {
    if start == graph.end {
        return 1;
    }
    let to = &graph.mapping[start];
    to.iter().map(|t| walk(*t, graph)).sum::<u64>()
}

fn dfs(start: usize, end: usize, mapping: &Mapping, cache: &mut [Option<u64>]) -> u64 {
    if let Some(cached) = cache[start] {
        return cached;
    }

    if start == end {
        cache[start] = Some(1);
        return 1;
    }

    let sum = mapping[start]
        .iter()
        .map(|t| dfs(*t, end, mapping, cache))
        .sum::<u64>();
    cache[start] = Some(sum);
    sum
}

fn find_path(from: usize, to: usize, mapping: &Mapping) -> u64 {
    let mut cache = vec![None; mapping.len()];
    dfs(from, to, mapping, &mut cache)
}

pub fn part1(input: &Input) -> u64 {
    let start = input.start;
    walk(start, input)
}

pub fn part2(input: &Input) -> u64 {
    let svr_fft = find_path(input.svr, input.fft, &input.mapping);
    let svr_dac = find_path(input.svr, input.dac, &input.mapping);
    let dac_fft = find_path(input.dac, input.fft, &input.mapping);
    let fft_dac = find_path(input.fft, input.dac, &input.mapping);
    let dac_out = find_path(input.dac, input.end, &input.mapping);
    let fft_out = find_path(input.fft, input.end, &input.mapping);

    svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part1() {
        let input = process(SAMPLE_INPUT);
        let actual = part1(&input);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
