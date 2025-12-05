use adventofcode_2025::read_input_from_file;
use std::collections::HashSet;
use std::str::Lines;

#[cfg(test)]
mod tests {
    use crate::{find_all_fresh_ingredients, find_number_of_fresh_ingredients, join_ranges};
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
        assert_eq!(find_all_fresh_ingredients(&input), 14);
    }

    #[test]
    fn test_join_ranges() {
        assert_eq!(join_ranges((3, 12), (10, 14)), (3, 14).into());
        assert_eq!(join_ranges((10, 14), (3, 12)), (3, 14).into());
        assert_eq!(join_ranges((10, 14), (3, 5)), None);
        assert_eq!(join_ranges((4, 10), (3, 12)), (3, 12).into());
    }
}

fn find_all_fresh_ingredients(input: &str) -> usize {
    let mut ranges_set: HashSet<Range> = parse_ranges(&mut input.lines()).into_iter().collect();
    find_all_fresh_ingredients_rec(&mut ranges_set)
}

type Range = (usize, usize);

fn find_all_fresh_ingredients_rec(ranges: &mut HashSet<Range>) -> usize {
    let original_ranges = ranges.clone();
    for range1 in original_ranges.iter() {
        for range2 in original_ranges.iter() {
            if range1 == range2 {
                continue;
            }
            if let Some(merged_range) = join_ranges(*range1, *range2) {
                ranges.remove(range1);
                ranges.remove(range2);
                ranges.insert(merged_range);
                return find_all_fresh_ingredients_rec(ranges);
            }
        }
    }

    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

fn join_ranges(range_1: Range, range_2: Range) -> Option<Range> {
    if range_1.0 > range_2.0 {
        return join_ranges(range_2, range_1);
    }

    if range_2.0 > range_1.1 {
        return None;
    }

    Some((range_1.0, usize::max(range_1.1, range_2.1)))
}

fn find_number_of_fresh_ingredients(input: &str) -> i32 {
    let mut lines = input.lines();

    let ranges = parse_ranges(&mut lines);

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

fn parse_ranges(lines: &mut Lines) -> Vec<Range> {
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
    ranges
}

fn main() {
    let input = read_input_from_file();

    println!(
        "part 1: {}, part 2: {}",
        find_number_of_fresh_ingredients(&input),
        find_all_fresh_ingredients(&input)
    );
}
