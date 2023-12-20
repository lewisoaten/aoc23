use std::{io::{BufRead, BufReader}, env, fs::File, collections::HashMap};

struct Platform {
    rows: Vec<String>,
    columns: Vec<String>,
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

impl Platform {
    fn new(rows: Vec<String>) -> Platform {
        let columns = Platform::columns(rows.clone());
        Platform {
            rows: rows,
            columns: columns,
        }
    }

    fn columns(rows: Vec<String>) -> Vec<String> {
        let mut columns = Vec::new();
        for row in rows {
            for (i, c) in row.chars().enumerate() {
                if columns.len() <= i {
                    columns.push("".to_string());
                }
                columns[i].push(c);
            }
        }
        columns
    }

    fn rows(columns: Vec<String>) -> Vec<String> {
        let mut rows = Vec::new();
        for column in columns {
            for (i, c) in column.chars().enumerate() {
                if rows.len() <= i {
                    rows.push("".to_string());
                }
                rows[i].push(c);
            }
        }
        rows
    }

    fn parse_platform<R: BufRead>(reader: R) -> Result<Platform, ParseError> {
        let mut rows = Vec::new();
        for line in reader.lines() {
            let line = line?;

            rows.push(line);
        }

        let platform = Platform::new(rows);

        Ok(platform)
    }

    fn tilt_north<'a>(columns: &'a Vec<String>, cache: &mut Box<HashMap<&'a Vec<String>, (Vec<String>, Vec<String>)>>) -> (Vec<String>, Vec<String>) {
        if let Some(result) = cache.get(&columns) {
            return result.clone();
        }

        // Move all O's in each column as far up the column as possible until they get to another O or #
        let mut new_columns = columns.clone();
        for column in new_columns.iter_mut() {
            let mut o_indexes = Vec::new();
            for (j, c) in column.chars().enumerate() {
                if c == 'O' {
                    o_indexes.push(j);
                }
            }

            for o_index in o_indexes {
                let mut k = o_index;
                while k > 0 {
                    let c = column.chars().nth(k - 1).unwrap();
                    if c == '#' || c == 'O' {
                        break;
                    }
                    column.remove(k);
                    column.insert(k - 1, 'O');
                    k -= 1;
                }
            }
        }

        let result = (Platform::rows(new_columns.clone()), new_columns);
        cache.insert(&columns, result);

        cache.get(&columns).unwrap().clone()
    }

    fn tilt_west<'a>(rows: &'a Vec<String>, cache: &mut Box<HashMap<&'a Vec<String>, (Vec<String>, Vec<String>)>>) -> (Vec<String>, Vec<String>) {
        if let Some(result) = cache.get(&rows) {
            return result.clone();
        }

        // Move all O's in each row as far left the row as possible until they get to another O or #
        let mut new_rows = rows.clone();
        for row in new_rows.iter_mut() {
            let mut o_indexes = Vec::new();
            for (j, c) in row.chars().enumerate() {
                if c == 'O' {
                    o_indexes.push(j);
                }
            }

            for o_index in o_indexes {
                let mut k = o_index;
                while k > 0 {
                    let c = row.chars().nth(k - 1).unwrap();
                    if c == '#' || c == 'O' {
                        break;
                    }
                    row.remove(k);
                    row.insert(k - 1, 'O');
                    k -= 1;
                }
            }
        }

        let new_columns = Platform::columns(new_rows.clone());

        let result = (new_rows, new_columns);
        cache.insert(&rows, result);

        cache.get(&rows).unwrap().clone()
    }

    fn tilt_south<'a>(columns: &'a Vec<String>, cache: &mut Box<HashMap<&'a Vec<String>, (Vec<String>, Vec<String>)>>) -> (Vec<String>, Vec<String>) {
        if let Some(result) = cache.get(&columns) {
            return result.clone();
        }

        // Move all O's in each column as far down the column as possible until they get to another O or #
        let mut new_columns = columns.clone();
        for column in new_columns.iter_mut() {
            let mut o_indexes = Vec::new();
            for (j, c) in column.chars().enumerate() {
                if c == 'O' {
                    o_indexes.push(j);
                }
            }

            for o_index in o_indexes.iter().rev() {
                let mut k = *o_index;
                while k < column.len() - 1 {
                    let c = column.chars().nth(k + 1).unwrap();
                    if c == '#' || c == 'O' {
                        break;
                    }
                    column.remove(k);
                    column.insert(k + 1, 'O');
                    k += 1;
                }
            }
        }

        let result = (Platform::rows(new_columns.clone()), new_columns);
        cache.insert(&columns, result);

        cache.get(&columns).unwrap().clone()
    }

    fn tilt_east<'a>(rows: &'a Vec<String>, cache: &mut Box<HashMap<&'a Vec<String>, (Vec<String>, Vec<String>)>>) -> (Vec<String>, Vec<String>) {
        if let Some(result) = cache.get(&rows) {
            return result.clone();
        }

        // Move all O's in each row as far right the row as possible until they get to another O or #
        let mut new_rows = rows.clone();
        for row in new_rows.iter_mut() {
            let mut o_indexes = Vec::new();
            for (j, c) in row.chars().enumerate() {
                if c == 'O' {
                    o_indexes.push(j);
                }
            }

            for o_index in o_indexes.iter().rev() {
                let mut k = *o_index;
                while k < row.len() - 1 {
                    let c = row.chars().nth(k + 1).unwrap();
                    if c == '#' || c == 'O' {
                        break;
                    }
                    row.remove(k);
                    row.insert(k + 1, 'O');
                    k += 1;
                }
            }
        }

        let new_columns = Platform::columns(new_rows.clone());

        let result = (new_rows, new_columns);
        cache.insert(&rows, result);

        cache.get(&rows).unwrap().clone()
    }

    fn spin_cycle(&mut self) -> bool {
        let previous_rows = self.rows.clone();

        let mut north_cache = Box::new(HashMap::new());
        let mut west_cache = Box::new(HashMap::new());
        let mut south_cache = Box::new(HashMap::new());
        let mut east_cache = Box::new(HashMap::new());
        
        (self.rows, self.columns) = Platform::tilt_north(&self.columns.clone(), &mut north_cache);
        (self.rows, self.columns) = Platform::tilt_west(&self.rows.clone(), &mut west_cache);
        (self.rows, self.columns) = Platform::tilt_south(&self.columns.clone(), &mut south_cache);
        (self.rows, self.columns) = Platform::tilt_east(&self.rows.clone(), &mut east_cache);

        self.rows == previous_rows
    }

    fn spin_n_times(&mut self, n: usize) {
        let mut previous_rows = HashMap::new();
        let mut remaining = 0;

        for i in 0..n {
            self.spin_cycle();
            if let Some(last_hit) = previous_rows.get(&self.rows) {
                remaining = (n-last_hit-1) % (i - last_hit);
                break;
            } else {
                previous_rows.insert(self.rows.clone(), i);
            }
        }

        for _ in 0..remaining {
            self.spin_cycle();
        }
    }

    fn count_north_load(&self) -> usize {
        let mut north_load = 0;
        for (i, row) in self.rows.iter().rev().enumerate() {
            north_load += row.chars().filter(|c| *c == 'O').count() * (i+1);
        }
        north_load
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut platform = Platform::parse_platform(reader).expect("Parsed platform");

    let mut north_cache = Box::new(HashMap::new());
    Platform::tilt_north(&platform.columns.clone(), &mut north_cache);

    println!("North load: {}", platform.count_north_load());

    platform.spin_n_times(1_000_000_000);

    println!("North load after 1b spins: {}", platform.count_north_load());
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    }

    #[test]
    fn test_parse_patterns() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let records = Platform::parse_platform(reader).unwrap();

        assert_eq!(records.rows.len(), 10);
    }

    #[test]
    fn test_tilt_north() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let mut platform = Platform::parse_platform(reader).unwrap();

        assert_eq!(platform.rows.len(), 10);

        let mut north_cache = Box::new(HashMap::new());
        (platform.rows, platform.columns) = Platform::tilt_north(&platform.columns.clone(), &mut north_cache);

        assert_eq!(platform.rows[0], "OOOO.#.O..");
        assert_eq!(platform.rows[1], "OO..#....#");
        assert_eq!(platform.rows[2], "OO..O##..O");
        assert_eq!(platform.rows[3], "O..#.OO...");
        assert_eq!(platform.rows[4], "........#.");
        assert_eq!(platform.rows[5], "..#....#.#");
        assert_eq!(platform.rows[6], "..O..#.O.O");
        assert_eq!(platform.rows[7], "..O.......");
        assert_eq!(platform.rows[8], "#....###..");
        assert_eq!(platform.rows[9], "#....#....");

        assert_eq!(platform.count_north_load(), 136);
    }

    #[test]
    fn test_spin_cycle() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let mut records = Platform::parse_platform(reader).unwrap();

        assert!(!records.spin_cycle());

        assert_eq!(records.rows[0], ".....#....");
        assert_eq!(records.rows[1], "....#...O#");
        assert_eq!(records.rows[2], "...OO##...");
        assert_eq!(records.rows[3], ".OO#......");
        assert_eq!(records.rows[4], ".....OOO#.");
        assert_eq!(records.rows[5], ".O#...O#.#");
        assert_eq!(records.rows[6], "....O#....");
        assert_eq!(records.rows[7], "......OOOO");
        assert_eq!(records.rows[8], "#...O###..");
        assert_eq!(records.rows[9], "#..OO#....");

        assert!(!records.spin_cycle());

        assert_eq!(records.rows[0], ".....#....");
        assert_eq!(records.rows[1], "....#...O#");
        assert_eq!(records.rows[2], ".....##...");
        assert_eq!(records.rows[3], "..O#......");
        assert_eq!(records.rows[4], ".....OOO#.");
        assert_eq!(records.rows[5], ".O#...O#.#");
        assert_eq!(records.rows[6], "....O#...O");
        assert_eq!(records.rows[7], ".......OOO");
        assert_eq!(records.rows[8], "#..OO###..");
        assert_eq!(records.rows[9], "#.OOO#...O");

        assert!(!records.spin_cycle());

        assert_eq!(records.rows[0], ".....#....");
        assert_eq!(records.rows[1], "....#...O#");
        assert_eq!(records.rows[2], ".....##...");
        assert_eq!(records.rows[3], "..O#......");
        assert_eq!(records.rows[4], ".....OOO#.");
        assert_eq!(records.rows[5], ".O#...O#.#");
        assert_eq!(records.rows[6], "....O#...O");
        assert_eq!(records.rows[7], ".......OOO");
        assert_eq!(records.rows[8], "#...O###.O");
        assert_eq!(records.rows[9], "#.OOO#...O");
    }

    #[test]
    fn test_spin_cycle_repeat() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let mut records = Platform::parse_platform(reader).unwrap();

        records.spin_n_times(1_000_000_000);

        assert_eq!(records.count_north_load(), 64);
    }
}