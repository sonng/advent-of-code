use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

use anyhow::Result;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day7.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let (_root, nodes) = parse(input)?;

    let mut result = 0;
    for (name, node) in nodes.iter() {
        let file_sizes = node.size(&nodes);
        if file_sizes <= 100000 {
            result += file_sizes;
        }
    }

    println!("Day 7-1: {}", result);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let (root, nodes) = parse(input)?;

    const MAX_SIZE: u64 = 70000000;
    const UPDATE_SIZE: u64 = 30000000;
    let total_size = root.size(&nodes);
    let delete_size: u64 = (total_size + UPDATE_SIZE) - MAX_SIZE;
    println!("{:?}", root.size(&nodes));

    let mut minimum_sized_dir = u64::MAX;
    let mut biggest = 0;
    for (name, node) in nodes.iter() {
        let file_sizes = node.size(&nodes);

        biggest = max(biggest, file_sizes);

        if file_sizes >= delete_size {
            // println!("{:?} - {:?}", name, file_sizes);
            minimum_sized_dir = min(minimum_sized_dir, file_sizes);
        }
    }

    // println!("Biggest: {:?}", biggest);
    // println!("Delete Size: {:?}", delete_size);

    println!("Day 7-2: {}", minimum_sized_dir);
    Ok(())
}

fn parse(input: &str) -> Result<(Node, HashMap<String, Node>)> {
    let mut nodes = HashMap::<String, Node>::new();
    let input: Vec<&str> = input.split('\n').collect();

    nodes.insert(
        "/".into(),
        Node::Directory {
            items: vec![],
            name: "/".into(),
        },
    );

    let mut i = 0;
    let mut current_folder = vec![];
    while i < input.len() {
        let line = input[i];

        if line == "$ cd .." {
            if current_folder.len() >= 2 {
                current_folder.pop();
            } else {
                println!("Trying to go lower than root!");
            }
        } else if line == "$ ls" {
        } else if &line[0..5] == "$ cd " {
            let name = &line[5..];
            // println!("Go into '{:?}'", name);
            current_folder.push(name);
        } else if &line[0..4] == "dir " {
            let parent_name = current_folder.join("/")[1..].to_string();
            let name = &line[4..];
            let full_path = parent_name.clone() + "/" + name;
            // println!("CurrentFolder {:?}", current_folder);
            // println!("Parsing folder {:?}", full_path);
            nodes.entry(full_path.clone()).or_insert(Node::Directory {
                items: vec![],
                name: full_path.clone(),
            });

            let parent_name = if parent_name.is_empty() {
                "/"
            } else {
                &parent_name
            };

            nodes.entry(parent_name.to_string()).and_modify(|e| {
                // println!("Modifying {:?}", e);
                insert_item(
                    e,
                    Node::Directory {
                        items: vec![],
                        name: full_path.into(),
                    },
                );
            });
            // .or_insert(Node::Directory {
            //     items: vec![],
            //     name: parent_name.to_string(),
            // });
            // println!("{:?}", nodes);
        } else {
            let split: Vec<&str> = line.split(' ').collect();

            let size: u64 = split[0].parse().unwrap();
            let filename: String = split[1].into();
            // println!("Parsing File {:?} - {:?}", size, filename);

            let parent_name = current_folder.join("/")[1..].to_string();
            nodes.entry(parent_name).and_modify(|e| {
                // println!("Modifying-File {:?}", e);
                insert_item(
                    e,
                    Node::File {
                        size: size,
                        name: filename.into(),
                    },
                );
            });
        }

        i += 1;
    }

    // println!("All Nodes: {:?}", nodes);

    Ok((nodes.get("/".into()).unwrap().clone(), nodes))
}

fn insert_item(node: &mut Node, item: Node) {
    match node {
        Node::Directory { items, name: _ } => {
            items.push(item);
        }
        Node::File { size, name } => {}
    }
}

#[derive(Clone, Debug)]
enum Node {
    Directory { items: Vec<Node>, name: String },
    File { size: u64, name: String },
}

impl Node {
    fn size(&self, folder_mapping: &HashMap<String, Node>) -> u64 {
        match self {
            Self::Directory { items, name } => {
                let folder_sizes: u64 = self
                    .folders()
                    .iter()
                    .flat_map(|name| folder_mapping.get(name))
                    .map(|n| n.size(folder_mapping))
                    .sum();

                items
                    .iter()
                    .filter(|n| n.is_file())
                    .map(|n| n.size(&folder_mapping))
                    .sum::<u64>()
                    + folder_sizes
            }
            Self::File { size, name } => *size,
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Directory { items, name } => name.into(),
            Self::File { size, name } => name.into(),
        }
    }

    fn is_directory(&self) -> bool {
        if let Self::Directory { name: _, items: _ } = self {
            true
        } else {
            false
        }
    }

    fn is_file(&self) -> bool {
        if let Self::Directory { name: _, items: _ } = self {
            false
        } else {
            true
        }
    }

    fn folders(&self) -> Vec<String> {
        match self {
            Self::File { size, name } => vec![],
            Self::Directory { items, name } => items
                .iter()
                .filter(|n| n.is_directory())
                .map(|n| n.name())
                .collect(),
        }
    }
}
