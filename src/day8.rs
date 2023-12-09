use crate::utils::open_file;
use num::integer::lcm;
use std::collections::HashMap;

pub fn day8() -> Result<u64, std::io::Error> {
    let contents = open_file("./inputs/8/input.txt")?;

    let result = resolve_puzzle(&contents);

    Ok(result)
}

fn resolve_puzzle(input: &str) -> u64 {
    let map = Map::parse(input);
    let mut ghost_map = GhostMap::new(map);

    ghost_map.resolve()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instructions(Vec<char>);

impl Instructions {
    fn parse(input: &str) -> Self {
        Self(input.chars().collect())
    }

    fn iter(&self) -> std::slice::Iter<char> {
        self.0.iter()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node(String, (String, String));

impl Node {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(" = ");
        let name = parts.next().unwrap().to_string();
        let children = parts
            .next()
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Self(name, (children[0].clone(), children[1].clone()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    instructions: Instructions,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let instructions = Instructions::parse(lines.next().unwrap());

        let nodes = lines.skip(1).map(Node::parse);

        let nodes = nodes.fold(HashMap::new(), |mut acc, node| {
            acc.insert(node.0.clone(), node.1.clone());
            acc
        });

        Self {
            instructions,
            nodes,
        }
    }

    fn resolve(&self) -> u32 {
        let mut current_node = self.nodes.get("AAA").unwrap();

        let Some(steps) =
            self.instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let next_step = match instruction {
                        'L' => current_node.0.clone(),
                        'R' => current_node.1.clone(),
                        _ => panic!("Invalid instruction"),
                    };

                    if next_step == "ZZZ" {
                        Some(index as u32 + 1)
                    } else {
                        current_node = self.nodes.get(&next_step).unwrap();
                        None
                    }
                })
        else {
            panic!("No solution found");
        };

        steps
    }
}

#[derive(Debug, Clone)]
struct GhostMap {
    map: Map,
    current_nodes: Vec<String>,
    completed_nodes: HashMap<String, usize>,
}

impl GhostMap {
    fn new(map: Map) -> Self {
        let mut current_nodes: Vec<_> = map
            .nodes
            .keys()
            .filter(|&key| key.ends_with('A'))
            .map(|k| k.to_string())
            .collect();
        current_nodes.sort();
        let completed_nodes = HashMap::new();
        Self {
            map,
            current_nodes,
            completed_nodes,
        }
    }

    fn step(&mut self, instruction: char) {
        let current_nodes: Vec<_> = self
            .current_nodes
            .iter()
            .filter(|node| !self.completed_nodes.contains_key(*node))
            .map(|node| {
                let next_node = match instruction {
                    'L' => &self.map.nodes[node].0,
                    'R' => &self.map.nodes[node].1,
                    _ => panic!("Invalid instruction"),
                };
                next_node.to_string()
            })
            .collect();

        self.current_nodes = current_nodes;
    }

    fn has_arrived(&mut self, steps: usize) -> bool {
        let completed: Vec<_> = self
            .current_nodes
            .iter()
            .filter(|node| node.ends_with('Z'))
            .filter(|node| !self.completed_nodes.contains_key(*node))
            .collect();

        for node in completed {
            self.completed_nodes.insert(node.to_string(), steps);
        }

        if self.current_nodes.is_empty() {
            dbg!(self.completed_nodes.clone());
        }

        self.current_nodes.is_empty() || self.completed_nodes.len() == self.current_nodes.len()
    }

    fn resolve(&mut self) -> u64 {
        let instructions = self.map.instructions.clone();
        let mut intruction = instructions.iter().cycle().enumerate();
        loop {
            let (step, next_instruction) = intruction.next().unwrap();
            if self.has_arrived(step) {
                break;
            }

            self.step(*next_instruction);
        }

        self.completed_nodes
            .values()
            .fold(1, |acc, &steps| lcm(acc, steps as u64))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(resolve_puzzle(input), 6);
    }

    #[test]
    fn test_resolve_possible_infinite() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(resolve_puzzle(input), 2);
    }

    #[test]
    fn test_parse_intructions() {
        let input = "RL";

        let instructions = Instructions::parse(input);

        let expected_instructions = Instructions(vec!['R', 'L']);

        assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn parse_node() {
        let input = "AAA = (BBB, CCC)";

        let node = Node::parse(input);

        let expected_node = Node("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()));

        assert_eq!(node, expected_node);
    }

    #[test]
    fn parse_map() {
        let input = "RL

AAA = (BBB, CCC)
CCC = (ZZZ, AAA)";

        let map = Map::parse(input);

        let expected_map = Map {
            instructions: Instructions(vec!['R', 'L']),
            nodes: {
                let mut nodes = HashMap::new();
                nodes.insert("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()));
                nodes.insert("CCC".to_string(), ("ZZZ".to_string(), "AAA".to_string()));
                nodes
            },
        };

        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_new_ghost_map() {
        let input = "LR

11A = (11B, XXX)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
11B = (XXX, 11Z)
XXX = (XXX, XXX)";
        let map = Map::parse(input);

        let ghost_map = GhostMap::new(map);

        assert_eq!(
            ghost_map.current_nodes,
            vec!["11A".to_string(), "22A".to_string()]
        );
    }

    #[test]
    fn test_ghost_map_step() {
        let input = "LR

11A = (11B, XXX)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
11B = (XXX, 11Z)
XXX = (XXX, XXX)";

        let map = Map::parse(input);
        let mut ghost_map = GhostMap::new(map);

        ghost_map.step('L');

        assert_eq!(
            ghost_map.current_nodes,
            vec!["11B".to_string(), "22B".to_string()]
        );
    }

    #[test]
    fn test_ghost_map_step_and_arrive() {
        let input = "LR

11A = (11Z, XXX)
11Z = (11B, XXX)
22A = (22Z, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
11B = (XXX, 11Z)
XXX = (XXX, XXX)";

        let map = Map::parse(input);
        let mut ghost_map = GhostMap::new(map);

        ghost_map.step('L');

        assert_eq!(
            ghost_map.current_nodes,
            vec!["11Z".to_string(), "22Z".to_string()]
        );

        assert!(ghost_map.has_arrived(1));
    }

    #[test]
    fn test_ghost_map_resolve() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let map = Map::parse(input);
        let mut ghost_map = GhostMap::new(map);

        let steps = ghost_map.resolve();

        assert_eq!(steps, 6);
    }
}
