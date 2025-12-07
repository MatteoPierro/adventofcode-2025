use adventofcode_2025::read_input_from_file;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use crate::find_number_of_splits;
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

        assert_eq!(find_number_of_splits(input), 21);
    }
}

fn find_number_of_splits(input: &str) -> usize {
    let minifold = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let start_y = 2;
    let start_x = minifold[2].iter().position(|&c| c == '^').unwrap();

    let start_position = (start_x, start_y);
    let mut positions = HashSet::new();

    visit_minifold(start_position, &minifold, &mut positions);

    let result = positions.len();
    result
}

fn visit_minifold(
    current_position: Position,
    minifold: &Vec<Vec<char>>,
    visited: &mut HashSet<Position>,
) {
    let (x, y) = current_position;
    if y >= minifold.len() {
        return;
    }
    if visited.contains(&current_position) {
        return;
    }

    visited.insert(current_position);

    for next_y in (y + 2..minifold.len()).step_by(2) {
        if minifold[next_y][x - 1] == '^' {
            visit_minifold((x - 1, next_y), minifold, visited);
            break;
        }
    }

    for next_y in (y + 2..minifold.len()).step_by(2) {
        if x + 1 < minifold[next_y].len() && minifold[next_y][x + 1] == '^' {
            visit_minifold((x + 1, next_y), minifold, visited);
            break;
        }
    }
}

type Position = (usize, usize);

fn main() {
    let input = read_input_from_file();

    println!("first part: {}", find_number_of_splits(&input));
}
