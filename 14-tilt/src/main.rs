use std::{io::{BufRead, BufReader}, env, fs::File};

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

    fn tilt_north(&mut self) {
        // Move all O's in each column as far up the column as possible until they get to another O or #
        for column in self.columns.iter_mut() {
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

        self.rows = Platform::rows(self.columns.clone());
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

    platform.tilt_north();

    println!("North load: {}", platform.count_north_load());

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
        let mut records = Platform::parse_platform(reader).unwrap();

        assert_eq!(records.rows.len(), 10);

        records.tilt_north();

        assert_eq!(records.rows[0], "OOOO.#.O..");
        assert_eq!(records.rows[1], "OO..#....#");
        assert_eq!(records.rows[2], "OO..O##..O");
        assert_eq!(records.rows[3], "O..#.OO...");
        assert_eq!(records.rows[4], "........#.");
        assert_eq!(records.rows[5], "..#....#.#");
        assert_eq!(records.rows[6], "..O..#.O.O");
        assert_eq!(records.rows[7], "..O.......");
        assert_eq!(records.rows[8], "#....###..");
        assert_eq!(records.rows[9], "#....#....");

        assert_eq!(records.count_north_load(), 136);
    }
}