use std::{io::{BufRead, BufReader}, env, fs::File, collections::HashMap, num::{IntErrorKind, ParseIntError}};

#[derive(Debug, Clone)]
enum Operation {
    Dash,
    Equals,
}

#[derive(Debug, Clone)]
struct Instruction {
    text: String,
    label: String,
    operation: Operation,
    focul_length: Option<u8>,
}

struct Init {
    sequence: Vec<Instruction>,
}

#[derive(Debug)]
enum ParseError {
    IoError(std::io::Error),
    TryFromSliceError(std::array::TryFromSliceError),
    OtherError(&'static str),
    ParseIntError(std::num::ParseIntError),
    FromUtf8Error(std::string::FromUtf8Error),
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

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ParseError::FromUtf8Error(error)
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Instruction) -> bool {
        self.label == other.label
    }

}

impl Instruction {
    fn parse_instruction(text: String) -> Result<Instruction, ParseError> {
        let new_text = text.clone();
        let label_and_operation = text.split_inclusive(['-', '=']).nth(0).expect("No label");
        let focul_length = text.split(['-', '=']).nth(1).expect("No focul_length");

        let label = &label_and_operation[..label_and_operation.len()-1];
        let operation = &label_and_operation[label_and_operation.len()-1..label_and_operation.len()]
            .chars()
            .nth(0)
            .expect("Can't strip off operation");

        let operation = match operation {
            '-' => Operation::Dash,
            '=' => Operation::Equals,
            _ => return Err("Invalid operation".into()),
        };

        let focul_length = match focul_length.parse() {
            Ok(focal_length) => Some(focal_length),
            Err(err) => {
                match (err as ParseIntError).kind() {
                    IntErrorKind::Empty => None,
                    _ => panic!("Invalid focul length")
                }
            },
        };

        Ok(Instruction {
            text: new_text,
            label: label.to_string(),
            operation,
            focul_length,
        })
    }

    fn hash(&self) -> u8 {
        //Determine the ASCII code for the current character of the string.
        //Increase the current value by the ASCII code you just determined.
        //Set the current value to itself multiplied by 17.
        //Set the current value to the remainder of dividing itself by 256.

        let mut hash: u32 = 0;
        for c in self.text.chars() {
            let ascii = c as u32;
            hash += ascii;
            hash *= 17;
            hash %= 256;
        }

        hash as u8
    }

    fn hash_label(&self) -> u8 {
        let mut hash: u32 = 0;
        for c in self.label.chars() {
            let ascii = c as u32;
            hash += ascii;
            hash *= 17;
            hash %= 256;
        }

        hash as u8
    }
}

impl Init {
    fn new(sequence: Vec<Instruction>) -> Self {
        Init { sequence }
    }

    fn parse_init_sequence<R: BufRead>(reader: R) -> Result<Init, ParseError> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in reader.split(b',') {
            let line = line?;

            let instruction_string = String::from_utf8(line)?.trim().to_string();

            instructions.push(Instruction::parse_instruction(instruction_string)?);
        }

        Ok(Init::new(instructions))
    }

    fn sum_hashes(&self) -> u32 {
        let mut sum: u32 = 0;
        for instruction in &self.sequence {
            sum += instruction.hash() as u32;
        }

        sum
    }

    fn initialise(&self) -> HashMap<u8, Vec<Instruction>> {
        let mut lens_boxes: HashMap<u8, Vec<Instruction>> = HashMap::new();

        for instruction in self.sequence.iter() {
            match instruction.operation {
                Operation::Dash => {
                    if let Some(lens_box) = lens_boxes.get_mut(&instruction.hash_label()) {
                        if let Some(index) = lens_box.iter().position(|x| x == instruction) {
                            lens_box.remove(index);
                        }
                    }
                },
                Operation::Equals => {
                    if let Some(lens_box) = lens_boxes.get_mut(&instruction.hash_label()) {
                        if let Some(index) = lens_box.iter().position(|x| x == instruction) {
                                lens_box[index] = (*instruction).clone();
                        } else {
                            lens_box.push((*instruction).clone());
                        }
                    } else {
                        let mut lens_box = Vec::new();
                        lens_box.push((*instruction).clone());
                        lens_boxes.insert(instruction.hash_label(), lens_box);
                    }
                },
            }
        }

        lens_boxes
    }

    fn calculate_focusing_power(lens_boxes: HashMap<u8, Vec<Instruction>>) -> u32 {
        let mut focusing_power: u32 = 0;
        for (hash, lens_box) in lens_boxes.iter() {
            if lens_box.len() >= 1 {
                for (position, instruction) in lens_box.iter().enumerate() {
                    if let Some(focal_length) = instruction.focul_length {
                        focusing_power += (hash.clone() as u32 + 1) * (position as u32 + 1) * focal_length as u32;
                    }
                }
            }
        }

        focusing_power
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let platform = Init::parse_init_sequence(reader).expect("Parsed init sequence");

    println!("Sum of hashes: {}", platform.sum_hashes());

    let lens_boxes = platform.initialise();

    println!("Focusing power: {}", Init::calculate_focusing_power(lens_boxes));
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    #[test]
    fn test_parse_init_sequence() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let init = Init::parse_init_sequence(reader).unwrap();

        assert_eq!(init.sequence.len(), 11);
    }

    #[test]
    fn test_hash() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let init = Init::parse_init_sequence(reader).unwrap();

        assert_eq!(init.sequence[0].hash(), 30);
        assert_eq!(init.sequence[1].hash(), 253);
        assert_eq!(init.sequence[2].hash(), 97);
        assert_eq!(init.sequence[3].hash(), 47);
        assert_eq!(init.sequence[4].hash(), 14);
        assert_eq!(init.sequence[5].hash(), 180);
        assert_eq!(init.sequence[6].hash(), 9);
        assert_eq!(init.sequence[7].hash(), 197);
        assert_eq!(init.sequence[8].hash(), 48);
        assert_eq!(init.sequence[9].hash(), 214);
        assert_eq!(init.sequence[10].hash(), 231);
    }

    #[test]
    fn test_sum_hashes() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let init = Init::parse_init_sequence(reader).unwrap();

        assert_eq!(init.sum_hashes(), 1320);
    }

    #[test]
    fn test_initialise() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let init = Init::parse_init_sequence(reader).unwrap();

        let lens_boxes = init.initialise();

        assert_eq!(lens_boxes.len(), 3);
        assert_eq!(lens_boxes[&0].len(), 2);
        assert_eq!(lens_boxes[&3].len(), 3);
    }

    #[test]
    fn test_calculate_focusing_power() {
        let input = test_data();
        let reader = std::io::Cursor::new(input);
        let init = Init::parse_init_sequence(reader).unwrap();

        let lens_boxes = init.initialise();

        assert_eq!(Init::calculate_focusing_power(lens_boxes), 145);
    }
}