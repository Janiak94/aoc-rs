type Input = u32;

const UPPER_BOUND: u32 = 9;

pub fn process(input: &str) -> u32 {
    let (_presents, objectives) = input.rsplit_once("\n\n").expect("Invalid input");

    objectives
        .lines()
        .filter(|line| {
            let (uni, n_presents) = line.split_once(": ").expect("Invalid line");

            let w = uni[0..2].parse::<u32>().expect("Invalid line");
            let h = uni[3..5].parse::<u32>().expect("Invalid line");
            let size = w * h;

            UPPER_BOUND
                * n_presents
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().expect("Invalid line"))
                    .sum::<u32>()
                <= size
        })
        .count() as u32
}

pub fn part1(input: &Input) -> u32 {
    *input
}
