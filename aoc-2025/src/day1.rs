use std::str::FromStr;

use winnow::error::InputError;
use winnow::prelude::*;
use winnow::token::one_of;

struct Dial {
    num: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self { num: 50 }
    }
}

impl Dial {
    fn rotate(&mut self, rot: Rotation) {
        self.num = match rot {
            Rotation::Left(count) => self.num - count,
            Rotation::Right(count) => self.num + count,
        }
        .rem_euclid(100);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s;
        let direction = one_of(['L', 'R'])
            .parse_next(&mut s)
            .map_err(|_: InputError<&str>| "Could not parse direction".to_string())?;
        let count = s[..]
            .parse::<i32>()
            .map_err(|_| "Could not parse count".to_owned())?;
        match direction {
            'L' => Ok(Self::Left(count)),
            'R' => Ok(Self::Right(count)),
            _ => unreachable!(),
        }
    }
}

fn calc_pass_zero_count(dial: i32, rot: Rotation) -> i32 {
    use Rotation::*;
    match rot {
        Left(count) => {
            let full_rotations = count.div_euclid(100);
            let rem = count.rem_euclid(100);

            let crosses = (dial - rem <= 0 && rem != 0 && dial != 0) as i32;
            full_rotations + crosses
        }
        Right(count) => {
            let full_rotations = count.div_euclid(100);
            let rem = count.rem_euclid(100);

            let crosses = (dial + rem >= 100 && rem != 0 && dial != 0) as i32;
            full_rotations + crosses
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut dial = Dial::default();
    let rotations = input
        .lines()
        .map(|r| Rotation::from_str(r).expect("parse line"));

    let mut z_count = 0;
    for rot in rotations {
        dial.rotate(rot);
        z_count += (dial.num == 0) as i32;
    }
    z_count
}

pub fn part2(input: &str) -> i32 {
    let mut dial = Dial::default();
    let rotations = input
        .lines()
        .map(|r| Rotation::from_str(r).expect("parse line"));

    let mut z_count = 0;
    for rot in rotations {
        z_count += calc_pass_zero_count(dial.num, rot);
        dial.rotate(rot);
    }
    z_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_rotation_from_string() {
        let rotation = Rotation::from_str("L12").unwrap();
        assert_eq!(rotation, Rotation::Left(12));
    }

    #[rstest]
    fn test_part1() {
        let input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .join("\n");
        let actual = part1(&input);
        assert_eq!(actual, 3);
    }

    #[rstest]
    fn test_part2() {
        let input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .join("\n");
        let actual = part2(&input);
        assert_eq!(actual, 6);
    }

    #[rstest]
    #[case(50, Rotation::Left(68), 1, 82)]
    #[case(82, Rotation::Left(30), 0, 52)]
    #[case(52, Rotation::Right(48), 1, 0)]
    #[case(0, Rotation::Left(5), 0, 95)]
    #[case(95, Rotation::Right(60), 1, 55)]
    #[case(55, Rotation::Left(55), 1, 0)]
    #[case(0, Rotation::Left(1), 0, 99)]
    #[case(99, Rotation::Left(99), 1, 0)]
    #[case(0, Rotation::Right(14), 0, 14)]
    #[case(14, Rotation::Left(82), 1, 32)]
    fn examples(
        #[case] dial: i32,
        #[case] rot: Rotation,
        #[case] expected: i32,
        #[case] end_dial: i32,
    ) {
        let actual = calc_pass_zero_count(dial, rot);
        assert_eq!(
            actual, expected,
            "count did not match actual={} expected={}",
            actual, expected
        );

        let mut dial = Dial { num: dial };
        dial.rotate(rot);
        assert_eq!(
            dial.num, end_dial,
            "dial did not match actual={} expected={}",
            dial.num, end_dial,
        );
    }
}
