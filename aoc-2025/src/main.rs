use aoc_utils::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    aoc!(
        2025 day01 => part1, part2;
        2025 day02 => part1, part2;
        2025 day03 => part1, part2;
        2025 day04 => process => part1, part2;
        2025 day05 => part1, part2;
        2025 day06 => part1, part2;
        2025 day07 => part1, part2;
        2025 day08 => process => part1, part2;
        2025 day09 => process => part1, part2;
        2025 day10 => process => part1, part2;
        2025 day11 => process => part1, part2;
        2025 day12 => process => part1;
    );
}
