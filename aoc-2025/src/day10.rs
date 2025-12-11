use itertools::Itertools;

type Input = Vec<Manual>;

#[derive(Debug, PartialEq)]
struct Lights {
    repr: u32,
    size: usize,
}

impl Lights {
    fn at(&self, index: usize) -> bool {
        let bit = self.repr >> index & 1;
        bit == 1
    }
}

#[derive(Debug, PartialEq)]
pub struct Manual {
    lights: Lights,
    buttons: Vec<Lights>,
    joltage: Vec<u64>,
}

pub fn process(input: &str) -> Input {
    parse::parse_input(input)
}

fn solve_lights_contrained(lights: &Lights, buttons: &[Lights]) -> Option<u64> {
    for i in 0..buttons.len() {
        let combinations = buttons.iter().combinations(i);
        for comb in combinations {
            let res = comb.iter().fold(
                Lights {
                    repr: 0,
                    size: lights.size,
                },
                |acc, b| Lights {
                    repr: acc.repr ^ b.repr,
                    ..acc
                },
            );
            if res == *lights {
                return Some(comb.len() as u64);
            }
        }
    }
    None
}

fn solve_joltage_constrained(buttons: &[Lights], joltages: &[u64]) -> Option<u64> {
    let solver = z3::Optimize::new();
    let vars: Vec<_> = (0..buttons.len())
        .map(|j| {
            let presses = z3::ast::Int::fresh_const(&format!("count_{}", j));
            solver.assert(&presses.ge(0));
            presses
        })
        .collect();
    let objective = vars.iter().sum::<z3::ast::Int>();

    for (i, target) in joltages.iter().enumerate() {
        let joltage_sum = buttons
            .iter()
            .zip(vars.iter())
            .map(|(b, v)| v * (b.at(i) as u64))
            .sum::<z3::ast::Int>();
        solver.assert(&joltage_sum.eq(*target));
    }

    solver.minimize(&objective);

    match solver.check(&[]) {
        z3::SatResult::Sat => {
            let model = solver.get_model().expect("no model");
            model.eval(&objective, true)?.as_u64()
        }
        _ => unreachable!(),
    }
}

pub fn part1(input: &Input) -> u64 {
    input
        .iter()
        .map(|manual| {
            solve_lights_contrained(&manual.lights, &manual.buttons).expect("No solution found")
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    input
        .iter()
        .map(|manual| {
            solve_joltage_constrained(&manual.buttons, &manual.joltage).expect("No solution found")
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        let input = process(SAMPLE_INPUT);
        let actual = part1(&input);
        let expected = 7;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let input = process(SAMPLE_INPUT);
        let actual = part2(&input);
        let expected = 33;
        assert_eq!(expected, actual);
    }
}

mod parse {
    use crate::day10::{Lights, Manual};
    use winnow::Parser as _;

    fn lights(input: &mut &str) -> winnow::Result<Lights> {
        let lights: Vec<_> = winnow::combinator::delimited(
            '[',
            winnow::combinator::repeat(
                1..,
                winnow::combinator::alt(('#'.value(true), '.'.value(false))),
            ),
            ']',
        )
        .parse_next(input)?;

        let size = lights.len();
        let lights =
            lights
                .into_iter()
                .enumerate()
                .fold(Lights { repr: 0, size }, |acc, (i, l)| {
                    let repr = (l as u32) << i ^ acc.repr;
                    Lights { repr, size }
                });
        Ok(lights)
    }

    fn button(size: usize) -> impl FnMut(&mut &str) -> winnow::Result<Lights> {
        move |input: &mut &str| {
            let button: Vec<u32> = winnow::combinator::delimited(
                '(',
                winnow::combinator::separated(1.., winnow::ascii::digit1.parse_to::<u32>(), ','),
                ')',
            )
            .parse_next(input)?;
            let button = button.into_iter().fold(Lights { repr: 0, size }, |acc, b| {
                let repr = 1 << b ^ acc.repr;
                Lights { repr, size }
            });
            Ok(button)
        }
    }

    fn joltage(input: &mut &str) -> winnow::Result<Vec<u64>> {
        winnow::combinator::delimited(
            '{',
            winnow::combinator::separated(1.., winnow::ascii::digit1.parse_to::<u64>(), ','),
            '}',
        )
        .parse_next(input)
    }

    fn manual(input: &mut &str) -> winnow::Result<Manual> {
        let lights = lights.parse_next(input)?;

        winnow::ascii::space1.parse_next(input)?;

        let buttons: Vec<Lights> =
            winnow::combinator::separated(1.., button(lights.size), winnow::ascii::space1)
                .parse_next(input)?;

        winnow::ascii::space1.parse_next(input)?;

        let joltage = joltage.parse_next(input)?;

        Ok(Manual {
            lights,
            buttons,
            joltage,
        })
    }

    fn parser(input: &mut &str) -> winnow::Result<Vec<Manual>> {
        let manuals =
            winnow::combinator::separated(1.., manual, winnow::ascii::newline).parse_next(input)?;
        winnow::ascii::multispace0.parse_next(input)?;
        Ok(manuals)
    }

    pub fn parse_input(input: &str) -> Vec<Manual> {
        parser.parse(input).expect("could not parse input")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_input() {
            let input = "[###.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
";

            let expected = vec![Manual {
                lights: Lights {
                    repr: 0b0111,
                    size: 4,
                },
                buttons: vec![
                    Lights {
                        repr: 0b1000,
                        size: 4,
                    },
                    Lights {
                        repr: 0b1010,
                        size: 4,
                    },
                    Lights {
                        repr: 0b0100,
                        size: 4,
                    },
                    Lights {
                        repr: 0b1100,
                        size: 4,
                    },
                    Lights {
                        repr: 0b0101,
                        size: 4,
                    },
                    Lights {
                        repr: 0b0011,
                        size: 4,
                    },
                ],
                joltage: vec![3, 5, 4, 7],
            }];
            let actual = parse_input(input);
            assert_eq!(expected, actual);
        }
    }
}
