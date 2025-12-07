use adventofcode_2025::read_input_from_file;
use itertools::Itertools;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::visit_minifold;
    use indoc::indoc;

    #[test]
    fn it_finds_number_of_splits() {
        let input = indoc! {
            ".......S.......
             ...............
             .......^.......
             ...............
             ......^.^......
             ...............
             .....^.^.^.....
             ...............
             ....^.^...^....
             ...............
             ...^.^...^.^...
             ...............
             ..^...^.....^..
             ...............
             .^.^.^.^.^...^.
             ..............."
        };

        let (number_of_splits, number_of_different_timelines) = visit_minifold(input);
        assert_eq!(number_of_splits, 21);
        assert_eq!(number_of_different_timelines, 40);
    }
}

fn visit_minifold(input: &str) -> (usize, usize) {
    let minifold = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let start_y = 2;
    let start_x = minifold[2].iter().position(|&c| c == '^').unwrap();

    let start_position = (start_x, start_y);
    let mut positions = HashMap::new();

    let number_of_different_timelines =
        visit_minifold_rec(start_position, &minifold, &mut positions);

    let number_of_splits = positions.len();
    (number_of_splits, number_of_different_timelines)
}

fn visit_minifold_rec(
    current_position: Position,
    minifold: &Vec<Vec<char>>,
    visited: &mut HashMap<Position, usize>,
) -> usize {
    let (x, y) = current_position;
    if y >= minifold.len() {
        return 1;
    }
    if visited.keys().contains(&current_position) {
        return visited[&current_position];
    }

    let mut left_result = 1;
    for next_y in (y + 2..minifold.len()).step_by(2) {
        if minifold[next_y][x - 1] == '^' {
            left_result = visit_minifold_rec((x - 1, next_y), minifold, visited);
            break;
        }
    }

    let mut right_result = 1;
    for next_y in (y + 2..minifold.len()).step_by(2) {
        if x + 1 < minifold[next_y].len() && minifold[next_y][x + 1] == '^' {
            right_result = visit_minifold_rec((x + 1, next_y), minifold, visited);
            break;
        }
    }

    visited.insert(current_position, left_result + right_result);
    left_result + right_result
}

type Position = (usize, usize);

fn main() {
    let input = read_input_from_file();

    let (number_of_splits, number_of_different_timelines) = visit_minifold(&input);
    println!(
        "first part: {} second part: {}",
        number_of_splits, number_of_different_timelines
    );
}
