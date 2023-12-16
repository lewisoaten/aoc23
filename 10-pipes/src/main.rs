// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

use std::{collections::{HashMap, HashSet}, io::{BufRead, BufReader}, env, fs::File, fmt::Display};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum TileType {
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    Start,
}

type Coordinate = (u32, u32);

#[derive(Clone)]
struct Tile {
    tile_type: TileType,
    coord: Coordinate,
}

struct Map {
    tiles: HashMap<Coordinate, Tile>,
    start: Coordinate,
}

struct Pointer {
    a_coord: Coordinate,
    b_coord: Coordinate,
    tile_visited: HashSet<Coordinate>,
    number_map: HashMap<Coordinate, String>,
    star_map: HashMap<Coordinate, String>,
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

impl From<&char> for TileType {
    fn from(c: &char) -> Self {
        match c {
            '|' => TileType::VerticalPipe,
            '-' => TileType::HorizontalPipe,
            'L' => TileType::NorthEastPipe,
            'J' => TileType::NorthWestPipe,
            '7' => TileType::SouthWestPipe,
            'F' => TileType::SouthEastPipe,
            '.' => TileType::Ground,
            'S' => TileType::Start,
            _ => panic!("Unknown tile type: {}", c),
        }
    }
}

impl From<TileType> for String {
    fn from(value: TileType) -> Self {
        match value {
            TileType::VerticalPipe => "|".to_string(),
            TileType::HorizontalPipe => "-".to_string(),
            TileType::NorthEastPipe => "L".to_string(),
            TileType::NorthWestPipe => "J".to_string(),
            TileType::SouthWestPipe => "7".to_string(),
            TileType::SouthEastPipe => "F".to_string(),
            TileType::Ground => ".".to_string(),
            TileType::Start => "S".to_string(),
            _ => panic!("Unknown tile type"),
        }
    }
}

impl From<&Tile> for String {
    fn from(tile: &Tile) -> Self {
        format!("{}", <TileType as Into<String>>::into(tile.tile_type))
    }
}

impl Tile {
    fn new(tile_type: TileType, coord: Coordinate) -> Self {
        Self {
            tile_type,
            coord,
        }
    }
}

impl Map {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            start: (0, 0),
        }
    }

    fn parse_map<R: BufRead>(reader: R) -> Result<Map, ParseError> {
        let mut map = Map::new();
        for (y, line) in reader.lines().enumerate() {
            let line = line?;

            for (x, c) in line.chars().enumerate() {
                let tile_type = TileType::from(&c);
                let coord = (x as u32, y as u32);
                let tile = Tile::new(tile_type, coord);
                if tile.tile_type == TileType::Start {
                    map.start = coord;
                }
                map.add_tile(tile);
            }
        }

        Ok(map)
    }

    fn add_tile(&mut self, tile: Tile) {
        self.tiles.insert(tile.coord, tile);
    }

    fn get_tile(&self, coord: Coordinate) -> Option<&Tile> {
        self.tiles.get(&coord)
    }
}

impl Pointer {
    fn new(coord: Coordinate) -> Self {
        Self {
            a_coord: coord,
            b_coord: coord,
            tile_visited: HashSet::new(),
            number_map: HashMap::new(),
            star_map: HashMap::new(),
        }
    }

    fn print_number_map(&self) -> String {
        let mut output = "".to_string();
        for y in 0 as u32..999 {
            if self.number_map.get(&(0 as u32,y)).is_none() {
                break;
            }
            for x in 0 as u32..999 {
                output += match self.number_map.get(&(x,y)) {
                    Some(tile) => tile,
                    None => break,
                }
            }
            output += "\n";
        }

        output
    }

    fn print_star_map(&self) -> String {
        let mut output = "".to_string();
        for y in 0 as u32..999 {
            if self.star_map.get(&(0 as u32,y)).is_none() {
                break;
            }
            for x in 0 as u32..999 {
                output += match self.star_map.get(&(x,y)) {
                    Some(tile) => tile,
                    None => break,
                }
            }
            output += "\n";
        }

        output
    }

    fn longest_unvisited_path(&mut self, map: &Map) -> u32 {
        self.tile_visited.clear();
        self.tile_visited.insert(self.a_coord);
        self.tile_visited.insert(self.b_coord);

        for tile in map.tiles.keys() {
            match map.tiles.get(tile) {
                Some(tile) => {
                    self.number_map.insert(tile.coord, <&Tile as Into<String>>::into(tile));
                    self.star_map.insert(tile.coord, <&Tile as Into<String>>::into(tile));
                },
                None => (),
            };
        }
        
        let mut step = 0;

        let mut a_dead_end = false;
        let mut b_dead_end = false;

        let mut a_previous_coord = self.a_coord;
        let mut b_previous_coord = self.b_coord;

        //Need to get starting two options by checking adjacent tiles for matching routes out.
        // then need to follow each route until the next part is already visited and exit with that number (do so for both routes)
        loop {
            if !a_dead_end {
                self.a_coord = match self.proceed(a_previous_coord, self.a_coord, map) {
                    Some(coord) => {
                        self.number_map.insert(coord, format!("{}", step+1));
                        self.star_map.insert(coord, "*".to_string() );
                        a_previous_coord = self.a_coord;
                        coord
                    },
                    None => {
                        a_dead_end = true;
                        (0,0)
                    },
                };
            }

            if !b_dead_end {
                self.b_coord = match self.proceed(b_previous_coord, self.b_coord, map) {
                    Some(coord) => {
                        self.number_map.insert(coord, format!("{}", step+1));
                        b_previous_coord = self.b_coord;
                        coord
                    },
                    None => {
                        b_dead_end = true;
                        (0,0)
                    },
                };
            }

            if a_dead_end && b_dead_end {
                break;
            }

            step += 1;
        }
        
        step
    }

    fn proceed(&mut self, previous_coord: Coordinate, starting_coord: Coordinate, map: &Map) -> Option<Coordinate> {
        let mut next_coord: Option<Coordinate> = None;

        if let Some(tile) = map.get_tile(starting_coord) {
            next_coord = match tile.tile_type {
                TileType::VerticalPipe => {
                    if previous_coord.1 < starting_coord.1 {
                        Some((starting_coord.0, starting_coord.1 + 1))
                    } else {
                        Some((starting_coord.0, starting_coord.1 - 1))
                    }
                },
                TileType::HorizontalPipe => {
                    if previous_coord.0 < starting_coord.0  {
                        Some((starting_coord.0 + 1, starting_coord.1))
                    } else {
                        Some((starting_coord.0 - 1, starting_coord.1))
                    }
                },
                TileType::NorthEastPipe => {
                    if previous_coord.1 < starting_coord.1 {
                        Some((starting_coord.0 + 1, starting_coord.1))
                    } else {
                        Some((starting_coord.0, starting_coord.1 - 1))
                    }
                },
                TileType::NorthWestPipe => {
                    if previous_coord.1 < starting_coord.1 {
                        Some((starting_coord.0 - 1, starting_coord.1))
                    } else {
                        Some((starting_coord.0, starting_coord.1 - 1))
                    }
                },
                TileType::SouthWestPipe => {
                    if previous_coord.1 > starting_coord.1 {
                        Some((starting_coord.0 - 1, starting_coord.1))
                    } else {
                        Some((starting_coord.0, starting_coord.1 + 1))
                    }
                },
                TileType::SouthEastPipe => {
                    if previous_coord.1 > starting_coord.1 {
                        Some((starting_coord.0 + 1, starting_coord.1))
                    } else {
                        Some((starting_coord.0, starting_coord.1 + 1))
                    }
                },
                TileType::Ground => { None },
                TileType::Start => { 
                    self.find_starting_route(starting_coord, map)
                 },
            };
        }

        if self.tile_visited.contains(&next_coord?) || map.get_tile(next_coord?).is_none() {
            return None;
        }

        self.tile_visited.insert(next_coord?);

        next_coord
    }

    fn find_starting_route(&self, starting_coord: Coordinate, map: &Map) -> Option<Coordinate> {
        let mut next_step: Option<Coordinate> = None;
        
        for coord in (-1 as i32..2).flat_map(move |a| (-1 as i32..2).map(move |b| (a, b))) {
            if starting_coord.0 == 0 && coord.0 <0 {
                continue;
            }
            if starting_coord.1 == 0 && coord.1 <0 {
                continue;
            }
            
            let test_coord = ((starting_coord.0 as i32 + coord.0) as u32, (starting_coord.1 as i32 + coord.1) as u32);
            if self.tile_visited.contains(&test_coord) {
                continue;
            }

            if let Some(tile) = map.get_tile(test_coord) {
                next_step = match tile.tile_type {
                    TileType::VerticalPipe => {
                        if coord.0 == 0 {
                            Some(test_coord)
                        } else { None }
                    },
                    TileType::HorizontalPipe => {
                        if coord.1 == 0 {
                            Some(test_coord)
                        } else { None }
                    },
                    TileType::NorthEastPipe => {
                        if (coord.0 == 0 && coord.1 == 1) || (coord.0 == -1 && coord.1 == 0) {
                            Some(test_coord)
                        } else { None }
                    },
                    TileType::NorthWestPipe => {
                        if (coord.0 == 0 && coord.1 == 1) || (coord.0 == 1 && coord.1 == 0) {
                            Some(test_coord)
                        } else { None }
                    },
                    TileType::SouthWestPipe => {
                        if (coord.0 == 0 && coord.1 == -1) || (coord.0 == 1 && coord.1 == 0) {
                            Some(test_coord)
                        } else { None }
                    },
                    TileType::SouthEastPipe => {
                        if (coord.0 == 0 && coord.1 == -1) || (coord.0 == -1 && coord.1 == 0) {
                            Some(test_coord)
                        } else { None }
                    },
                    TileType::Ground => { None },
                    TileType::Start => { None },
                };

                if next_step.is_some() {
                    break;
                }
            }
        }
        next_step
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let map = Map::parse_map(reader).expect("Parsed map");

    let mut pointer = Pointer::new(map.start);
    let longest_path = pointer.longest_unvisited_path(&map);

    println!("{}", pointer.print_star_map());

    println!("Longest path: {}", longest_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
".....
.S-7.
.|.|.
.L-J.
....."
    }

    #[test]
    fn test_add_tile() {
        let mut map = Map::new();
        let tile = Tile::new(TileType::Ground, (0, 0));
        map.add_tile(tile);

        assert_eq!(map.tiles.len(), 1);
    }

    #[test]
    fn test_get_tile() {
        let mut map = Map::new();
        let tile = Tile::new(TileType::Ground, (0, 0));
        map.add_tile(tile);

        let retrieved_tile = map.get_tile((0, 0));

        assert_eq!(retrieved_tile.unwrap().tile_type, TileType::Ground);
    }

    #[test]
    fn test_parse_map() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let map = Map::parse_map(reader).unwrap();

        assert_eq!(map.tiles.len(), 25);
        assert_eq!(map.start, (1, 1));
    }

    #[test]
    fn test_longest_unvisited_path() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let map = Map::parse_map(reader).unwrap();

        let mut pointer = Pointer::new(map.start);
        let longest_path = pointer.longest_unvisited_path(&map);

        println!("{}", pointer.print_number_map());

        assert_eq!(longest_path, 4);
    }
}