use std::{env, fs};

#[cfg(test)]
mod test {
    use crate::find_number_of_forklifts_accessible;

    #[test]
    fn it_works() {
        let input = indoc::indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@."};

        assert_eq!(find_number_of_forklifts_accessible(input), 13)
    }
}

fn find_number_of_forklifts_accessible(input: &str) -> i32 {
    let mut result = 0;

    let diagram = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = diagram.len();
    let width = diagram[0].len();

    for (r, row) in diagram.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col != &'@' {
                continue;
            }

            let mut papers = 0;
            for (y, x) in neightbours_positions(r, c, height, width) {
                if diagram[y][x] == '@' {
                    papers += 1;
                }
            }

            if papers < 4 {
                result += 1;
            }
        }
    }
    result
}

fn neightbours_positions(
    r: usize,
    c: usize,
    height: usize,
    width: usize,
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
        .iter()
        .for_each(|(dr, dc)| {
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;
            if new_r >= 0 && new_r < height as isize && new_c >= 0 && new_c < width as isize {
                result.push((new_r as usize, new_c as usize));
            }
        });
    result
}

fn main() {
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("Please provide input as first argument"),
    )
    .expect("input");

    println!("result {}", find_number_of_forklifts_accessible(&input));
}
