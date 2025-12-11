type Input = (u64, u64);

const SURROUNDING: [Position; 8] = [
    Position(1, 0),
    Position(1, 1),
    Position(0, 1),
    Position(-1, 1),
    Position(-1, 0),
    Position(-1, -1),
    Position(0, -1),
    Position(1, -1),
];

#[derive(Copy, Clone, Debug)]
struct Position(isize, isize);

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Grid<T> {
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

fn check_blocked(pos: Position, grid: &Grid<u8>) -> bool {
    if grid[pos] == b'.' {
        return false;
    }
    SURROUNDING
        .iter()
        .filter(|&offset| grid.contains(*offset + pos) && grid[*offset + pos] == b'@')
        .count()
        >= 4
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
    let mut grid = Grid {
        items,
        width,
        height,
    };

    let mut occupied: Vec<_> = grid
        .items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if *item == b'@' {
                Some(index_to_position(i, width))
            } else {
                None
            }
        })
        .collect();

    let mut part_1 = 0;
    let mut removed = 0;
    for i in 0.. {
        let (blocked, unblocked): (Vec<_>, Vec<_>) = occupied
            .into_iter()
            .partition(|&pos| check_blocked(pos, &grid));

        removed += unblocked.len() as u64;
        if i == 0 {
            part_1 = removed;
        }

        if unblocked.is_empty() {
            break;
        }

        for pos in unblocked {
            grid[pos] = b'.';
        }

        occupied = blocked;
    }
    (part_1, removed)
}

fn index_to_position(index: usize, width: usize) -> Position {
    Position((index / width) as isize, (index % width) as isize)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
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
