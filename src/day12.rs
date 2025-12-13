use adventofcode_2025::read_input_from_file;

fn find_number_fit_all_regions(input: &str) -> usize {
    let raw_blocks = input.lines().collect::<Vec<_>>();
    raw_blocks
        .split(|line| line.is_empty())
        .last()
        .unwrap()
        .iter()
        .filter(|gift_area| is_fitting(gift_area))
        .count()
}

fn is_fitting(raw_gift_region: &str) -> bool {
    let gift_region = raw_gift_region.split(" ").collect::<Vec<_>>();
    let available_area = gift_region[0]
        .replace(":", "")
        .split("x")
        .map(|s| s.parse::<usize>().unwrap())
        .reduce(|a, b| a * b)
        .unwrap();
    let area = gift_region[1..]
        .iter()
        .map(|s| s.parse::<usize>().unwrap() * 9)
        .sum::<usize>();
    area <= available_area
}

fn main() {
    let input = read_input_from_file();

    println!("part 1: {}", find_number_fit_all_regions(&input));
}
