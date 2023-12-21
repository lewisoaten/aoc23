use std::{io::{BufRead, BufReader}, env, fs::File, collections::HashMap, hash::Hasher};

enum Tile {
    Empty,
    RightMirror,
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}
type Coordinate = (usize, usize);
struct Contraption {
    tiles: HashMap<Coordinate, Tile>,
    max_x: usize,
    max_y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct BeamManager {
    beams: Vec<Beam>,
    energised_map: HashMap<Coordinate, bool>,
    first_startup: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Beam {
    coord: Coordinate,
    direction: Direction,
    off_map: bool,
}

#[derive(Debug)]
enum ParseError {
    IoError(std::io::Error),
    TryFromSliceError(std::array::TryFromSliceError),
    OtherError(&'static str),
    ParseIntError(std::num::ParseIntError),
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

impl From<std::num::ParseIntError> for ParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        ParseError::ParseIntError(error)
    }
}

impl Contraption {
    fn new(tiles: HashMap<Coordinate, Tile>) -> Contraption {
        let max_x = match tiles.keys().map(|(x, _)| x).max() {
            Some(x) => x.clone(),
            None => 0,
        };
        let max_y = match tiles.keys().map(|(_, y)| y).max() {
            Some(y) => y.clone(),
            None => 0,
        };
        Contraption {
            tiles,
            max_x,
            max_y,
        }
    }

    fn parse_platform<R: BufRead>(reader: R) -> Result<Contraption, ParseError> {
        let mut tiles = HashMap::new();
        for (y, line) in reader.lines().enumerate() {
            let line = line?;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => tiles.insert((x, y), Tile::Empty),
                    '/' => tiles.insert((x, y), Tile::RightMirror),
                    '\\' => tiles.insert((x, y), Tile::LeftMirror),
                    '|' => tiles.insert((x, y), Tile::VerticalSplitter),
                    '-' => tiles.insert((x, y), Tile::HorizontalSplitter),
                    _ => return Err(ParseError::OtherError("Invalid character")),
                };
            }
        }

        let platform = Contraption::new(tiles);

        Ok(platform)
    }
}

impl std::hash::Hash for Beam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
        self.direction.hash(state);
        self.off_map.hash(state);
    }
}

impl Beam {
    fn new(coord: Coordinate, direction: Direction) -> Beam {
        let mut energised_map = HashMap::new();
        energised_map.insert(coord, true);
        Beam {
            coord,
            direction,
            off_map: false,
        }
    }

    fn step(&mut self, contraption: &Contraption, first: bool) -> Option<Beam> {
        if self.off_map {
            return None;
        }
        let (x, y) = self.coord;
        match self.direction {
            Direction::Up => {
                if y == 0 {
                    self.off_map = true;
                    return None;
                } else if !first {
                    self.coord = (x, y - 1);
                }

                match contraption.tiles.get(&self.coord) {
                    Some(Tile::Empty) | Some(Tile::VerticalSplitter) => {
                        return None;
                    },
                    Some(Tile::RightMirror) => {
                        self.direction = Direction::Right;
                        return None;
                    },
                    Some(Tile::LeftMirror) => {
                        self.direction = Direction::Left;
                        return None;
                    },
                    Some(Tile::HorizontalSplitter) => {
                        self.direction = Direction::Right;
                        return Some(Beam::new(self.coord, Direction::Left));
                    },
                    None => {
                        panic!("Beam found a hole in the contraption");
                    },
                }
            },
            Direction::Down => {
                if y == contraption.max_y {
                    self.off_map = true;
                    return None;
                } else if !first {
                    self.coord = (x, y + 1);
                }

                match contraption.tiles.get(&self.coord) {
                    Some(Tile::Empty) | Some(Tile::VerticalSplitter) => {
                        return None;
                    },
                    Some(Tile::RightMirror) => {
                        self.direction = Direction::Left;
                        return None;
                    },
                    Some(Tile::LeftMirror) => {
                        self.direction = Direction::Right;
                        return None;
                    },
                    Some(Tile::HorizontalSplitter) => {
                        self.direction = Direction::Left;
                        return Some(Beam::new(self.coord, Direction::Right));
                    },
                    None => {
                        panic!("Beam found a hole in the contraption");
                    },
                }
            },
            Direction::Left => {
                if x == 0 {
                    self.off_map = true;
                    return None;
                } else if !first {
                    self.coord = (x - 1, y);
                }

                match contraption.tiles.get(&self.coord) {
                    Some(Tile::Empty) | Some(Tile::HorizontalSplitter) => {
                        return None;
                    },
                    Some(Tile::RightMirror) => {
                        self.direction = Direction::Down;
                        return None;
                    },
                    Some(Tile::LeftMirror) => {
                        self.direction = Direction::Up;
                        return None;
                    },
                    Some(Tile::VerticalSplitter) => {
                        self.direction = Direction::Down;
                        return Some(Beam::new(self.coord, Direction::Up));
                    },
                    None => {
                        panic!("Beam found a hole in the contraption");
                    },
                }
            },
            Direction::Right => {
                if x == contraption.max_x {
                    self.off_map = true;
                    return None;
                } else if !first {
                    self.coord = (x + 1, y);
                }

                match contraption.tiles.get(&self.coord) {
                    Some(Tile::Empty) | Some(Tile::HorizontalSplitter) => {
                        return None;
                    },
                    Some(Tile::RightMirror) => {
                        self.direction = Direction::Up;
                        return None;
                    },
                    Some(Tile::LeftMirror) => {
                        self.direction = Direction::Down;
                        return None;
                    },
                    Some(Tile::VerticalSplitter) => {
                        self.direction = Direction::Up;
                        return Some(Beam::new(self.coord, Direction::Down));
                    },
                    None => {
                        panic!("Beam found a hole in the contraption");
                    },
                }
            },
        }
    }
}

impl BeamManager {
    fn new() -> BeamManager {
        BeamManager {
            beams: Vec::new(),
            energised_map: HashMap::new(),
            first_startup: true,
        }
    }

    fn step(&mut self, contraption: &Contraption) -> bool {
        let energised_tiles = self.get_energised_tiles();

        let mut new_beams = Vec::new();
        for beam in self.beams.iter_mut() {
            if let Some(new_beam) = beam.step(contraption, self.first_startup) {
                new_beams.push(new_beam);
                
            }
        }
        self.beams.extend(&new_beams);
        self.first_startup = false;

        //Dedupe beams
        self.beams.sort();
        self.beams.dedup();

        self.energise();

        let new_energised_tiles = self.get_energised_tiles();
        
        new_energised_tiles > energised_tiles
    }

    fn get_energised_tiles(&self) -> usize {
        self.energised_map.values().filter(|v| **v).count()
    }

    fn energise(&mut self) {
        for beam in self.beams.iter() {
            self.energised_map.insert(beam.coord, true);
        }
    }
}

impl std::fmt::Display for BeamManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();

        let max_x = self.energised_map.keys().map(|(x, _)| x).max().unwrap().clone();
        let max_y = self.energised_map.keys().map(|(_, y)| y).max().unwrap().clone();
        
        for y in 0..=max_y {
            for x in 0..=max_x {
                let coord = (x, y);
                let beams_here: Vec<Beam> = self.beams.iter().filter(|beam| !beam.off_map && beam.coord == coord).map(|beam| *beam ).collect();
                
                if beams_here.len() > 1 {
                    output += format!("{}", beams_here.len()).as_str();
                } else if beams_here.len() == 1 {
                    match beams_here[0].direction {
                        Direction::Up => output.push('^'),
                        Direction::Down => output.push('v'),
                        Direction::Left => output.push('<'),
                        Direction::Right => output.push('>'),
                    }
                } else if let Some(true) = self.energised_map.get(&coord) {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let contraption = Contraption::parse_platform(reader).expect("Parsed platform");

    let mut beam_manager = BeamManager::new();
    let beam = Beam::new((0, 0), Direction::Right);
    beam_manager.beams.push(beam);

    print!("{}[2J", 27 as char);

    let mut i = 0;

    //Do it 3 times to get over short loops where the number of energised tiles doesn't change
    loop {
        let mut cont = Vec::new();
        let step = 100;
        for _ in 0..step {
            cont.push(beam_manager.step(&contraption));
        }

        if cont.iter().all(|v| !*v) {
            break;
        }

        i += step;

        let output = format!("{}", beam_manager);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Step {}", i);
        println!("{}", output);
        println!("");
        std::thread::sleep(std::time::Duration::from_millis(50));

        
    }

    println!("Steps required: {}", i);

    println!("Count of energised tiles: {}", beam_manager.get_energised_tiles());

    let mut max_energised_tiles = 0;
    // Test top side
    for col in 0..=contraption.max_x {
        let mut beam_manager = BeamManager::new();
        let beam = Beam::new((col, 0), Direction::Down);
        beam_manager.beams.push(beam);

        loop {
            let mut cont = Vec::new();
            let step = 100;
            for _ in 0..step {
                cont.push(beam_manager.step(&contraption));
            }

            if cont.iter().all(|v| !*v) {
                break;
            }
        }

        let energised_tiles = beam_manager.get_energised_tiles();
        if energised_tiles > max_energised_tiles {
            println!("New max: {} at col {} down", energised_tiles, col);
            max_energised_tiles = energised_tiles;
        }   
    }

    // Test right side
    for row in 0..=contraption.max_y {
        let mut beam_manager = BeamManager::new();
        let beam = Beam::new((contraption.max_x, row), Direction::Left);
        beam_manager.beams.push(beam);

        loop {
            let mut cont = Vec::new();
            let step = 100;
            for _ in 0..step {
                cont.push(beam_manager.step(&contraption));
            }

            if cont.iter().all(|v| !*v) {
                break;
            }
        }

        let energised_tiles = beam_manager.get_energised_tiles();
        if energised_tiles > max_energised_tiles {
            println!("New max: {} at row {} left", energised_tiles, row);
            max_energised_tiles = energised_tiles;
        }   
    }

    // Test bottom side
    for col in 0..=contraption.max_x {
        let mut beam_manager = BeamManager::new();
        let beam = Beam::new((col, contraption.max_y), Direction::Up);
        beam_manager.beams.push(beam);

        loop {
            let mut cont = Vec::new();
            let step = 100;
            for _ in 0..step {
                cont.push(beam_manager.step(&contraption));
            }

            if cont.iter().all(|v| !*v) {
                break;
            }
        }

        let energised_tiles = beam_manager.get_energised_tiles();
        if energised_tiles > max_energised_tiles {
            println!("New max: {} at col {} up", energised_tiles, col);
            max_energised_tiles = energised_tiles;
        }   
    }

    // Test left side
    for row in 1..=contraption.max_y {
        let mut beam_manager = BeamManager::new();
        let beam = Beam::new((0, row), Direction::Right);
        beam_manager.beams.push(beam);

        loop {
            let mut cont = Vec::new();
            let step = 100;
            for _ in 0..step {
                cont.push(beam_manager.step(&contraption));
            }

            if cont.iter().all(|v| !*v) {
                break;
            }
        }

        let energised_tiles = beam_manager.get_energised_tiles();
        if energised_tiles > max_energised_tiles {
            println!("New max: {} at row {} right", energised_tiles, row);
            max_energised_tiles = energised_tiles;
        }   
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
    }

    #[test]
    fn test_parse_patterns() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let contraption = Contraption::parse_platform(reader).unwrap();

        assert_eq!(contraption.tiles.len(), 100);
    }

    #[test]
    fn test_step() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let contraption = Contraption::parse_platform(reader).unwrap();

        let mut beam_manager = BeamManager::new();
        let beam = Beam::new((0, 0), Direction::Right);
        beam_manager.beams.push(beam);
        beam_manager.step(&contraption);
        beam_manager.energise();

        assert_eq!(format!("{}", beam_manager),
r".2
");

        beam_manager.step(&contraption);
        beam_manager.energise();

        assert_eq!(format!("{}", beam_manager),
r".^
.v
");

        beam_manager.step(&contraption);
        beam_manager.energise();

        assert_eq!(format!("{}", beam_manager),
r".^
.#
.v
");
    }

    #[test]
    fn test_count_energised_tiles() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let contraption = Contraption::parse_platform(reader).unwrap();

        let mut beam_manager = BeamManager::new();
        let beam = Beam::new((0, 0), Direction::Right);
        beam_manager.beams.push(beam);
        beam_manager.step(&contraption);
        beam_manager.energise();

        assert_eq!(beam_manager.get_energised_tiles(), 1);

        beam_manager.step(&contraption);
        beam_manager.energise();

        assert_eq!(beam_manager.get_energised_tiles(), 2);

        beam_manager.step(&contraption);
        beam_manager.energise();

        assert_eq!(beam_manager.get_energised_tiles(), 3);
    }

    #[test]
    fn test_process_to_completion() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let contraption = Contraption::parse_platform(reader).unwrap();

        let mut beam_manager = BeamManager::new();
        let beam = Beam::new((0, 0), Direction::Right);
        beam_manager.beams.push(beam);

        while beam_manager.step(&contraption) {
            ()
        }

        beam_manager.step(&contraption);

        println!("{}", beam_manager);

        assert_eq!(beam_manager.get_energised_tiles(), 46);
    }
}