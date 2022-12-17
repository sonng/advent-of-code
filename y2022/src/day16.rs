use std::{
    cmp::{max, Ordering},
    collections::{HashMap, HashSet, VecDeque},
    fs,
    hash::Hash,
    time::Instant,
};

use anyhow::Result;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day16.txt")?;
    let example_input = fs::read_to_string("./inputs/day16_example.txt")?;
    solve_part_1(&example_input)?;
    // solve_part_1(&input)?;
    let start = Instant::now();
    solve_part_2(&example_input)?;
    println!("Time Elapsed: {:?}", start.elapsed().as_micros());
    let start = Instant::now();
    solve_part_2(&input)?;
    println!("Time Elapsed: {:?}", start.elapsed().as_micros());
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let valves: Vec<Valve> = input.split('\n').map(|l| Valve::from(l)).collect();
    let mut mapping: HashMap<String, Valve> = HashMap::new();
    let mut connected_mapping: HashMap<&str, Vec<String>> = HashMap::new();
    let mut memorized_mapping: HashMap<String, Vec<String>> = HashMap::new();

    valves.iter().for_each(|v| {
        connected_mapping.insert(&v.name, v.connected.clone());
        mapping.insert(v.name.clone(), v.clone());
    });

    const MAX_TIME: usize = 30;
    let to_open: HashSet<&str> = valves
        .iter()
        .filter(|v| v.flow != 0)
        .map(|v| v.name.as_str())
        .collect();

    let mut check_queue: Vec<PathState> = vec![PathState {
        path: vec![],
        to_open,
        score_so_far: 0,
    }];

    let mut max_score = 0;

    let initial = ValveOpen(0, "AA".into());
    while let Some(state) = check_queue.pop() {
        let cur_position = state.path.last().unwrap_or(&initial);

        if state.to_open.is_empty() || cur_position.0 >= MAX_TIME {
            max_score = max(max_score, scoree(state.path.iter(), &mapping, MAX_TIME));
            continue;
        }

        let temp = state.clone();
        for valve_to_open in state.to_open {
            let mut new_state = temp.clone();

            let shortest_path = find_shortest_path(
                &cur_position.1,
                valve_to_open,
                &connected_mapping,
                &mut memorized_mapping,
            );

            new_state.to_open.remove(valve_to_open);
            new_state.path.push(ValveOpen(
                cur_position.0 + shortest_path.len(),
                valve_to_open.to_string(),
            ));

            check_queue.push(new_state);
        }
    }

    println!("Day 16-1: {:?}", max_score);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let valves: Vec<Valve> = input.split('\n').map(|l| Valve::from(l)).collect();
    let mut mapping: HashMap<String, Valve> = HashMap::new();
    let mut connected_mapping: HashMap<&str, Vec<String>> = HashMap::new();
    let mut memorized_mapping: HashMap<String, Vec<String>> = HashMap::new();

    valves.iter().for_each(|v| {
        mapping.insert(v.name.clone(), v.clone());
        connected_mapping.insert(&v.name, v.connected.clone());
    });

    const MAX_TIME: usize = 26;
    let to_open: HashSet<&str> = valves
        .iter()
        .filter(|v| v.flow != 0)
        .map(|v| v.name.as_str())
        .collect();

    let mut all_paths = generate_all_paths(
        to_open,
        MAX_TIME,
        &mapping,
        &connected_mapping,
        &mut memorized_mapping,
    );
    all_paths.sort();
    println!("Paths: {}", all_paths.len());
    let mut max_score = 0;
    for person in &all_paths {
        for elephant in &all_paths {
            if !person.0.is_disjoint(&elephant.0) {
                continue;
            }

            let score = person.1 + elephant.1;
            if score > max_score {
                println!("S: {}", score);
                max_score = score;
            }
        }
    }

    println!("Day 16-2: {:?}", max_score);
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

fn scoree<'a, I>(timings: I, mapping: &HashMap<String, Valve>, max_time: usize) -> usize
where
    I: Iterator<Item = &'a ValveOpen>,
{
    timings
        .map(|timing| calculate_released_flow(&timing, mapping, max_time))
        .sum::<usize>()
}

fn calculate_released_flow(
    open_timing: &ValveOpen,
    mapping: &HashMap<String, Valve>,
    max_time: usize,
) -> usize {
    if open_timing.0 < max_time {
        let flow = mapping.get(&open_timing.1).unwrap().flow;
        let total_time = max_time - open_timing.0;

        flow * total_time
    } else {
        0
    }
}

#[derive(Clone)]
struct PathState<'a> {
    path: Vec<ValveOpen>,
    to_open: HashSet<&'a str>,
    score_so_far: usize,
}

impl Ord for PathState<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score_so_far.cmp(&other.score_so_far)
    }
}

impl PartialOrd for PathState<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score_so_far.partial_cmp(&other.score_so_far)
    }
}

impl PartialEq<PathState<'_>> for PathState<'_> {
    fn eq(&self, other: &PathState<'_>) -> bool {
        self.score_so_far == other.score_so_far
    }
}

impl Eq for PathState<'_> {}

#[derive(Debug, Clone)]
struct State {
    path: Vec<String>,
    visited: HashSet<String>,
    time_so_far: usize,
}

#[derive(Debug, Clone)]
struct StateWithTakeTime<'a> {
    path: Vec<ValveOpen>,
    visited: HashSet<&'a str>,
    time_so_far: usize,
}

#[derive(Debug, Clone)]
struct PathWithScore(HashSet<ValveOpen>, usize);

impl Ord for PathWithScore {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for PathWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl PartialEq for PathWithScore {
    fn eq(&self, other: &Self) -> bool {
        other.1 == self.1
    }
}

impl Eq for PathWithScore {}

#[derive(Debug, Clone)]
struct ValveOpen(usize, String);

impl Hash for ValveOpen {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl PartialEq for ValveOpen {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for ValveOpen {}

fn generate_all_paths(
    options: HashSet<&str>,
    max_time: usize,
    mapping: &HashMap<String, Valve>,
    connection_mapping: &HashMap<&str, Vec<String>>,
    memorized_mapping: &mut HashMap<String, Vec<String>>,
) -> Vec<PathWithScore> {
    let mut all_paths: Vec<PathWithScore> = vec![];

    let initial = ValveOpen(0, "AA".to_string());

    let mut wip = VecDeque::new();
    wip.push_back(StateWithTakeTime {
        path: vec![],
        visited: HashSet::new(),
        time_so_far: 0,
    });

    while !wip.is_empty() {
        let num = wip.len();
        for _ in 0..num {
            let state = wip.pop_front().unwrap();

            if !state.path.is_empty() {
                let path = HashSet::from_iter(state.path.iter().map(|m| m.clone()));
                let score = scoree(path.iter(), mapping, max_time);
                all_paths.push(PathWithScore(path, score));
            }

            if state.time_so_far >= max_time {
                continue;
            }

            let cur = &state.path.last().unwrap_or(&initial).1;
            let difference = options.difference(&state.visited);

            for option in difference.into_iter() {
                let path = find_shortest_path(&cur, option, connection_mapping, memorized_mapping);
                let mut new_state = state.clone();
                new_state.time_so_far += path.len();
                new_state
                    .path
                    .push(ValveOpen(new_state.time_so_far, option.to_string()));
                new_state.visited.insert(option);

                wip.push_back(new_state);
            }
        }
    }

    all_paths
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
        time_so_far: 0,
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

    use super::{find_shortest_path, scoree, Valve};

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
        // let score = scoree(&timings, &mapping, 30);

        // assert_eq!(score, 1649);

        let timings = vec![
            (2, "DD"),
            (5, "BB"),
            (9, "JJ"),
            (17, "HH"),
            (21, "EE"),
            (24, "CC"),
        ];
        // let score = scoree(&timings, &mapping, 30);

        // assert_eq!(score, 1651);
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
