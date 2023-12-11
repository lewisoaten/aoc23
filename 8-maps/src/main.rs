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

            nodes.insert(name, (left, right));
        }

        Ok(())
    }

    fn follow_route(&self, next: Location, route_position: usize) -> u32 {
        let (left, right) = self.nodes.get(&next).expect("Node not found");

        let route_position = match route_position>=self.directions.len() {
            true => 0,
            false => route_position,
        };

        let next = match self.directions.get(route_position as usize) {
            Some(Direction::L) => left,
            Some(Direction::R) => right,
            None => panic!("Invalid route"),
        };

        if next == &['Z', 'Z', 'Z'] {
            return 1;
        } else {
            return 1 + self.follow_route(*next, route_position + 1)
        }
    }
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut map = Map::new();
    map.parse_map(reader).expect("Can't parse map");

    let result = map.follow_route(['A', 'A', 'A'], 0);

    println!("Steps taken to navigate route: {}", result);
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

        assert_eq!(map.follow_route(['A', 'A', 'A'], 0), 2);
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

        assert_eq!(map.follow_route(['A', 'A', 'A'], 0), 6);
    }
}