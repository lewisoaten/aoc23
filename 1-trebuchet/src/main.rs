use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();

    println!("Enter calibration document, then press enter:");

    // Read lines from the terminal until an empty line is supplied
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                println!("Error reading line");
                continue;
            }
        };

        if line.is_empty() {
            break;
        }

        lines.push(line);
    }

    // Output the answer on the CLI
    match calculate(lines) {
        Some(answer) => println!("The sum of the calebration values are: {}", answer),
        None => println!("No answer found"),
    }
}

fn calculate(lines: Vec<String>) -> Option<u32> {
    lines
        .iter()
        .map(|line| decode(line.to_string()))
        .reduce(|a, b| a + b)
}

fn decode(line: String) -> u32 {
    // Convert into vector of u8, ignoring all strings that are not numbers
    let numbers: Vec<u8> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u8)
        .collect();

    // Create a number from the first and last digit in the numbers vector
    numbers[0] as u32 * 10 + numbers[numbers.len() - 1] as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let lines = vec![
            String::from("123"),
            String::from("456"),
            String::from("789"),
        ];

        assert_eq!(calculate(lines), Some(138));
    }

    #[test]
    fn test_calculate_official_example() {
        let lines = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];

        assert_eq!(calculate(lines), Some(142));
    }
}