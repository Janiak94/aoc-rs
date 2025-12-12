use aoc_2025::*;

fn run_days() {
    aoc!(day01 => part1, part2);
    aoc!(day02 => part1, part2);
    aoc!(day03 => part1, part2);
    aoc!(day04 => process => part1, part2);
    aoc!(day05 => part1, part2);
    aoc!(day06 => part1, part2);
    aoc!(day07 => part1, part2);
    aoc!(day08 => process => part1, part2);
    aoc!(day09 => process => part1, part2);
    aoc!(day10 => process => part1, part2);
    aoc!(day11 => process => part1, part2);
    aoc!(day12 => process => part1);
}

fn main() {
    let start = std::time::Instant::now();
    run_days();
    let elapsed = start.elapsed();
    println!("Total elapsed: {:.3?}", elapsed);
}
