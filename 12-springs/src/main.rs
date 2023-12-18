use std::{io::{BufRead, BufReader}, env, fs::File};

struct MaintenanceRecord {
    springs: Vec<char>,
    damaged_springs: Vec<usize>,
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

impl MaintenanceRecord {
    fn new() -> MaintenanceRecord {
        MaintenanceRecord {
            springs: Vec::new(),
            damaged_springs: Vec::new(),
        }
    }

    fn parse_maintenance_record(line: String) -> Result<MaintenanceRecord, ParseError> {
        let mut maintenance_record = MaintenanceRecord::new();

        let line_parts: Vec<&str> = line.split(' ').collect();
        assert!(line_parts.len() == 2);

        maintenance_record.springs = line_parts[0].chars().collect();

        maintenance_record.damaged_springs = line_parts[1].split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(maintenance_record)
    }

    fn parse_all_maintenance_records<R: BufRead>(reader: R) -> Result<Vec<MaintenanceRecord>, ParseError> {
        let mut maintenance_records = Vec::new();
        for (_, line) in reader.lines().enumerate() {
            let line = line?;
            let maintenance_record = MaintenanceRecord::parse_maintenance_record(line)?;
            maintenance_records.push(maintenance_record);
        }
        Ok(maintenance_records)
    }

    fn possible_failures(&self, lava: &[char], springs: &[usize]) -> usize {
        let mut result = 0;

        if springs.is_empty() {
            return if lava.contains(&'#') { 0 } else { 1 };
        }

        let (current, springs) = (springs[0], &springs[1..]);
        for i in 0..(lava.len() as usize - springs.iter().sum::<usize>() - springs.len() as usize - current + 1) {
            if lava[..(lava.len()).min(i)].contains(&'#') {
                break;
            }

            let next = i + current;
            if next <= lava.len() as usize && !lava[i..(lava.len()).min(next)].contains(&'.') && lava[next..(lava.len()).min(next+1)] != ['#'] {
                result += self.possible_failures(&lava[(lava.len()).min(next + 1)..], springs);
            }
        }

        result
    }

    fn count_possible_failures(&self) -> usize {
        self.possible_failures(&self.springs[..], &self.damaged_springs)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let maintenance_record = MaintenanceRecord::parse_all_maintenance_records(reader).expect("Parsed maintenance records");

    let mut total_possible_failures = 0;
    for record in maintenance_record {
        total_possible_failures += record.count_possible_failures();
    }

    println!("Total possible failures: {}", total_possible_failures);
    
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    }

    #[test]
    fn test_parse_maintenance_records() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let records = MaintenanceRecord::parse_all_maintenance_records(reader).unwrap();

        assert_eq!(records.len(), 6);
        assert_eq!(records[0].springs.len(), 7);
        assert_eq!(records[0].damaged_springs.len(), 3);
    }

    #[test]
    fn test_possible_failures() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let records = MaintenanceRecord::parse_all_maintenance_records(reader).unwrap();

        assert_eq!(records[0].count_possible_failures(), 1);
        assert_eq!(records[1].count_possible_failures(), 4);
        assert_eq!(records[2].count_possible_failures(), 1);
        assert_eq!(records[3].count_possible_failures(), 1);
        assert_eq!(records[4].count_possible_failures(), 4);
        assert_eq!(records[5].count_possible_failures(), 10);
    }
}