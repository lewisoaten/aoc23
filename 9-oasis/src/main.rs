use std::{io::{BufRead, BufReader}, env, fs::File};

#[derive(Debug, Eq, PartialEq, Clone)]
struct History {
    data_points: Vec<i64>,
}

impl TryFrom<&str> for History {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let data_points = s
            .split_whitespace()
            .map(|s| s.parse::<i64>().map_err(|_| "Invalid number"))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { data_points })
    }
}

impl History {
    fn new(data_points: Vec<i64>) -> Self {
        Self {
            data_points
        }
    }

    fn extrapolate(&self) -> Option<i64> {
        // Find difference between each data point
        let derived_history = History::new(
            self.data_points
                .clone()
                .iter()
                .zip(self.data_points.iter().skip(1))
                .map(|(&a, &b)| b - a).collect()
        );

        let last_datum = self.data_points.last().copied()?;

        if derived_history.all_zero() {
            return Some(last_datum);
        } else {
            return match derived_history.extrapolate() {
                Some(derived_datapoint) => {
                    return Some(last_datum + derived_datapoint)
                },
                None => {
                    None
                }
            }
        }
    }

    fn extrapolate_backwards(&self) -> Option<i64> {
        // Find difference between each data point
        let derived_history = History::new(
            self.data_points
                .clone()
                .iter()
                .zip(self.data_points.iter().skip(1))
                .map(|(&a, &b)| b - a).collect()
        );

        let first_datum = self.data_points.first().copied()?;

        if derived_history.all_zero() {
            return Some(first_datum);
        } else {
            return match derived_history.extrapolate_backwards() {
                Some(derived_datapoint) => {
                    return Some(first_datum - derived_datapoint)
                },
                None => {
                    None
                }
            }
        }
    }

    fn all_zero(&self) -> bool {
        self.data_points.iter().all(|&n| n == 0)
    }
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

fn parse_histories<R: BufRead>(reader: R) -> Result<Vec<History>, ParseError> {
    let mut histories = Vec::<History>::new();
    for line in reader.lines() {
        let line = line?;

        histories.push(line.as_str().try_into()?);
    }

    Ok(histories)
}

fn sum_extrapolations(histories: Vec<History>) -> i64 {
    histories
        .iter()
        .map(|history| history.extrapolate().expect("No history should be empty"))
        .reduce(|a, i| a+i)
        .expect("Cant accumulate histories")
}

fn sum_backwards_extrapolations(histories: Vec<History>) -> i64 {
    histories
        .iter()
        .map(|history| history.extrapolate_backwards().expect("No history should be empty"))
        .reduce(|a, i| a+i)
        .expect("Cant accumulate backwards histories")
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let histories = parse_histories(reader).expect("Parsed histories");

    let sums = sum_extrapolations(histories.clone());

    println!("Sum of extrapolations: {}", sums);

    let backwards_sums = sum_backwards_extrapolations(histories);

    println!("Sum of backwards extrapolations: {}", backwards_sums);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> String {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".to_string()
    }

    #[test]
    fn test_parse_histories() {
        let test_data = test_data();
        let reader = BufReader::new(test_data.as_bytes());
        let histories = parse_histories(reader).unwrap();

        assert_eq!(histories, vec![
            History::new(vec![0, 3, 6, 9, 12, 15]),
            History::new(vec![1, 3, 6, 10, 15, 21]),
            History::new(vec![10, 13, 16, 21, 30, 45]),
            ]);
    }

    #[test]
    fn test_extrapolate() {
        let history = History::new(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(history.extrapolate(), Some(18));

        let history = History::new(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(history.extrapolate(), Some(28));

        let history = History::new(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(history.extrapolate(), Some(68));
    }

    #[test]
    fn test_sum_extrapolations() {
        let test_data = test_data();
        let reader = BufReader::new(test_data.as_bytes());
        let histories = parse_histories(reader).unwrap();

        assert_eq!(sum_extrapolations(histories), 114)
    }

    #[test]
    fn test_sum_backwards_extrapolations() {
        let test_data = test_data();
        let reader = BufReader::new(test_data.as_bytes());
        let histories = parse_histories(reader).unwrap();

        assert_eq!(sum_backwards_extrapolations(histories), 2)
    }
}