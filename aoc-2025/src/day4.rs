type Input = (Vec<Position>, Grid<u8>);

const SURROUNDING: [Position; 8] = [
    Position(-1, 1),
    Position(0, 1),
    Position(1, 1),
    Position(-1, 0),
    Position(1, 0),
    Position(-1, -1),
    Position(0, -1),
    Position(1, -1),
];

#[derive(Copy, Clone, Debug)]
pub struct Position(isize, isize);

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    items: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn contains(&self, pos: Position) -> bool {
        pos.0 >= 0 && pos.0 < self.height as isize && pos.1 >= 0 && pos.1 < self.width as isize
    }
}

impl<T> std::ops::Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self.items[index.0 as usize * self.width + index.1 as usize]
    }
}

impl<T> std::ops::IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.items[index.0 as usize * self.width + index.1 as usize]
    }
}

pub fn process(input: &str) -> Input {
    let mut lines = input.lines();
    let first_line = lines.next().expect("No first line");
    let items = lines.filter(|line| !line.is_empty()).fold(
        Vec::from_iter(first_line.bytes()),
        |mut acc, line| {
            acc.extend(line.bytes());
            acc
        },
    );

    let width = first_line.len();
    let height = items.len() / width;
    let in_grid = Grid {
        items,
        width,
        height,
    };
    let mut grid = Grid {
        items: vec![u8::MAX; width * height],
        width,
        height,
    };

    let mut to_remove = Vec::with_capacity(width * height);
    for i in 0..width as isize {
        for j in 0..height as isize {
            let pos = Position(i, j);

            if in_grid[pos] == b'@' {
                let n_count = SURROUNDING
                    .iter()
                    .map(|&p| p + pos)
                    .filter(|&n| in_grid.contains(n) && in_grid[n] == b'@')
                    .count() as u8;
                if n_count < 4 {
                    to_remove.push(pos);
                }
                grid[pos] = n_count;
            }
        }
    }
    (to_remove, grid)
}

pub fn part1(input: &Input) -> u64 {
    input.0.len() as u64
}

pub fn part2(input: &Input) -> u64 {
    let (mut to_remove, mut grid) = input.clone();

    let mut removed = 0;
    while let Some(pos) = to_remove.pop() {
        removed += 1;

        for next in SURROUNDING.iter().map(|&p| pos + p) {
            if grid.contains(next) {
                if grid[next] == 4 {
                    to_remove.push(next);
                }

                grid[next] -= 1;
            }
        }
    }
    removed
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const SAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[rstest]
    fn test_part1() {
        let input = process(SAMPLE_INPUT);
        let expected = 13;
        let actual = part1(&input);
        assert_eq!(actual, expected, "expected={}, actual={}", expected, actual)
    }

    #[rstest]
    fn test_part2() {
        let input = process(SAMPLE_INPUT);

        let expected = 43;
        let actual = part2(&input);
        assert_eq!(actual, expected, "expected={}, actual={}", expected, actual)
    }
}
