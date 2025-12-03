use itertools::Itertools;
use std::{env, fs, result};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_largest_pair() {
        assert_eq!(find_maximum_joltage("12345"), 45);
        assert_eq!(find_maximum_joltage("811111111111119"), 89);
        assert_eq!(find_maximum_joltage("987654321111111"), 98);
        assert_eq!(find_maximum_joltage("818181911112111"), 92);
    }

    #[test]
    fn it_calculates_the_total_joltage() {
        let banks = indoc::indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111"};

        assert_eq!(total_joltage(banks, find_maximum_joltage), 357);
    }

    #[test]
    fn test_something() {
        assert_eq!(something("987654321111111"), 987654321111);
        assert_eq!(something("811111111111119"), 811111111119);
        assert_eq!(something("234234234234278"), 434234234278);
        assert_eq!(something("818181911112111"), 888911112111);
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
        total_joltage(&input, find_maximum_joltage),
        total_joltage(&input, something)
    );
}

fn something(bank: &str) -> usize {
    let batteries = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    something_rec(12, &batteries, vec![])
}

fn something_rec(digits_left: usize, batteries: &[usize], mut result: Vec<usize>) -> usize {
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
            return something_rec(
                digits_left - 1,
                &batteries[pos + 1..batteries.len()],
                result,
            );
        }
    }

    unreachable!("should not reach here");
}

fn total_joltage(banks: &str, maximum_joltage_finder: fn(&str) -> usize) -> usize {
    banks
        .lines()
        .map(|line| (maximum_joltage_finder)(line))
        .sum::<usize>()
}

fn find_maximum_joltage(bank: &str) -> usize {
    let batteries = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let max_battery = batteries.iter().max().unwrap();
    let (pos, _) = batteries
        .iter()
        .find_position(|x| *x == max_battery)
        .unwrap();

    if pos == batteries.len() - 1 {
        let first = batteries[0..pos].iter().max().unwrap();

        return first * 10 + max_battery;
    }

    let second = batteries[pos + 1..batteries.len()].iter().max().unwrap();

    max_battery * 10 + second
}
