use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
enum Direction{
    L,
    R,
}

type Location = [char; 3];

type Route = (Location, Location);

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<Location, Route>,
    start_nodes: Vec<Location>,
    end_nodes: Vec<Location>,
}

#[derive(Debug)]
enum ParseError {
    IoError(std::io::Error),
    TryFromSliceError(std::array::TryFromSliceError),
    OtherError(&'static str),
}

impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError::IoError(error)
    }
}

impl From<std::array::TryFromSliceError> for ParseError {
    fn from(error: std::array::TryFromSliceError) -> Self {
        ParseError::TryFromSliceError(error)
    }
}

impl From<&'static str> for ParseError {
    fn from(error: &'static str) -> Self {
        ParseError::OtherError(error)
    }
}

impl Map{
    fn new() -> Self {
        Map {
            directions: Vec::new(),
            nodes: HashMap::new(),
            start_nodes: Vec::new(),
            end_nodes: Vec::new(),
        }
    }

    fn parse_map<R: BufRead>(&mut self, reader: R) -> Result<(), ParseError> {
        let mut lines = reader.lines();
        let first_line = lines.next().ok_or("Empty file")??;
        
        self.directions = Vec::new();
    
        for c in first_line.chars() {
            match c {
                'L' => self.directions.push(Direction::L),
                'R' => self.directions.push(Direction::R),
                _ => return Err("Invalid character".into()),
            }
        }
    
        lines.next(); // Skip empty line

        let nodes = &mut self.nodes;
    
        for line in lines {    
            let line = line?;

            let chars: Vec<char> = line.chars().collect();

            let name: [char; 3] = chars[0..3].try_into()?;
            let left: [char; 3] = chars[7..10].try_into()?;
            let right: [char; 3] = chars[12..15].try_into()?;

            if name[2] == 'A' {
                self.start_nodes.push(name);
            } else if name[2] == 'Z' {
                self.end_nodes.push(name);
            }

            nodes.insert(name, (left, right));
        }

        self.start_nodes.sort();
        self.end_nodes.sort();

        Ok(())
    }

    fn follow_route(&self, next: Location, route_position: usize, end_z_only: bool) -> u64 {
        let mut step = 1;
        let mut next = next;
        let mut route_position = route_position;

        loop {
            let (left, right) = self.nodes.get(&next).expect("Node not found");

            route_position = match route_position>=self.directions.len() {
                true => 0,
                false => route_position,
            };

            next = match self.directions.get(route_position as usize) {
                Some(Direction::L) => *left,
                Some(Direction::R) => *right,
                None => panic!("Invalid route"),
            };

            if !end_z_only && next == ['Z', 'Z', 'Z'] {
                break;
            } else if end_z_only && next[2] == 'Z' {
                break;
            } else {
                route_position += 1;
                step += 1;
            }
        }
        step
    }

    fn follow_route_ghost(&self) -> u64 {
        let ghost_result = self.start_nodes
            .iter()
            .map(|start| self.follow_route(*start, 0, true))
            .collect::<Vec<u64>>();
        ghost_result
            .into_iter()
            .reduce(|a, b| lcm(a, b))
            .expect("Can't calculate ghost result")
        
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut map = Map::new();
    map.parse_map(reader).expect("Can't parse map");

    let result = map.follow_route(['A', 'A', 'A'], 0, false);

    println!("Steps taken to navigate route: {}", result);

    let ghost_result = map.follow_route_ghost();

    println!("Steps as a ghost to navigate route: {}", ghost_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> String {
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".to_string()
    }

    #[test]
    fn test_parse_map() {
        let input = test_data();
        let reader = BufReader::new(input.as_bytes());
        let mut map = Map::new();
        let result = map.parse_map(reader);

        assert!(result.is_ok());

        assert_eq!(map.directions, vec![Direction::R, Direction::L]);
    
        assert_eq!(map.nodes.get(&['A', 'A', 'A']).expect("Node not found"), &(['B', 'B', 'B'], ['C', 'C', 'C']));
        assert_eq!(map.nodes.get(&['B', 'B', 'B']).expect("Node not found"), &(['D', 'D', 'D'], ['E', 'E', 'E']));
        assert_eq!(map.nodes.get(&['C', 'C', 'C']).expect("Node not found"), &(['Z', 'Z', 'Z'], ['G', 'G', 'G']));
        assert_eq!(map.nodes.get(&['D', 'D', 'D']).expect("Node not found"), &(['D', 'D', 'D'], ['D', 'D', 'D']));
        assert_eq!(map.nodes.get(&['E', 'E', 'E']).expect("Node not found"), &(['E', 'E', 'E'], ['E', 'E', 'E']));
        assert_eq!(map.nodes.get(&['G', 'G', 'G']).expect("Node not found"), &(['G', 'G', 'G'], ['G', 'G', 'G']));
        assert_eq!(map.nodes.get(&['Z', 'Z', 'Z']).expect("Node not found"), &(['Z', 'Z', 'Z'], ['Z', 'Z', 'Z']));
    }

    #[test]
    fn test_follow_simple_route() {
        let input = test_data();
        let reader = BufReader::new(input.as_bytes());
        let mut map = Map::new();
        map.parse_map(reader).expect("Can't parse map");

        assert_eq!(map.follow_route(['A', 'A', 'A'], 0, false), 2);
    }

    #[test]
    fn test_follow_advanced_route() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let reader = BufReader::new(input.as_bytes());
        let mut map = Map::new();
        map.parse_map(reader).expect("Can't parse map");

        assert_eq!(map.follow_route(['A', 'A', 'A'], 0, false), 6);
    }

    #[test]
    fn test_follow_route_ghosts() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let reader = BufReader::new(input.as_bytes());
        let mut map = Map::new();
        map.parse_map(reader).expect("Can't parse map");

        assert_eq!(map.follow_route_ghost(), 6);
    }
}