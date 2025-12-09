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

    fn total_polygon_area(points: &[(isize, isize)]) -> f64 {
        let mut total_area: f64 = 0.0;
        for pairs in points.windows(2) {
            let pair1 = pairs[0];
            let pair2 = pairs[1];
            total_area += 0.5 * (pair1.1 + pair2.1) as f64 * (pair1.0 - pair2.0) as f64;
        }

        let last = points.last().unwrap();
        total_area += 0.5 * (last.1 + points[0].1) as f64 * (last.0 - points[0].0) as f64;
        total_area
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

    let mut max_area = -1;

    for i in 0..points.len() {
        let v1 = points[i];
        let v2 = points[(i + 1) % points.len()];
        let v3 = points[(i + 2) % points.len()];
        let v4 = vertexes(&vec![v1, v3])
            .into_iter()
            .find(|v| *v != v1 && *v != v2 && *v != v3)
            .unwrap();
        if !is_point_in_poly(v4, &points) {
            continue;
        }

        max_area = area(&vec![v1, v3]).max(max_area);
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
