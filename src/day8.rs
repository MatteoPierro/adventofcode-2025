use adventofcode_2025::read_input_from_file;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashSet;

#[cfg(test)]
mod test {
    use crate::find_multiply_first_three_larger_circuits;
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

        assert_eq!(find_multiply_first_three_larger_circuits(input, 10), 40)
    }
}

fn find_multiply_first_three_larger_circuits(input: &str, connections: i32) -> usize {
    let ordered_pairs: Vec<_> = input
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .combinations(2)
        .sorted_by(sorter)
        .collect();

    // ordered_pairs.iter().for_each(|p| {
    //     let distance = distance(&p[0], &p[1]);
    //     println!("{:?} distance {distance}", p)
    // });

    let mut circuits: Vec<RefCell<HashSet<Position>>> = vec![];
    let mut ordered_pair_iter = ordered_pairs.iter();
    let mut connections_made = 0;
    while connections_made < connections {
        let pair = ordered_pair_iter.next().unwrap();
        let first = pair[0];
        let second = pair[1];
        let circuits_to_connect: Vec<_> = circuits
            .iter()
            .filter(|circuit| {
                circuit.borrow().contains(&first) || circuit.borrow().contains(&second)
            })
            .by_ref()
            .collect();
        let len = circuits_to_connect.len();
        match len {
            0 => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(first);
                new_circuit.insert(second);
                circuits.push(RefCell::new(new_circuit));
            }
            1 => {
                let circuit = &circuits_to_connect[0];
                circuit.borrow_mut().insert(first);
                circuit.borrow_mut().insert(second);
            }
            2 => {
                println!("connecting two circuits");
                let circuit1 = circuits_to_connect[0].clone();
                println!("circuit1: {:?}", circuit1.borrow());
                let circuit2 = circuits_to_connect[1].clone();
                println!("circuit2: {:?}", circuit2.borrow());
                let circuit1_position = circuits
                    .iter()
                    .position(|c| c.borrow().eq(&circuits_to_connect[0].borrow()))
                    .unwrap();
                let circuit2_position = circuits
                    .iter()
                    .position(|c| c.borrow().eq(&circuits_to_connect[1].borrow()))
                    .unwrap();
                if circuit1_position < circuit2_position {
                    circuits.remove(circuit1_position);
                    circuits.remove(circuit2_position - 1);
                } else {
                    circuits.remove(circuit2_position);
                    circuits.remove(circuit1_position - 1);
                }

                println!("circuits after removes");
                circuits.iter().for_each(|circuit| println!("{:?}", circuit.borrow()));
                circuit1.borrow_mut().extend(circuit2.borrow().iter().cloned());
                circuit1.borrow_mut().insert(first);
                circuit1.borrow_mut().insert(second);
                circuits.push(circuit1.clone());
            }
            _ => panic!("no more than 2")
        }

        connections_made += 1;
        println!("--------------");
        println!("connection {connections_made} number of circuits: {}", circuits.len());
        println!("path between {:?} and {:?}", first, second);
        circuits.iter().for_each(|circuit| println!("{:?}", circuit.borrow()));
        println!("--------------");
    }

    circuits
        .sort_by_key(|c| { c.borrow().len() as isize * -1 });
    let result = circuits[0].borrow().len() * circuits[1].borrow().len() * circuits[2].borrow().len();
    result
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
    println!("Part 1: {}", find_multiply_first_three_larger_circuits(&input, 1000));
}
