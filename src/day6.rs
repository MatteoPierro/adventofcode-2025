use adventofcode_2025::read_input_from_file;

#[cfg(test)]
mod tests {
    use crate::find_sum_problems_answer;
    use indoc::indoc;

    #[test]
    fn it_finds_sum_problems_answer() {
        let input = indoc! {"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +"};

        assert_eq!(find_sum_problems_answer(input), 4277556);
    }
}

fn find_sum_problems_answer(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
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

type Operation = fn(usize, usize) -> usize;
type Operations = Vec<Operation>;

fn main() {
    let input = read_input_from_file();

    println!("The answer is {}", find_sum_problems_answer(&input));
}
