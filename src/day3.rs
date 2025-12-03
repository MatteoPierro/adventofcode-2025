use itertools::Itertools;
use std::{env, fs};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_calculates_bank_joltage() {
        assert_eq!(bank_joltage("12345", 2), 45);
        assert_eq!(bank_joltage("811111111111119", 2), 89);
        assert_eq!(bank_joltage("987654321111111", 2), 98);
        assert_eq!(bank_joltage("818181911112111", 2), 92);

        assert_eq!(bank_joltage("987654321111111", 12), 987654321111);
        assert_eq!(bank_joltage("811111111111119", 12), 811111111119);
        assert_eq!(bank_joltage("234234234234278", 12), 434234234278);
        assert_eq!(bank_joltage("818181911112111", 12), 888911112111);
    }

    #[test]
    fn it_calculates_the_total_joltage() {
        let banks = indoc::indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111"};

        assert_eq!(total_joltage(banks, 2), 357);
    }
}

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("Please provide input as first argument"),
    )
    .expect("input");

    println!(
        "first part: {}, second part: {}",
        total_joltage(&input, 2),
        total_joltage(&input, 12)
    );
}

fn total_joltage(banks: &str, length: usize) -> usize {
    banks
        .lines()
        .map(|bank| bank_joltage(bank, length))
        .sum::<usize>()
}

fn bank_joltage(bank: &str, length: usize) -> usize {
    let batteries = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    bank_joltage_rec(length, &batteries, vec![])
}

fn bank_joltage_rec(digits_left: usize, batteries: &[usize], mut result: Vec<usize>) -> usize {
    if digits_left == 0 {
        return result.iter().enumerate().fold(0, |acc, (i, v)| {
            acc + v * 10_usize.pow((result.len() - i - 1) as u32)
        });
    }

    for battery in (1..=9).rev() {
        let Some((pos, _)) = batteries.iter().find_position(|&x| *x == battery) else {
            continue;
        };

        if batteries.len() - pos >= digits_left {
            result.push(battery);
            return bank_joltage_rec(
                digits_left - 1,
                &batteries[pos + 1..batteries.len()],
                result,
            );
        }
    }

    unreachable!("should not reach here");
}
