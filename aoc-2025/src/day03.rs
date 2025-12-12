use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
struct BatteryBank {
    bank: Vec<u8>,
    active: VecDeque<usize>,
}

struct BankBuilder<C> {
    capacity: C,
}

impl Default for BankBuilder<usize> {
    fn default() -> Self {
        BankBuilder { capacity: 2 }
    }
}

impl<C> BankBuilder<C> {
    fn with_capacity(self, capacity: usize) -> BankBuilder<usize> {
        BankBuilder { capacity }
    }
}

impl BankBuilder<usize> {
    fn build_from_str(&self, bank: &str) -> BatteryBank {
        let bank = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        BatteryBank {
            bank,
            active: (0..self.capacity).collect(),
        }
    }
}

fn parse_input(input: &str, builder: &BankBuilder<usize>) -> Vec<BatteryBank> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(builder.build_from_str(l))
            }
        })
        .collect()
}

impl BatteryBank {
    fn joltage(&self) -> u128 {
        self.active
            .iter()
            .fold(0u128, |acc, idx| acc * 10 + self.bank[*idx] as u128)
    }
}

fn select_batteries(bank: BatteryBank) -> BatteryBank {
    let capacity = bank.active.len();
    let total = bank.bank.len();

    let mut active = VecDeque::new();
    for active_idx in 0..capacity {
        let options = match active.back() {
            Some(&i) => i + 1..=total - capacity + active_idx,
            None => 0..=total - capacity + active_idx,
        };
        let idx = options.rev().max_by_key(|&i| bank.bank[i]).unwrap();
        active.push_back(idx);
    }
    BatteryBank {
        bank: bank.bank,
        active,
    }
}

pub fn part1(input: &str) -> u128 {
    let builder = BankBuilder::default();
    let parsed = parse_input(input, &builder);
    parsed
        .into_iter()
        .map(|b| select_batteries(b).joltage())
        .sum()
}

pub fn part2(input: &str) -> u128 {
    let builder = BankBuilder::default().with_capacity(12);
    let parsed = parse_input(input, &builder);
    parsed
        .into_iter()
        .map(|b| select_batteries(b).joltage())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_parse_input() {
        let bank_builder = BankBuilder::default().with_capacity(2);
        let input = "12\n34";
        let parsed = parse_input(input, &bank_builder);
        assert_eq!(
            parsed,
            vec![
                BatteryBank {
                    bank: vec![1, 2],
                    active: vec![0, 1].into_iter().collect(),
                },
                BatteryBank {
                    bank: vec![3, 4],
                    active: vec![0, 1].into_iter().collect(),
                }
            ]
        );
    }

    #[rstest]
    fn test_joltage() {
        let bank = BatteryBank {
            bank: vec![1, 2, 3],
            active: vec![0, 2].into_iter().collect(),
        };
        assert_eq!(bank.joltage(), 13);
    }

    #[rstest]
    fn test_part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        let expected = 357;
        let actual = part1(input);
        assert_eq!(actual, expected, "{} {}", expected, actual);
    }

    #[rstest]
    fn test_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        let expected = 3121910778619;
        let actual = part2(input);
        assert_eq!(actual, expected, "{} {}", expected, actual);
    }
}
