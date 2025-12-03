use aoc_2025::*;

fn run_days() {
    aoc!(day1, part1, part2);
    aoc!(day2, part1, part2);
    aoc!(day3, part1, part2);
}

fn main() {
    let start = std::time::Instant::now();
    run_days();
    let elapsed = start.elapsed();
    println!("Total elapsed: {:.3?}", elapsed);
}
