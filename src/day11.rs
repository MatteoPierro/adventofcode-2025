use adventofcode_2025::read_input_from_file;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::{find_number_of_path_visiting_both_fft_and_dac, find_number_of_paths_from_you};
    use indoc::indoc;

    #[test]
    fn it_finds_number_of_paths_from_you() {
        let input = indoc!(
            "
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out"
        );

        assert_eq!(find_number_of_paths_from_you(input), 5);
    }

    #[test]
    fn it_finds_number_of_path_visiting_both_fft_and_dac() {
        let input = indoc!(
            "
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out"
        );

        assert_eq!(find_number_of_path_visiting_both_fft_and_dac(input), 2)
    }
}

fn find_number_of_path_visiting_both_fft_and_dac(input: &str) -> usize {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    input
        .lines()
        .flat_map(|line| {
            let connections = line
                .replace(":", "")
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let start = connections[0].clone();
            connections[1..]
                .iter()
                .map(|d| (start.clone(), d.clone()))
                .collect::<Vec<(String, String)>>()
        })
        .for_each(|(s, d)| {
            connections.entry(s).or_insert(vec![]).push(d);
        });

    dfs("svr", false, false, &connections, &mut HashMap::new())
}

type MemoEntry = (String, bool, bool);

fn dfs(
    from: &str,
    mut found_fft: bool,
    mut found_dac: bool,
    connections: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<MemoEntry, usize>,
) -> usize {
    found_dac = found_dac || from == "dac";
    found_fft = found_fft || from == "fft";

    if found_fft && found_dac && from == "out" {
        return 1;
    }

    if from == "out" {
        return 0;
    }

    let memo_entry = (from.to_string(), found_fft, found_dac);
    if memo.contains_key(&memo_entry) {
        return memo[&memo_entry];
    }

    let mut result = 0;

    if let Some(neighbors) = connections.get(from) {
        for neighbor in neighbors {
            result += dfs(neighbor, found_fft, found_dac, &connections, memo);
        }
    }

    memo.insert(memo_entry, result);
    result
}

fn find_number_of_paths_from_you(input: &str) -> usize {
    find_paths_from(input, "you".to_string()).len()
}

fn find_paths_from(input: &str, starting_node: String) -> Vec<Vec<String>> {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    input
        .lines()
        .flat_map(|line| {
            let connections = line
                .replace(":", "")
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let start = connections[0].clone();
            connections[1..]
                .iter()
                .map(|d| (start.clone(), d.clone()))
                .collect::<Vec<(String, String)>>()
        })
        .for_each(|(s, d)| {
            connections.entry(s).or_insert(vec![]).push(d);
        });

    let mut paths = vec![];
    let mut queue: Vec<Vec<String>> = vec![vec![starting_node]];
    while let Some(current_path) = queue.pop() {
        let node = current_path.last().unwrap();
        if node == "out" {
            paths.push(current_path.clone());
            continue;
        }
        if let Some(neighbors) = connections.get(node) {
            neighbors.iter().for_each(|n| {
                let mut new_path = current_path.clone();
                new_path.push(n.clone());
                queue.push(new_path);
            });
        }
    }
    paths
}

fn main() {
    let input = read_input_from_file();

    println!(
        "Part 1: {}, Part 2: {}",
        find_number_of_paths_from_you(&input),
        find_number_of_path_visiting_both_fft_and_dac(&input)
    );
}
