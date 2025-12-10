use adventofcode_2025::read_input_from_file;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use crate::find_fewest_presses;
    use indoc::indoc;

    #[test]
    fn it_works() {
        let input = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"};

        assert_eq!(find_fewest_presses(input), 7);
    }
}

fn find_fewest_presses(input: &str) -> usize {
    let machines: Vec<Machine> = input.lines().map(|l| l.parse().unwrap()).collect();
    let fewest_presses_for_machine: Vec<usize> = machines
        .iter()
        .map(|machine| find_fewest_presses_for_a_machine(machine))
        .collect();
    let fewest_presses = fewest_presses_for_machine.iter().sum::<usize>();
    fewest_presses
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
        let parts: Vec<_> = line
            .split(' ')
            .map(|p| p.chars().collect::<Vec<_>>())
            .collect();
        let lights_diagram: Vec<LightState> = parts[0][1..parts[0].len() - 1]
            .iter()
            .map(|c| match c {
                '.' => LightState::Off,
                '#' => LightState::On,
                _ => panic!("invalid character in light diagram"),
            })
            .collect();

        let buttons: Vec<_> = parts[1..parts.len() - 1]
            .into_iter()
            .map(|b| {
                b[1..b.len() - 1]
                    .iter()
                    .filter(|&d| d != &',')
                    .map(|d| d.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect();
        Ok(Self {
            lights_diagram,
            buttons,
        })
    }
}

#[derive(Debug)]
struct Machine {
    lights_diagram: Vec<LightState>,
    buttons: Vec<Vec<usize>>,
}

fn main() {
    let input = read_input_from_file();

    println!("The answer to part 1 is {}", find_fewest_presses(&input));
}
