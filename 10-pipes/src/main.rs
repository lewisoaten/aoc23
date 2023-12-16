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
    star_map: Vec<Coordinate>,
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
            star_map: Vec::new(),
        }
    }

    fn print_star_map(&self) -> String {
        let mut output = "".to_string();

        let max_x = self.star_map.iter().map(|(x, _)| x).max().unwrap()+1;
        let max_y = self.star_map.iter().map(|(_, y)| y).max().unwrap()+1;

        for y in 0 as u32..max_y {
            for x in 0 as u32..max_x {
                output += match self.star_map.contains(&(x,y)) {
                    true => "*",
                    false => " ",
                }
            }
            output += "\n";
        }

        output
    }

    fn print_number_map(&self) -> String {
        let mut output = "".to_string();

        let max_x = self.number_map.keys().max_by_key(|(x, _)| x).unwrap().0+1;
        let max_y = self.number_map.keys().max_by_key(|(_, y)| y).unwrap().1+1;

        for y in 0 as u32..max_y {
            for x in 0 as u32..max_x {
                output += match self.number_map.get(&(x,y)) {
                    Some(tile) => tile,
                    None => " ",
                }
            }
            output += "\n";
        }

        output
    }

    fn walk_tunnel(&mut self, map: &Map) {
        self.star_map.clear();

        let mut previous_coord = map.start;
        let mut current_coord = map.start;

        self.star_map.push(previous_coord);

        loop {
            current_coord = match Pointer::proceed(previous_coord, current_coord, map, None) {
                Some(coord) => {
                    if self.star_map.contains(&coord) {
                        break;
                    } else {
                        self.star_map.push(coord);
                        previous_coord = current_coord;
                        coord
                    }
                },
                None => {
                    break;
                },
            };
        }
    }

    fn longest_unvisited_path(&mut self, map: &Map) -> u32 {
        self.tile_visited.clear();
        self.tile_visited.insert(self.a_coord);
        self.tile_visited.insert(self.b_coord);
        
        let mut step = 0;

        let mut a_dead_end = false;
        let mut b_dead_end = false;

        let mut a_previous_coord = self.a_coord;
        let mut b_previous_coord = self.b_coord;

        self.number_map.insert(self.a_coord, format!("{}", step));

        //Need to get starting two options by checking adjacent tiles for matching routes out.
        // then need to follow each route until the next part is already visited and exit with that number (do so for both routes)
        loop {
            if !a_dead_end {
                self.a_coord = match Pointer::proceed(a_previous_coord, self.a_coord, map, None) {
                    Some(coord) => {
                        if self.tile_visited.contains(&coord) {
                            a_dead_end = true;
                            (0,0)
                        } else {
                            self.tile_visited.insert(coord);
                            self.number_map.insert(coord, format!("{}", step+1));
                            a_previous_coord = self.a_coord;
                            coord
                        }
                    },
                    None => {
                        a_dead_end = true;
                        (0,0)
                    },
                };
            }

            if !b_dead_end {
                self.b_coord = match Pointer::proceed(b_previous_coord, self.b_coord, map, Some(self.a_coord)) {
                    Some(coord) => {
                        if self.tile_visited.contains(&coord) {
                            b_dead_end = true;
                            (0,0)
                        } else {
                            self.tile_visited.insert(coord);
                            self.number_map.insert(coord, format!("{}", step+1));
                            b_previous_coord = self.b_coord;
                            coord
                        }
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

    fn proceed(previous_coord: Coordinate, starting_coord: Coordinate, map: &Map, ignore_coord: Option<Coordinate>) -> Option<Coordinate> {
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
                    Pointer::find_starting_route(starting_coord, map, ignore_coord)
                 },
            };
        }

        if map.get_tile(next_coord?).is_none() {
            return None;
        }

        next_coord
    }

    fn find_starting_route(starting_coord: Coordinate, map: &Map, ignore_coord: Option<Coordinate>) -> Option<Coordinate> {
        let mut next_step: Option<Coordinate> = None;
        
        for coord in (-1 as i32..2).flat_map(move |a| (-1 as i32..2).map(move |b| (a, b))) {
            if starting_coord.0 == 0 && coord.0 <0 {
                continue;
            }
            if starting_coord.1 == 0 && coord.1 <0 {
                continue;
            }
            
            let test_coord = ((starting_coord.0 as i32 + coord.0) as u32, (starting_coord.1 as i32 + coord.1) as u32);

            if let Some(ignore_coord) = ignore_coord {
                if test_coord == ignore_coord {
                    continue;
                }
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

    fn winding_number(&self, point: Coordinate) -> i32 {
        let mut winding_number = 0;
        let n = self.star_map.len();
        for i in 0..n {
            let v1 = self.star_map[i];
            let v2 = self.star_map[(i + 1) % n];
            if v1.1 <= point.1 {
                if v2.1 > point.1 && self.is_left(&v1, &v2, &point) > 0 {
                    winding_number += 1;
                }
            } else {
                if v2.1 <= point.1 && self.is_left(&v1, &v2, &point) < 0 {
                    winding_number -= 1;
                }
            }
        }
        winding_number
    }

    fn is_left(&self, v1: &Coordinate, v2: &Coordinate, point: &Coordinate) -> i32 {
        (v2.0 as i32 - v1.0 as i32) * (point.1 as i32 - v1.1 as i32) - (point.0 as i32 - v1.0 as i32) * (v2.1 as i32 - v1.1 as i32)
    }

    fn is_inside(&self, point: Coordinate) -> bool {
        self.winding_number(point) != 0
    }

    fn tiles_inside_loop(&self, map: &Map) -> u32 {
        let mut tiles_inside = 0;

        let non_loop_tiles = map
            .tiles
            .iter()
            .filter(|(coord, _)| !self.star_map.contains(coord))
            .map(|(coord, _)| coord)
            .collect::<Vec<&Coordinate>>();

        for tile in non_loop_tiles {
            if self.is_inside(*tile) {
                tiles_inside += 1;
            }
        }

        tiles_inside
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

    pointer.walk_tunnel(&map);

    println!("{}", pointer.print_star_map());

    println!("Longest path: {}", longest_path);

    println!("Tiles inside loop: {}", pointer.tiles_inside_loop(&map));
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

        pointer.walk_tunnel(&map);

        println!("{}", pointer.print_star_map());
        println!("{}", pointer.print_number_map());

        assert_eq!(longest_path, 4);
    }

    #[test]
    fn test_winding_number() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let map = Map::parse_map(reader).unwrap();

        let mut pointer = Pointer::new(map.start);
        pointer.longest_unvisited_path(&map);

        pointer.walk_tunnel(&map);

        println!("{}", pointer.print_star_map());



        assert_eq!(pointer.is_inside((0, 0)), false);
        assert_eq!(pointer.is_inside((2, 2)), true);
    }


    #[test]
    fn test_tiles_inside_loop() {
        let input = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let reader = std::io::Cursor::new(input);
        let map = Map::parse_map(reader).unwrap();

        let mut pointer = Pointer::new(map.start);

        pointer.walk_tunnel(&map);

        println!("{}", pointer.print_star_map());

        assert_eq!(pointer.tiles_inside_loop(&map), 4);
    }

    #[test]
    fn test_tiles_inside_loop_no_gap() {
        let input = "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        let reader = std::io::Cursor::new(input);
        let map = Map::parse_map(reader).unwrap();

        let mut pointer = Pointer::new(map.start);

        pointer.walk_tunnel(&map);

        println!("{}", pointer.print_star_map());

        assert_eq!(pointer.tiles_inside_loop(&map), 4);
    }

    #[test]
    fn test_tiles_inside_loop_larger_example() {
        let input = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let reader = std::io::Cursor::new(input);
        let map = Map::parse_map(reader).unwrap();

        let mut pointer = Pointer::new(map.start);

        pointer.walk_tunnel(&map);

        println!("{}", pointer.print_star_map());

        assert_eq!(pointer.tiles_inside_loop(&map), 8);
    }

    #[test]
    fn test_tiles_inside_loop_junk() {
        let input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let reader = std::io::Cursor::new(input);
        let map = Map::parse_map(reader).unwrap();

        let mut pointer = Pointer::new(map.start);

        pointer.walk_tunnel(&map);

        println!("{}", pointer.print_star_map());

        assert_eq!(pointer.tiles_inside_loop(&map), 10);
    }
}