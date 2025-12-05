use std::{env, fs};

#[cfg(test)]
mod tests {
    use crate::find_number_of_fresh_ingredients;
    use indoc::indoc;

    #[test]
    fn test_find_number_of_fresh_ingredients() {
        let input = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
        "};

        assert_eq!(find_number_of_fresh_ingredients(input), 3);
    }
}

fn find_number_of_fresh_ingredients(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut ranges = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let range = line
            .split('-')
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        ranges.push((range[0], range[1]))
    }

    let mut ingredients_id = vec![];
    while let Some(line) = lines.next() {
        ingredients_id.push(line.parse::<usize>().unwrap());
    }

    let mut result = 0;

    for id in ingredients_id {
        for range in &ranges {
            if id >= range.0 && id <= range.1 {
                result += 1;
                break;
            }
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("Please provide input as first argument"),
    )
    .expect("input");

    println!("{}", find_number_of_fresh_ingredients(&input));
}
