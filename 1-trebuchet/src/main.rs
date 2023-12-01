use std::{io::{self, BufRead}, vec};

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

fn calculate(lines: Vec<String>) -> Option<usize> {
    lines
        .iter()
        .map(|line| decode(line.to_string()))
        .reduce(|a, b| a + b)
}

fn decode(line: String) -> usize {
    // Find position of each number word in the line
    let numbers = vec![
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ];

    // Find leftmost and rightmost word on the line
    let mut lefmost_word = None;
    for number in numbers.clone() {
        if let Some(position) = line.find(&number.0) {
            if let Some((_, current_position)) = lefmost_word {
                if position < current_position {
                    lefmost_word = Some((number, position));
                }
            } else {
                lefmost_word = Some((number, position));
            }
        }

        
    }

    let mut line = line;

    if let Some((number, position)) = lefmost_word {
        // line = line.replacen(&number.0, &number.1.to_string(), 1);
        line.replace_range(position..position+1, &number.1.to_string());
    }

    // Find rightmost word on the line
    let mut rightmost_word = None;
    for number in numbers.clone() {
        if let Some(position) = line.rfind(&number.0) {
            if let Some((_, current_position)) = rightmost_word {
                if position > current_position {
                    rightmost_word = Some((number, position));
                }
            } else {
                rightmost_word = Some((number, position));
            }
        }
    }

    if let Some((number, position)) = rightmost_word {
        line.replace_range(position..position+1, &number.1.to_string());
    }


    // Convert into vector of usize, ignoring all strings that are not numbers
    let numbers: Vec<u8> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u8)
        .collect();

    // Create a number from the first and last digit in the numbers vector
    numbers[0] as usize * 10 + numbers[numbers.len() - 1] as usize
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

    #[test]
    fn test_calculate_official_example_part_two() {
        let lines = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];

        assert_eq!(calculate(lines), Some(281));
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(String::from("one")), 11);
        assert_eq!(decode(String::from("two")), 22);
        assert_eq!(decode(String::from("three")), 33);
        assert_eq!(decode(String::from("four")), 44);
        assert_eq!(decode(String::from("five")), 55);
        assert_eq!(decode(String::from("six")), 66);
        assert_eq!(decode(String::from("seven")), 77);
        assert_eq!(decode(String::from("eight")), 88);
        assert_eq!(decode(String::from("nine")), 99);
        assert_eq!(decode(String::from("123")), 13);
        assert_eq!(decode(String::from("456")), 46);
        assert_eq!(decode(String::from("789")), 79);
        assert_eq!(decode(String::from("1abc2")), 12);
        assert_eq!(decode(String::from("pqr3stu8vwx")), 38);
        assert_eq!(decode(String::from("a1b2c3d4e5f")), 15);
        assert_eq!(decode(String::from("treb7uchet")), 77);
        assert_eq!(decode(String::from("two1nine")), 29);
        assert_eq!(decode(String::from("eightwothree")), 83);
        assert_eq!(decode(String::from("abcone2threexyz")), 13);
        assert_eq!(decode(String::from("xtwone3four")), 24);
        assert_eq!(decode(String::from("4nineeightseven2")), 42);
        assert_eq!(decode(String::from("zoneight234")), 14);
        assert_eq!(decode(String::from("7pqrstsixteen")), 76);
        assert_eq!(decode(String::from("eightwo")), 82);
    }
}