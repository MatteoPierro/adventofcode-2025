use adventofcode_2025::read_input_from_file;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::find_all_paths;
    use indoc::indoc;

    #[test]
    fn it_works() {
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

        assert_eq!(find_all_paths(input), 5);
    }
}

fn find_all_paths(input: &str) -> i32 {
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

    let mut paths = 0;
    let mut queue: Vec<String> = vec!["you".to_string()];
    while let Some(node) = queue.pop() {
        if node == "out" {
            paths += 1;
            continue;
        }
        if let Some(neighbors) = connections.get(&node) {
            neighbors.iter().for_each(|n| {
                queue.push(n.clone());
            });
        }
    }
    paths
}

fn main() {
    let input = read_input_from_file();

    println!("Part 1: {}", find_all_paths(&input));
}
