use adventofcode_2025::read_input_from_file;
use std::ops::Div;

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn no_increment_if_does_not_stop_at_zero() {
        let input: &str = indoc! {"
        L68
        "};

        assert_eq!(find_entrance_password(input).0, 0);
    }

    #[test]
    fn it_increments_if_it_stops_at_zero() {
        let input: &str = indoc! {"
        L50
        "};

        assert_eq!(find_entrance_password(input).0, 1);
    }

    #[test]
    fn it_increments_twice_if_it_stops_at_zero_twice() {
        let input: &str = indoc! {"
        L50
        R50
        L50
        "};

        assert_eq!(find_entrance_password(input).0, 2);
    }

    #[test]
    fn it_solve_example() {
        let input = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
        "};
        assert_eq!(find_entrance_password(input), (3, 6));
    }
}

fn main() {
    let input = read_input_from_file();

    let result = find_entrance_password(&input);
    println!("first part: {}; second part {}", result.0, result.1);
}

fn find_entrance_password(input: &str) -> (i32, i32) {
    let mut position = 50;
    let values = input.split("\n");
    let mut first_part_result = 0;
    let mut second_part_result = 0;
    for b in values {
        if b.is_empty() {
            continue;
        }
        let direction = b.chars().nth(0).unwrap();
        let step = b
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        match direction {
            'L' => {
                if position == 0 {
                    second_part_result -= 1;
                }

                let walk = position - step;

                second_part_result += (walk as f32).div(100f32).floor().abs() as i32;

                position = walk.rem_euclid(100);

                if position == 0 {
                    second_part_result += 1;
                }
            }
            'R' => {
                let next_position = position + step;
                second_part_result += (next_position as f32).div(100f32).floor() as i32;

                position = (position + step).rem_euclid(100);
            }
            _ => todo!("unknown direction"),
        }

        if position == 0 {
            first_part_result += 1
        }
    }

    (first_part_result, second_part_result)
}
