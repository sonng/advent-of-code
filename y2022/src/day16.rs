use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    fs, path,
};

use anyhow::Result;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("/Users/son/workspace/rust-aoc/y2022/inputs/day16.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let valves: Vec<Valve> = input.split('\n').map(|l| Valve::from(l)).collect();
    let mut mapping: HashMap<&str, Valve> = HashMap::new();
    let mut connected_mapping: HashMap<&str, Vec<String>> = HashMap::new();
    let mut memorized_mapping: HashMap<String, Vec<String>> = HashMap::new();

    valves.iter().for_each(|v| {
        mapping.insert(&v.name, v.clone());
        connected_mapping.insert(&v.name, v.connected.clone());
    });

    const MAX_TIME: usize = 30;
    let mut cur = String::from("AA");
    let mut opened: HashSet<String> = HashSet::new();
    let mut time = 0;
    let mut timing: HashMap<usize, Valve> = HashMap::new();

    let to_open: HashSet<&str> = valves
        .iter()
        .filter(|v| v.flow != 0)
        .map(|v| v.name.as_str())
        .collect();

    let mut check_queue: Vec<PathState> = vec![PathState {
        path: vec![],
        visited: HashSet::new(),
        to_open,
        time: 0,
    }];

    let mut max_score = 0;

    while let Some(state) = check_queue.pop() {
        if state.to_open.is_empty() || state.time >= MAX_TIME {
            max_score = max(
                max_score,
                calculate_complete_released_flow(&state.path, &mapping, MAX_TIME),
            );
            continue;
        }

        let temp = state.clone();
        for valve_to_open in state.to_open {
            let mut new_state = temp.clone();

            let cur_position = new_state.path.last().unwrap_or(&(0, "AA"));
            let shortest_path = find_shortest_path(
                cur_position.1,
                valve_to_open,
                &connected_mapping,
                &mut memorized_mapping,
            );

            // Travel there
            new_state.time += shortest_path.len() - 1;
            new_state.visited.insert(valve_to_open);
            // Open it
            new_state.time += 1;
            new_state.to_open.remove(valve_to_open);
            new_state.path.push((new_state.time, valve_to_open));

            check_queue.push(new_state);
        }
    }

    println!("Day 16-1: {:?}", max_score);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    println!("Day 16-2: {}", "");
    Ok(())
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow: usize,
    connected: Vec<String>,
}

impl Valve {
    fn new(name: &str, flow: usize, connected: Vec<&str>) -> Self {
        Valve {
            name: name.into(),
            flow,
            connected: connected.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl From<&str> for Valve {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split("; ").collect();
        let valve = split[0].split('=').collect::<Vec<&str>>();

        let name: String = valve[0].replace("Valve ", "").replace(" has flow rate", "");
        let flow: usize = valve[1].parse().unwrap_or(0);
        let binding = split[1]
            .replace("tunnels leads to valves ", "")
            .replace("tunnels leads to valve ", "")
            .replace("tunnels lead to valves ", "")
            .replace("tunnels lead to valve ", "")
            .replace("tunnel leads to valve ", "")
            .replace("tunnel lead to valves ", "")
            .replace("tunnel leads to valves ", "");
        let connected: Vec<&str> = binding.split(", ").collect();

        Valve::new(&name, flow, connected)
    }
}

fn calculate_complete_released_flow(
    timings: &Vec<(usize, &str)>,
    mapping: &HashMap<&str, Valve>,
    max_time: usize,
) -> usize {
    timings
        .iter()
        .map(|timing| calculate_released_flow((timing.0, timing.1), mapping, max_time))
        .sum::<usize>()
}

fn calculate_released_flow(
    open_timing: (usize, &str),
    mapping: &HashMap<&str, Valve>,
    max_time: usize,
) -> usize {
    if open_timing.0 < max_time {
        let flow = mapping.get(open_timing.1).unwrap().flow;
        let total_time = max_time - open_timing.0;

        flow * total_time
    } else {
        0
    }
}

fn calculate_score_string(
    path: &Vec<String>,
    mapping: &HashMap<&str, Valve>,
    time_left: usize,
) -> usize {
    calculate_score(
        &path.iter().map(|s| s.as_str()).collect(),
        mapping,
        time_left,
    )
}

fn calculate_score(path: &Vec<&str>, mapping: &HashMap<&str, Valve>, time_left: usize) -> usize {
    let valve = mapping.get(*path.last().unwrap()).unwrap();
    let steps = path.len() - 1;
    let score = valve.flow;

    (score * (time_left - steps - 1)) / steps
}

#[derive(Clone)]
struct PathState<'a> {
    path: Vec<(usize, &'a str)>,
    visited: HashSet<&'a str>,
    to_open: HashSet<&'a str>,
    time: usize,
}

#[derive(Debug, Clone)]
struct State {
    path: Vec<String>,
    visited: HashSet<String>,
}

fn find_shortest_path<'a>(
    start: &'a str,
    end: &'a str,
    mapping: &'a HashMap<&'a str, Vec<String>>,
    memorized_mapping: &'a mut HashMap<String, Vec<String>>,
) -> Vec<String> {
    let key = format!("{}-{}", start, end);
    if memorized_mapping.contains_key(&key) {
        return memorized_mapping.get(&key).unwrap().to_vec();
    }

    let mut paths = VecDeque::new();

    paths.push_back(State {
        path: vec![start.to_string()],
        visited: HashSet::new(),
    });

    while !paths.is_empty() {
        let num_to_check = paths.len();

        for _ in 0..num_to_check {
            let mut state = paths.pop_front().unwrap();
            let cur = state.path.last().unwrap();

            if state.visited.contains(&cur.to_string()) {
                continue;
            }

            if *cur == end {
                memorized_mapping.insert(key, state.path.clone());
                return state.path.clone();
            }

            state.visited.insert(cur.to_string());

            let other_paths = mapping.get(cur.as_str()).unwrap();

            for other_path in other_paths {
                if !state.visited.contains(other_path) {
                    let mut state_clone = state.clone();
                    state_clone.path.push((&other_path).to_string());
                    paths.push_back(state_clone);
                }
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::day16::calculate_score;

    use super::{calculate_complete_released_flow, find_shortest_path, Valve};

    #[test]
    fn test_calculate_complete_timings() {
        let input = fs::read_to_string("./inputs/day16_example.txt").unwrap();
        let valves: Vec<Valve> = input.split('\n').map(|l| Valve::from(l)).collect();
        let mut mapping: HashMap<&str, Valve> = HashMap::new();
        let mut connected_mapping: HashMap<&str, Vec<String>> = HashMap::new();

        valves.iter().for_each(|v| {
            mapping.insert(&v.name, v.clone());
            connected_mapping.insert(&v.name, v.connected.clone());
        });

        let timings = vec![
            (2, "DD"),
            (6, "JJ"),
            (10, "BB"),
            (17, "HH"),
            (21, "EE"),
            (24, "CC"),
        ];
        let score = calculate_complete_released_flow(&timings, &mapping, 30);

        assert_eq!(score, 1649);

        let timings = vec![
            (2, "DD"),
            (5, "BB"),
            (9, "JJ"),
            (17, "HH"),
            (21, "EE"),
            (24, "CC"),
        ];
        let score = calculate_complete_released_flow(&timings, &mapping, 30);

        assert_eq!(score, 1651);
    }

    #[test]
    fn test_calculate_score() {
        let input = fs::read_to_string("./inputs/day16_example.txt").unwrap();
        let valves: Vec<Valve> = input.split('\n').map(|l| Valve::from(l)).collect();
        let mut mapping: HashMap<&str, Valve> = HashMap::new();
        let mut connected_mapping: HashMap<&str, Vec<String>> = HashMap::new();

        valves.iter().for_each(|v| {
            mapping.insert(&v.name, v.clone());
            connected_mapping.insert(&v.name, v.connected.clone());
        });

        let d_to_j = vec!["DD", "AA", "II", "JJ"];
        let d_to_b = vec!["DD", "AA", "BB"];

        let dj_score = calculate_score(&d_to_j, &mapping, 28);
        let db_score = calculate_score(&d_to_b, &mapping, 28);

        assert!(dj_score < db_score);
    }

    #[test]
    fn test_shortest_path() {
        let other_mapping: HashMap<&str, Vec<String>> = HashMap::from([
            (
                "AA",
                vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
            ),
            ("BB", vec!["CC".to_string(), "AA".to_string()]),
            ("CC", vec!["DD".to_string(), "BB".to_string()]),
            (
                "DD",
                vec!["CC".to_string(), "AA".to_string(), "EE".to_string()],
            ),
            ("EE", vec!["FF".to_string(), "DD".to_string()]),
            ("FF", vec!["EE".to_string(), "GG".to_string()]),
            ("GG", vec!["FF".to_string(), "HH".to_string()]),
            ("HH", vec!["GG".to_string()]),
            ("II", vec!["AA".to_string(), "JJ".to_string()]),
            ("JJ", vec!["II".to_string()]),
        ]);

        let mut memory = HashMap::new();
        let path = find_shortest_path(&"AA", &"JJ", &other_mapping, &mut memory);

        assert_eq!(path.len(), 3);
    }
}
