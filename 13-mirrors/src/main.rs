use std::{io::{BufRead, BufReader}, env, fs::File, collections::{HashMap, HashSet}};

struct Pattern {
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

impl Pattern {
    fn new(rows: Vec<String>) -> Pattern {
        let columns = Pattern::columns(rows.clone());
        Pattern {
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

    fn parse_pattern(pattern: String) -> Result<Pattern, ParseError> {
        let mut patterns = Vec::new();
        for line in pattern.lines() {
            patterns.push(line.to_string());
        }

        Ok(Pattern::new(patterns))
    }

    fn parse_all_patterns<R: BufRead>(reader: R) -> Result<Vec<Pattern>, ParseError> {
        let mut maintenance_records = Vec::new();
        let mut lines = "".to_string();
        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                let maintenance_record = Pattern::parse_pattern(lines)?;
                maintenance_records.push(maintenance_record);
                lines = "".to_string();
                continue;
            }
            lines += (line + "\n").as_str();
        }

        let maintenance_record = Pattern::parse_pattern(lines)?;
        maintenance_records.push(maintenance_record);

        Ok(maintenance_records)
    }

    fn vertical_point_of_incidence(&self) -> Option<usize> {
        let mut remaining_reflection_points: HashSet<usize> = (1..self.rows[0].len()).collect();

        for row in self.rows.iter() {
            for point in remaining_reflection_points.clone() {
                let reflection_size = usize::min(point, row.len() - point);
                let reflection_start = (point as i64 - reflection_size as i64).max(0) as usize;
                let reflection_end = usize::min(row.len(), point + reflection_size);
                let left = row[reflection_start..point].to_string();
                let right = row[point..reflection_end].chars().rev().collect::<String>();
                if left != right {
                    remaining_reflection_points.remove(&point);
                }
            }
        }

        if remaining_reflection_points.len() == 1 {
            Some(*remaining_reflection_points.iter().next().unwrap())
        } else {
            None
        }
    }

    fn horizontal_point_of_incidence(&self) -> Option<usize> {
        let mut remaining_reflection_points: HashSet<usize> = (1..self.columns[0].len()).collect();

        for col in self.columns.iter() {
            for point in remaining_reflection_points.clone() {
                let reflection_size = usize::min(point, col.len() - point);
                let reflection_start = (point as i64 - reflection_size as i64).max(0) as usize;
                let reflection_end = usize::min(col.len(), point + reflection_size);
                let left = col[reflection_start..point].to_string();
                let right = col[point..reflection_end].chars().rev().collect::<String>();
                if left != right {
                    remaining_reflection_points.remove(&point);
                }
            }
        }

        if remaining_reflection_points.len() == 1 {
            Some(*remaining_reflection_points.iter().next().unwrap())
        } else {
            None
        }
    }

    fn sum_of_reflection_points(&self) -> usize {
        let mut sum = 0;
        if let Some(point) = self.vertical_point_of_incidence() {
            sum += point;
        }
        if let Some(point) = self.horizontal_point_of_incidence() {
            sum += 100 * point;
        }
        sum
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let patterns = Pattern::parse_all_patterns(reader).expect("Parsed maintenance records");

    let sum_of_reflection_points: usize = patterns.iter().map(|p| p.sum_of_reflection_points()).sum();
    
    println!("Sum of reflection points: {}", sum_of_reflection_points);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    }

    #[test]
    fn test_parse_patterns() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let records = Pattern::parse_all_patterns(reader).unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].rows.len(), 7);
        assert_eq!(records[0].rows.len(), 7);

        assert_eq!(records[0].rows[0], "#.##..##.");
        assert_eq!(records[0].columns[0], "#.##..#");
    }

    #[test]
    fn test_vertical_point_of_incidence() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let records = Pattern::parse_all_patterns(reader).unwrap();

        assert_eq!(records[0].vertical_point_of_incidence(), Some(5));
        assert_eq!(records[1].vertical_point_of_incidence(), None);

        assert_eq!(records[0].horizontal_point_of_incidence(), None);
        assert_eq!(records[1].horizontal_point_of_incidence(), Some(4));

        assert_eq!(records[0].sum_of_reflection_points(), 5);
        assert_eq!(records[1].sum_of_reflection_points(), 4);
    }
}