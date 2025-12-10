use adventofcode_2025::read_input_from_file;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use crate::{Machine, find_fewest_presses, find_fewest_presses_machine_for_joltage_level};
    use indoc::indoc;

    #[test]
    fn it_works() {
        let input = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"};

        assert_eq!(find_fewest_presses(input), 7);

        let machines: Vec<Machine> = input.lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(
            find_fewest_presses_machine_for_joltage_level(&machines[0]),
            10
        );
        assert_eq!(
            find_fewest_presses_machine_for_joltage_level(&machines[1]),
            12
        );
        assert_eq!(
            find_fewest_presses_machine_for_joltage_level(&machines[2]),
            11
        );
    }
}

fn find_fewest_presses_for_joltage_level(input: &str) -> usize {
    let machines: Vec<Machine> = input.lines().map(|l| l.parse().unwrap()).collect();
    let fewest_presses_for_machine: Vec<usize> = machines
        .iter()
        .map(|machine| find_fewest_presses_machine_for_joltage_level(machine))
        .collect();
    fewest_presses_for_machine.iter().sum::<usize>()
}

fn find_fewest_presses_machine_for_joltage_level(machine: &Machine) -> usize {
    let target = &machine.joltage_levels;
    let initial_state = target.iter().map(|_| 0).collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    heap.push(JoltageState {
        buttons_pressed: 0,
        state: initial_state.clone(),
    });
    let mut visited: HashSet<Vec<usize>> = HashSet::new();
    let mut result = usize::MAX;
    while let Some(JoltageState {
        buttons_pressed,
        state,
    }) = heap.pop()
    {
        if visited.iter().contains(&state) {
            continue;
        }
        if &state == target {
            result = buttons_pressed;
            break;
        }

        if state
            .iter()
            .enumerate()
            .any(|(i, &level)| level > target[i])
        {
            continue;
        }
        for button in machine.buttons.iter() {
            let mut new_state = state.clone();
            for &i in button {
                new_state[i] += 1;
            }
            heap.push(JoltageState {
                buttons_pressed: buttons_pressed + 1,
                state: new_state,
            });
        }
        visited.insert(state.clone());
    }
    result
}

fn find_fewest_presses(input: &str) -> usize {
    let machines: Vec<Machine> = input.lines().map(|l| l.parse().unwrap()).collect();
    let fewest_presses_for_machine: Vec<usize> = machines
        .iter()
        .map(|machine| find_fewest_presses_for_a_machine(machine))
        .collect();
    fewest_presses_for_machine.iter().sum::<usize>()
}

fn find_fewest_presses_for_a_machine(machine: &Machine) -> usize {
    let target = &machine.lights_diagram;
    let initial_state = target.iter().map(|_| LightState::Off).collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    heap.push(State {
        buttons_pressed: 0,
        state: initial_state.clone(),
    });
    let mut visited: HashSet<Vec<LightState>> = HashSet::new();
    let mut result = usize::MAX;
    while let Some(State {
        buttons_pressed,
        state,
    }) = heap.pop()
    {
        if visited.iter().contains(&state) {
            continue;
        }
        if &state == target {
            result = buttons_pressed;
            break;
        }
        for button in machine.buttons.iter() {
            let mut new_state = state.clone();
            for &i in button {
                new_state[i] = state[i].flip();
            }
            heap.push(State {
                buttons_pressed: buttons_pressed + 1,
                state: new_state,
            });
        }
        visited.insert(state.clone());
    }
    result
}

#[derive(Clone, Eq, PartialEq)]
struct JoltageState {
    buttons_pressed: usize,
    state: Vec<usize>,
}

impl Ord for JoltageState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.buttons_pressed.cmp(&self.buttons_pressed)
    }
}

impl PartialOrd for JoltageState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    buttons_pressed: usize,
    state: Vec<LightState>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.buttons_pressed.cmp(&self.buttons_pressed)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for LightState {
    fn to_string(&self) -> String {
        match self {
            LightState::On => "#",
            LightState::Off => ".",
        }
        .to_string()
    }
}

impl LightState {
    fn flip(&self) -> Self {
        match self {
            LightState::On => LightState::Off,
            LightState::Off => LightState::On,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum LightState {
    On,
    Off,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = line.split(' ').collect();
        let lights_diagram: Vec<LightState> = parts[0].chars().collect::<Vec<_>>()
            [1..parts[0].len() - 1]
            .iter()
            .map(|c| match c {
                '.' => LightState::Off,
                '#' => LightState::On,
                _ => panic!("invalid character in light diagram"),
            })
            .collect();

        let buttons: Vec<_> = parts[1..parts.len() - 1]
            .into_iter()
            .map(|&b| {
                let b = b.replace("(", "").replace(")", "");
                b.split(",")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();
        let joltage_levels = parts
            .last()
            .unwrap()
            .replace("{", "")
            .replace("}", "")
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            lights_diagram,
            buttons,
            joltage_levels,
        })
    }
}

#[derive(Debug)]
struct Machine {
    lights_diagram: Vec<LightState>,
    buttons: Vec<Vec<usize>>,
    joltage_levels: Vec<usize>,
}

fn main() {
    let input = read_input_from_file();

    println!(
        "part 1:  {}, part 2: {}",
        find_fewest_presses(&input),
        find_fewest_presses_for_joltage_level(&input)
    );
}
