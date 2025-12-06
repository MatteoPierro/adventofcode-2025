use adventofcode_2025::read_input_from_file;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::{
        find_sum_problems_answer, find_sum_right_to_left_problems_answer, parse_operations,
    };
    use indoc::indoc;
    use itertools::Itertools;

    #[test]
    fn it_finds_sum_problems_answer() {
        let input = indoc! {"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +"};

        assert_eq!(find_sum_problems_answer(input), 4277556);
    }

    #[test]
    fn it_finds_sum_right_to_left_problems_answer() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +";

        assert_eq!(find_sum_right_to_left_problems_answer(input), 3263827);
    }
}

fn find_sum_right_to_left_problems_answer(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let raw_numbers = lines[0..lines.len() - 1].to_vec();

    let raws = raw_numbers
        .iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect())
        .reduce(|acc: Vec<_>, chars| {
            acc.into_iter()
                .zip(chars)
                .map(|(a, b)| format!("{a}{b}"))
                .collect_vec()
        })
        .unwrap();

    let problem_operands: Vec<_> = raws.split(|l| l.replace(" ", "").is_empty()).collect();

    let (operations, mut values) = parse_operations(&lines);

    for (problem_index, problem) in problem_operands.iter().enumerate() {
        let operation = operations[problem_index];
        values[problem_index] = problem
            .iter()
            .map(|n| n.trim().parse::<usize>().unwrap())
            .reduce(|a, b| operation(a, b))
            .unwrap();
    }

    values.iter().sum::<usize>()
}

fn find_sum_problems_answer(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    let (operations, mut values) = parse_operations(&lines);

    for line in &lines[0..lines.len() - 1] {
        let nums = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        for (i, num) in nums.iter().enumerate() {
            values[i] = operations[i](values[i], *num);
        }
    }

    values.iter().sum::<usize>()
}

fn parse_operations(lines: &Vec<&str>) -> (Operations, Vec<usize>) {
    let raw_operations = lines.last().unwrap().split_whitespace().collect::<String>();

    let mut operations: Operations = vec![];
    let mut values: Vec<usize> = vec![];
    for operation in raw_operations.chars() {
        match operation {
            '+' => {
                operations.push(|a, b| a + b);
                values.push(0);
            }
            '*' => {
                operations.push(|a, b| a * b);
                values.push(1);
            }
            _ => panic!("Unknown operation"),
        }
    }
    (operations, values)
}

type Operation = fn(usize, usize) -> usize;
type Operations = Vec<Operation>;

fn main() {
    let input = read_input_from_file();

    println!(
        "first part {}, second part {}",
        find_sum_problems_answer(&input),
        find_sum_right_to_left_problems_answer(&input)
    );
}
