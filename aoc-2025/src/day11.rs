pub type Input = Graph;

#[derive(Debug)]
struct NodeIndexer {
    mapping: Vec<Option<usize>>,
    size: usize,
}

#[inline]
fn node_to_index(node: &str) -> usize {
    node.bytes()
        .take(3)
        .fold(0, |acc, b| 26 * acc + usize::from(b - b'a'))
}

impl NodeIndexer {
    fn new() -> Self {
        let mapping = vec![None; 26usize.pow(3)];
        Self { mapping, size: 0 }
    }

    #[inline]
    fn get_or_create_index(&mut self, node: &str) -> usize {
        let idx = node_to_index(node);
        if let Some(mapped) = self.mapping[idx] {
            return mapped;
        }
        let prev_size = self.size;
        self.size += 1;
        self.mapping[idx] = Some(prev_size);
        prev_size
    }

    #[inline]
    fn index_of(&self, node: &str) -> Option<usize> {
        let idx = node_to_index(node);
        self.mapping[idx]
    }
}

type Mapping = Vec<Vec<usize>>;

#[derive(Debug)]
pub struct Graph {
    mapping: Mapping,
    indexer: NodeIndexer,
}

pub fn process(input: &'_ str) -> Input {
    let lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();

    let mut mapping = vec![vec![]; lines.len() + 1];
    let mut indexer = NodeIndexer::new();
    for line in lines {
        let mut nodes = line.split_ascii_whitespace();

        let from = nodes.next().expect("no start node");
        let from = indexer.get_or_create_index(from);

        let to: Vec<_> = nodes
            .map(|node_str| indexer.get_or_create_index(node_str))
            .collect();
        mapping[from] = to;
    }

    Graph { mapping, indexer }
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

fn find_path(from: &str, to: &str, graph: &Graph) -> u64 {
    let indexer = &graph.indexer;
    let mut cache = vec![None; graph.mapping.len()];
    dfs(
        indexer.index_of(from).expect("from not found"),
        indexer.index_of(to).expect("to not found"),
        &graph.mapping,
        &mut cache,
    )
}

pub fn part1(input: &Input) -> u64 {
    find_path("you", "out", input)
}

pub fn part2(input: &Input) -> u64 {
    let svr_fft = find_path("svr", "fft", input);
    let svr_dac = find_path("svr", "dac", input);
    let dac_fft = find_path("dac", "fft", input);
    let fft_dac = find_path("fft", "dac", input);
    let dac_out = find_path("dac", "out", input);
    let fft_out = find_path("fft", "out", input);

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
