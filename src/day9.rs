use adventofcode_2025::read_input_from_file;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
                        7,1
                        11,1
                        11,7
                        9,7
                        9,5
                        2,5
                        2,3
                        7,3"};

        assert_eq!(50, find_largest_area(input));
    }
}

fn find_largest_area(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            let x: isize = parts.next().unwrap().parse().unwrap();
            let y: isize = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .combinations(2)
        .map(|pair| area(&pair))
        // .max_by(|pair, pair2| {
        //     (area(pair))
        //         .cmp(&((pair2[0].0 - pair2[1].0).abs() * (pair2[0].1 - pair2[1].1).abs()))
        // })
        .max()
        .unwrap()
}

fn area(pair: &Vec<(isize, isize)>) -> isize {
    ((pair[0].0 - pair[1].0).abs() + 1) * ((pair[0].1 - pair[1].1).abs() + 1)
}

fn main() {
    let input = read_input_from_file();

    println!("Part 1: {}", find_largest_area(&input));
}
