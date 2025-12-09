use adventofcode_2025::read_input_from_file;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::isize;

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
        assert_eq!(24, calculate_max_area_within_polygon(input));
    }
}

type Point = (isize, isize);

fn calculate_max_area_within_polygon(input: &str) -> isize {
    let points = input
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            let x: isize = parts.next().unwrap().parse().unwrap();
            let y: isize = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let combinations = points.iter().combinations(2).collect::<Vec<_>>();

    let mut max_area = -1;

    for combination in combinations {
        let v1 = *combination[0];
        let v2 = *combination[1];
        let other_vertexes: Vec<_> = vertexes(&vec![v1, v2])
            .into_iter()
            .filter(|v| *v != v1 && *v != v2)
            .collect();

        if other_vertexes.iter().any(|v| !is_point_in_poly(*v, &points)) {
            continue;
        }

        max_area = area(&vec![v1, v2]).max(max_area);
    }
    
    max_area
}

fn is_point_in_poly(p: Point, poly: &[Point]) -> bool {
    let n = poly.len();

    // Edge check
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        if point_on_segment(p, a, b) {
            return true;
        }
    }

    // Ray casting
    let mut inside = false;
    let mut p1 = poly[0];

    for i in 0..=n {
        let p2 = poly[i % n];

        if p.1 > p1.1.min(p2.1) && p.1 <= p1.1.max(p2.1) && p.0 <= p1.0.max(p2.0) {
            let x_inters = (p.1 - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1) + p1.0;

            if p.1 <= x_inters {
                inside = !inside;
            }
        }

        p1 = p2;
    }

    inside
}

fn point_on_segment(p: Point, a: Point, b: Point) -> bool {
    // Check collinearity using cross product
    let cross = (p.0 - a.0) * (b.1 - a.1) - (p.1 - a.1) * (b.0 - a.0);
    if cross.abs() > 0 {
        return false;
    }

    // Check if within bounding box (via dot product)
    let dot = (p.0 - a.0) * (p.0 - b.0) + (p.1 - a.1) * (p.1 - b.1);
    if dot > 0 {
        return false;
    }

    true
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
        .max()
        .unwrap()
}

fn vertexes(pair: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let x1 = pair[0].0;
    let y1 = pair[0].1;
    let x2 = pair[1].0;
    let y2 = pair[1].1;
    vec![pair[0], pair[1], (x1, y2), (x2, y1)]
}

fn area(pair: &Vec<(isize, isize)>) -> isize {
    ((pair[0].0 - pair[1].0).abs() + 1) * ((pair[0].1 - pair[1].1).abs() + 1)
}

fn main() {
    let input = read_input_from_file();

    println!(
        "Part 1: {}, Part 2: {}",
        find_largest_area(&input),
        calculate_max_area_within_polygon(&input)
    );
}
