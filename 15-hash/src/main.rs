use std::{io::{BufRead, BufReader}, env, fs::File, collections::HashMap};

struct Instruction {
    text: String,
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

impl Instruction {
    fn new(text: String) -> Self {
        Instruction { text }
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
}

impl Init {
    fn new(sequence: Vec<Instruction>) -> Self {
        Init { sequence }
    }

    fn parse_init_sequence<R: BufRead>(reader: R) -> Result<Init, ParseError> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in reader.split(b',') {
            let line = line?;

            instructions.push(Instruction::new(String::from_utf8(line)?.trim().to_string()));
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut platform = Init::parse_init_sequence(reader).expect("Parsed init sequence");

    println!("Sum of hashes: {}", platform.sum_hashes());
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    #[test]
    fn test_parse_patterns() {
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

}