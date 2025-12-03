use itertools::Itertools;
use std::{env, fs};

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

        assert_eq!(total_joltage(banks), 357);
    }
}

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("Please provide input as first argument"),
    )
    .expect("input");

    println!("result: {}", total_joltage(&input));
}

fn total_joltage(banks: &str) -> usize {
    banks
        .lines()
        .map(|line| find_maximum_joltage(line))
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
