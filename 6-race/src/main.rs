use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind}, num::ParseIntError, ops::Mul,
};

#[derive(Debug)]
struct Race {
    total_time: u64,
    record_distance: u64,
}

impl TryFrom<(&str, &str)> for Race {
    type Error = ParseIntError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let total_time = value.0.parse()?;
        let record_distance = value.1.parse()?;

        Ok(Race {total_time, record_distance})
    }
}

impl Race  {
    fn get_min_winning_press(&self) -> u64 {
        //=CEILING((total_time-SQRT(POW(total_time, 2)-4*record_distance))/2)
        let total_time = self.total_time as f64;
        let record_distance = self.record_distance as f64;

        let winning_press = (total_time - (total_time.powi(2) - 4.0 * record_distance).sqrt()) / 2.0;

        match winning_press.fract() == 0.0 {
            true => winning_press as u64 + 1,
            false => winning_press.ceil() as u64,
        }
    }

    fn get_max_winning_press(&self) -> u64 {
        //=FLOOR((total_time+SQRT(POW(total_time, 2)-4*record_distance))/2)
        let total_time = self.total_time as f64;
        let record_distance = self.record_distance as f64;

        let winning_press = (total_time + (total_time.powi(2) - 4.0 * record_distance).sqrt()) / 2.0;

        match winning_press.fract() == 0.0 {
            true => winning_press as u64 - 1,
            false => winning_press.floor() as u64,
        }
    }

    fn get_num_winning_presses(&self) -> u64 {
        self.get_max_winning_press() - self.get_min_winning_press() + 1
    }
}

fn define_races<R: BufRead>(reader: R) -> Result<Vec<Race>, Error> {
    let mut lines = reader.lines();

    let time_line = lines.next().expect("Failed to read line")?;
    assert!(time_line.starts_with("Time:"));

    let distance_line = lines.next().expect("Failed to read line")?;
    assert!(distance_line.starts_with("Distance:"));

    let time_values = time_line.split_whitespace().skip(1);
    let distance_values = distance_line.split_whitespace().skip(1);

    let race_tuples = time_values.zip(distance_values);

    match race_tuples.map(|r| r.try_into()).collect() {
        Ok(races) => Ok(races),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let races = define_races(reader).expect("Can't parse races");

    let num_winning_presses: Vec<u64> = races.iter().map(|r| r.get_num_winning_presses()).collect();

    let ways_to_beat_record = num_winning_presses.iter().fold(1, Mul::mul);

    println!("Number of winning presses: {:?}", num_winning_presses);
    println!("Ways to beat record: {:?}", ways_to_beat_record);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_races() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let reader = BufReader::new(input.as_bytes());

        let races = define_races(reader).unwrap();

        assert_eq!(races.len(), 3);

        assert_eq!(races[0].total_time, 7);
        assert_eq!(races[0].record_distance, 9);

        assert_eq!(races[1].total_time, 15);
        assert_eq!(races[1].record_distance, 40);

        assert_eq!(races[2].total_time, 30);
        assert_eq!(races[2].record_distance, 200);
    }

    #[test]
    fn test_get_min_winning_press() {
        let race = Race {
            total_time: 7,
            record_distance: 9,
        };
        assert_eq!(race.get_min_winning_press(), 2);

        let race = Race {
            total_time: 15,
            record_distance: 40,
        };
        assert_eq!(race.get_min_winning_press(), 4);

        let race = Race {
            total_time: 30,
            record_distance: 200,
        };
        assert_eq!(race.get_min_winning_press(), 11);
    }

    #[test]
    fn test_get_max_winning_press() {
        let race = Race {
            total_time: 7,
            record_distance: 9,
        };
        assert_eq!(race.get_max_winning_press(), 5);

        let race = Race {
            total_time: 15,
            record_distance: 40,
        };
        assert_eq!(race.get_max_winning_press(), 11);

        let race = Race {
            total_time: 30,
            record_distance: 200,
        };
        assert_eq!(race.get_max_winning_press(), 19);
    }

    #[test]
    fn test_get_num_winning_press() {
        let race = Race {
            total_time: 7,
            record_distance: 9,
        };
        assert_eq!(race.get_num_winning_presses(), 4);

        let race = Race {
            total_time: 15,
            record_distance: 40,
        };
        assert_eq!(race.get_num_winning_presses(), 8);

        let race = Race {
            total_time: 30,
            record_distance: 200,
        };
        assert_eq!(race.get_num_winning_presses(), 9);
    }
}