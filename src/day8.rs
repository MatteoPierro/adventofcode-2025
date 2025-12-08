use adventofcode_2025::read_input_from_file;
use itertools::Itertools;
use std::collections::HashSet;

#[cfg(test)]
mod test {
    use crate::connect_junctions;
    use indoc::indoc;

    #[test]
    fn it_finds_multiply_first_three_larger_circuits() {
        let input = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689"};

        let (first_three_larger_circuits, largest_circuit) = connect_junctions(input, 10);
        assert_eq!(first_three_larger_circuits, 40);
        assert_eq!(largest_circuit, 25272)
    }
}

fn connect_junctions(input: &str, connections: i32) -> (usize, usize) {
    let junctions = input.lines().count();
    let ordered_pairs: Vec<_> = input
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .combinations(2)
        .sorted_by(sorter)
        .collect();

    let mut circuits: Vec<HashSet<Position>> = vec![];
    let mut ordered_pair_iter = ordered_pairs.iter();
    let mut last_used_pair = None;
    let mut used_connections = 0;
    let mut first_three_larger_circuits = 0;
    while !(circuits.len() == 1 && circuits[0].len() == junctions) {
        let pair = ordered_pair_iter.next().unwrap();
        last_used_pair = Some(pair);
        let first = pair[0];
        let second = pair[1];
        let circuits_to_connect_indexes: Vec<usize> = circuits
            .iter()
            .enumerate()
            .filter_map(|(index, circuit)| {
                if circuit.contains(&first) || circuit.contains(&second) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();
        match circuits_to_connect_indexes.len() {
            0 => {
                circuits.push(HashSet::from([first, second]));
            }
            1 => {
                circuits[circuits_to_connect_indexes[0]].insert(first);
                circuits[circuits_to_connect_indexes[0]].insert(second);
            }
            2 => {
                let extension = circuits.remove(circuits_to_connect_indexes[1]);
                circuits[circuits_to_connect_indexes[0]].extend(extension);
            }
            _ => panic!("no more than 2"),
        }

        used_connections += 1;
        if used_connections == connections {
            circuits.sort_by_key(|c| c.len() as isize * -1);
            first_three_larger_circuits = circuits[0].len() * circuits[1].len() * circuits[2].len();
        }
    }

    let last_pair = last_used_pair.unwrap();
    let largest_circuit = last_pair[0].0 * last_pair[1].0;
    (first_three_larger_circuits, largest_circuit)
}
fn sorter(a: &Vec<Position>, b: &Vec<Position>) -> std::cmp::Ordering {
    let dist_a = distance(a.first().unwrap(), a.last().unwrap());
    let dist_b = distance(b.first().unwrap(), b.last().unwrap());
    dist_a.partial_cmp(&dist_b).unwrap()
}

fn distance(a: &Position, b: &Position) -> f64 {
    f64::sqrt(
        ((a.0 as isize - b.0 as isize).pow(2)
            + (a.1 as isize - b.1 as isize).pow(2)
            + (a.2 as isize - b.2 as isize).pow(2)) as f64,
    )
}
type Position = (usize, usize, usize);

fn main() {
    let input = read_input_from_file();
    let (first_three_larger_circuits, largest_circuit) = connect_junctions(&input, 1000);
    println!(
        "Part 1: {}, Part 2: {}",
        first_three_larger_circuits, largest_circuit
    );
}
