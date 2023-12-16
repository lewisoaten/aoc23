use std::{io::{BufRead, BufReader}, env, fs::File};

type Coordinate = (u64, u64);

#[derive(Debug, Clone)]
struct Observation {
    galaxies: Vec<Coordinate>,
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

impl Observation {
    fn new() -> Observation {
        Observation {
            galaxies: Vec::new(),
        }
    }

    fn add_galaxy(&mut self, galaxy: Coordinate) {
        self.galaxies.push(galaxy);
    }

    fn parse_map<R: BufRead>(reader: R) -> Result<Observation, ParseError> {
        let mut observation = Observation::new();
        for (y, line) in reader.lines().enumerate() {
            let line = line?;

            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    observation.add_galaxy((x as u64, y as u64));
                }
            }
        }

        Ok(observation)
    }

    fn perform_expansion(&mut self, amount: u64) {
        //Find columns without any galaxies
        let mut empty_columns = Vec::new();
        let max_x = self.galaxies.iter().map(|(x, _)| x).max().unwrap()+1;
        
        for x in 0..max_x {
            if self.galaxies.iter().find(|(gx, _)| *gx == x).is_none() {
                empty_columns.push(x);
            }
        }
        
        //Find rows without any galaxies
        let mut empty_rows = Vec::new();
        let max_y = self.galaxies.iter().map(|(_, y)| y).max().unwrap()+1;

        for y in 0..max_y {
            if self.galaxies.iter().find(|(_, gy)| *gy == y).is_none() {
                empty_rows.push(y);
            }
        }

        //Expand galaxies
        let mut new_galaxies = Vec::new();
        let mut new_x = 0;
        let mut new_y = 0;

        for y in 0..max_y {
            if empty_rows.contains(&y) {
                new_y += amount;
            }
            for x in 0..max_x {
                if empty_columns.contains(&x) {
                    new_x += amount;
                }

                if self.galaxies.contains(&(x,y)) {
                    new_galaxies.push((new_x, new_y));
                }

                new_x += 1;
            }

            new_x = 0;
            new_y += 1;
        }

        self.galaxies = new_galaxies;
    }

    fn print_observation(&self) -> String {
        let mut output = "".to_string();

        let max_x = self.galaxies.iter().map(|(x, _)| x).max().unwrap()+1;
        let max_y = self.galaxies.iter().map(|(_, y)| y).max().unwrap()+1;

        for y in 0 as u64..max_y {
            for x in 0 as u64..max_x {
                output += match self.galaxies.contains(&(x,y)) {
                    true => "#",
                    false => ".",
                }
            }
            output += "\n";
        }

        output
    }

    fn calculate_distance(&self, galaxy1: Coordinate, galaxy2: Coordinate) -> u64 {
        let x1 = galaxy1.0 as i64;
        let y1 = galaxy1.1 as i64;
        let x2 = galaxy2.0 as i64;
        let y2 = galaxy2.1 as i64;

        ((x1-x2).abs() + (y1-y2).abs()) as u64
    }

    fn distance_combinations(&self) -> u64 {
        let mut combinations = 0 as u64;

        for (i, galaxy1) in self.galaxies.iter().enumerate() {
            for galaxy2 in self.galaxies.iter().skip(i+1) {
                combinations += self.calculate_distance(*galaxy1, *galaxy2);
            }
        }

        combinations
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut observation = Observation::parse_map(reader).expect("Parsed map");

    let mut observation_two = observation.clone();

    observation.perform_expansion(1);

    println!("Distance combinations (part1):{}", observation.distance_combinations());

    observation_two.perform_expansion(1000000-1);

    println!("Distance combinations (part1):{}", observation_two.distance_combinations());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    }

    #[test]
    fn test_parse_map() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let observation = Observation::parse_map(reader).unwrap();

        assert_eq!(observation.galaxies.len(), 9);
    }

    #[test]
    fn test_expansion() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let mut observation = Observation::parse_map(reader).unwrap();

        observation.perform_expansion(1);

        assert_eq!(observation.print_observation(),
"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
");
    }

    #[test]
    fn test_distance() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let mut observation = Observation::parse_map(reader).unwrap();

        observation.perform_expansion(1);

        assert_eq!(observation.calculate_distance(observation.galaxies[0], observation.galaxies[6]), 15);
        assert_eq!(observation.calculate_distance(observation.galaxies[2], observation.galaxies[5]), 17);
        assert_eq!(observation.calculate_distance(observation.galaxies[7], observation.galaxies[8]), 5);
    }

    #[test]
    fn test_distance_combinations() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let mut observation = Observation::parse_map(reader).unwrap();

        observation.perform_expansion(1);

        assert_eq!(observation.distance_combinations(), 374);
    }

    #[test]
    fn test_distance_combinations_10x() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let mut observation = Observation::parse_map(reader).unwrap();

        observation.perform_expansion(9);

        assert_eq!(observation.distance_combinations(), 1030);
    }

    #[test]
    fn test_distance_combinations_100x() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let mut observation = Observation::parse_map(reader).unwrap();

        observation.perform_expansion(99);

        assert_eq!(observation.distance_combinations(), 8410);
    }
}