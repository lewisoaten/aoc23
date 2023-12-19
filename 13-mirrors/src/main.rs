use std::{io::{BufRead, BufReader}, env, fs::File, collections::HashSet};

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

    fn vertical_point_of_incidence(rows: Vec<String>, exclude_point: Option<usize>) -> Option<usize> {
        let mut remaining_reflection_points: HashSet<usize> = (1..rows[0].len()).collect();

        if let Some(exclude_point) = exclude_point {
            remaining_reflection_points.remove(&exclude_point);
        }

        for row in rows.iter() {
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

    fn horizontal_point_of_incidence(cols: Vec<String>, exclude_point: Option<usize>) -> Option<usize> {
        let mut remaining_reflection_points: HashSet<usize> = (1..cols[0].len()).collect();

        if let Some(exclude_point) = exclude_point {
            remaining_reflection_points.remove(&exclude_point);
        }

        for col in cols.iter() {
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

    fn vertical_point_of_incidence_smudge(rows: Vec<String>) -> Option<usize> {
        let original_vertical_point_of_incidence= Pattern::vertical_point_of_incidence(rows.clone(), None);

        for (row_num, row) in rows.iter().enumerate() {
            for (col_num, col) in row.chars().enumerate() {
                let mut new_rows = rows.clone();
                let new_symbol = if col == '#' { '.' } else { '#' };
                new_rows[row_num].replace_range(col_num..col_num+1, new_symbol.to_string().as_str());
                match Pattern::vertical_point_of_incidence(new_rows.clone(), original_vertical_point_of_incidence) {
                    Some(point) => {
                        if Some(point) != original_vertical_point_of_incidence || original_vertical_point_of_incidence.is_none() {
                            return Some(point);
                        }
                    },
                    None => (),
                
                }
            }
        }
        None
    }

    fn horizontal_point_of_incidence_smudge(cols: Vec<String>) -> Option<usize> {
        let original_vertical_point_of_incidence = Pattern::horizontal_point_of_incidence(cols.clone(), None);

        for (col_num, col) in cols.iter().enumerate() {
            for (row_num, row) in col.chars().enumerate() {
                let mut new_cols = cols.clone();
                let new_symbol = if row == '#' { '.' } else { '#' };
                new_cols[col_num].replace_range(row_num..row_num+1, new_symbol.to_string().as_str());
                match Pattern::horizontal_point_of_incidence(new_cols, original_vertical_point_of_incidence) {
                    Some(point) => {
                        if Some(point) != original_vertical_point_of_incidence || original_vertical_point_of_incidence.is_none() {
                            return Some(point);
                        }
                    },
                    None => (),
                
                }
            }
        }
        None
    }

    fn sum_of_reflection_points(&self) -> usize {
        let mut sum = 0;
        if let Some(point) = Pattern::vertical_point_of_incidence(self.rows.clone(), None) {
            sum += point;
        }
        if let Some(point) = Pattern::horizontal_point_of_incidence(self.columns.clone(), None) {
            sum += 100 * point;
        }

        assert!(sum > 0);
        sum
    }

    fn sum_of_reflection_points_smudge(&self) -> usize {
        let mut sum = 0;
        if let Some(point) = Pattern::vertical_point_of_incidence_smudge(self.rows.clone()) {
            sum += point;
        }
        if let Some(point) = Pattern::horizontal_point_of_incidence_smudge(self.columns.clone()) {
            sum += 100 * point;
        }
        assert!(sum > 0);
        sum
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let filename = args.get(1).expect("Please provide a filename");

    let filename = "input/input2.txt";

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let patterns = Pattern::parse_all_patterns(reader).expect("Parsed maintenance records");

    let sum_of_reflection_points: usize = patterns.iter().map(|p| p.sum_of_reflection_points()).sum();
    
    println!("Sum of reflection points: {}", sum_of_reflection_points);

    let sum_of_reflection_points_smudge: usize = patterns.iter().map(|p| p.sum_of_reflection_points_smudge()).sum();

    println!("Sum of reflection points smudge: {}", sum_of_reflection_points_smudge);
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

        assert_eq!(Pattern::vertical_point_of_incidence(records[0].rows.clone(), None), Some(5));
        assert_eq!(Pattern::vertical_point_of_incidence(records[1].rows.clone(), None), None);

        assert_eq!(Pattern::horizontal_point_of_incidence(records[0].columns.clone(), None), None);
        assert_eq!(Pattern::horizontal_point_of_incidence(records[1].columns.clone(), None), Some(4));

        assert_eq!(records[0].sum_of_reflection_points(), 5);
        assert_eq!(records[1].sum_of_reflection_points(), 400);
    }

    #[test]
    fn test_vertical_point_of_incidence_smudge() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let records = Pattern::parse_all_patterns(reader).unwrap();

        assert_eq!(Pattern::vertical_point_of_incidence_smudge(records[0].rows.clone()), None);
        assert_eq!(Pattern::vertical_point_of_incidence_smudge(records[1].rows.clone()), None);

        assert_eq!(Pattern::horizontal_point_of_incidence_smudge(records[0].columns.clone()), Some(3));
        assert_eq!(Pattern::horizontal_point_of_incidence_smudge(records[1].columns.clone()), Some(1));

        assert_eq!(records[0].sum_of_reflection_points_smudge(), 300);
        assert_eq!(records[1].sum_of_reflection_points_smudge(), 100);
    }

    #[test]
    fn test_vertical_point_of_incidence_smudge_failing1() {
        let input =
"......#
###.#..
###.##.
###.##.
###.#..
.....##
##..#..
##.#...
.###.#.
##.....
..#...#
#....##
#....##
..#...#
##.....";
        let reader = std::io::Cursor::new(input);
        let records = Pattern::parse_all_patterns(reader).unwrap();

        assert_eq!(Pattern::vertical_point_of_incidence(records[0].rows.clone(), None), None);
        assert_eq!(Pattern::horizontal_point_of_incidence(records[0].columns.clone(), None), Some(12));

        assert_eq!(Pattern::vertical_point_of_incidence_smudge(records[0].rows.clone()), None);
        assert_eq!(Pattern::horizontal_point_of_incidence_smudge(records[0].columns.clone()), Some(3));

        assert_eq!(records[0].sum_of_reflection_points(), 1200);
        assert_eq!(records[0].sum_of_reflection_points_smudge(), 300);
    }
}