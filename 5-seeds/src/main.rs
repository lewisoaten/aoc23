use std::collections::HashMap;
use std::{env, fs::File};
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Debug)]
struct AlmanacMap<S, D> {
    source_start: S,
    destination_start: D,
    range_length: usize,
}

enum AlmanacMapType {
    SeedSoil,
    SoilFertilizer,
    FertilizerWater,
    WaterLight,
    LightTemperature,
    TemperatureHumidity,
    HumidityLocation,
}

struct Almanac {
    seeds: Vec<usize>,
    seed_soil: Vec<AlmanacMap<usize, usize>>,
    soil_fertilizer: Vec<AlmanacMap<usize, usize>>,
    fertilizer_water: Vec<AlmanacMap<usize, usize>>,
    water_light: Vec<AlmanacMap<usize, usize>>,
    light_temperature: Vec<AlmanacMap<usize, usize>>,
    temperature_humidity: Vec<AlmanacMap<usize, usize>>,
    humidity_location: Vec<AlmanacMap<usize, usize>>,
}

impl AlmanacMap<usize, usize> {
    fn get_dest(&self, source: usize) -> Option<usize> {
        if source < self.source_start || source >= self.source_start + self.range_length {
            return None;
        }

        Some((source - self.source_start) + self.destination_start)
    }

    fn get_source(&self, dest: usize) -> Option<usize> {
        if dest < self.destination_start || dest >= self.destination_start + self.range_length {
            return None;
        }

        Some((dest - self.destination_start) + self.source_start)
    }
}

impl Almanac {
    fn traverse_almanac_map(
        &self,
        source: usize,
        almanac_map_type: AlmanacMapType,
    ) -> usize {
        let almanac_map = match almanac_map_type {
            AlmanacMapType::SeedSoil => &self.seed_soil,
            AlmanacMapType::SoilFertilizer => &self.soil_fertilizer,
            AlmanacMapType::FertilizerWater => &self.fertilizer_water,
            AlmanacMapType::WaterLight => &self.water_light,
            AlmanacMapType::LightTemperature => &self.light_temperature,
            AlmanacMapType::TemperatureHumidity => &self.temperature_humidity,
            AlmanacMapType::HumidityLocation => &self.humidity_location,
        };

        for map in almanac_map {
            if let Some(dest) = map.get_dest(source) {
                return dest;
            }
        }

        return source;
    }

    fn from_reader<R: BufRead>(reader: R) -> Result<Self, String> {
        let mut seeds: Vec<usize> = Vec::new();
        let mut seed_soil: Vec<AlmanacMap<usize, usize>> = Vec::new();
        let mut soil_fertilizer: Vec<AlmanacMap<usize, usize>> = Vec::new();
        let mut fertilizer_water: Vec<AlmanacMap<usize, usize>> = Vec::new();
        let mut water_light: Vec<AlmanacMap<usize, usize>> = Vec::new();
        let mut light_temperature: Vec<AlmanacMap<usize, usize>> = Vec::new();
        let mut temperature_humidity: Vec<AlmanacMap<usize, usize>> = Vec::new();
        let mut humidity_location: Vec<AlmanacMap<usize, usize>> = Vec::new();

        let mut lines = reader.lines();

        // Get seeds
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read seeds line");

            if line.is_empty() {
                continue;
            }

            let seeds_string = line.trim_start_matches("seeds:");
            seeds = seeds_string.split_whitespace()
                .map(|seed| seed.parse().map_err(|e| format!("Failed to parse seed: {}", e)))
                .collect::<Result<Vec<_>, _>>()?;

            break;
        }

        // Get seed_soil
        let mut found = false;
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read seed_soil line");

            if line.is_empty() {
                if !found {
                    continue; // Whitespace before
                } else {
                    break; // Whitespace after
                }
            }

            if line.starts_with("seed-to-soil map:") {
                found = true;
                continue;
            }

            //Create AlmanacMap from a line like "50 98 2"
            let mut map_data = line.split_whitespace();
            seed_soil.push(
                AlmanacMap {
                    destination_start: map_data.next().expect("Failed to get soil").trim().parse().expect("Failed to parse soil"),
                    source_start: map_data.next().expect("Failed to get seed").trim().parse().expect("Failed to parse seed"),
                    range_length: map_data.next().expect("Failed to get range length").trim().parse().expect("Failed to parse range length"),
                }
            );
        }

        // Get soil_fertilizer
        let mut found = false;
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read soil_fertilizer line");

            if line.is_empty() {
                if !found {
                    continue; // Whitespace before
                } else {
                    break; // Whitespace after
                }
            }

            if line.starts_with("soil-to-fertilizer map:") {
                found = true;
                continue;
            }

            //Create AlmanacMap from a line like "50 98 2"
            let mut map_data = line.split_whitespace();
            soil_fertilizer.push(
                AlmanacMap {
                    destination_start: map_data.next().expect("Failed to get soil").trim().parse().expect("Failed to parse soil"),
                    source_start: map_data.next().expect("Failed to get seed").trim().parse().expect("Failed to parse seed"),
                    range_length: map_data.next().expect("Failed to get range length").trim().parse().expect("Failed to parse range length"),
                }
            );
        }

        // Get fertilizer_water
        let mut found = false;
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read fertilizer_water line");

            if line.is_empty() {
                if !found {
                    continue; // Whitespace before
                } else {
                    break; // Whitespace after
                }
            }

            if line.starts_with("fertilizer-to-water map:") {
                found = true;
                continue;
            }

            //Create AlmanacMap from a line like "50 98 2"
            let mut map_data = line.split_whitespace();
            fertilizer_water.push(
                AlmanacMap {
                    destination_start: map_data.next().expect("Failed to get soil").trim().parse().expect("Failed to parse soil"),
                    source_start: map_data.next().expect("Failed to get seed").trim().parse().expect("Failed to parse seed"),
                    range_length: map_data.next().expect("Failed to get range length").trim().parse().expect("Failed to parse range length"),
                }
            );
        }

        // Get water_light
        let mut found = false;
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read water_light line");

            if line.is_empty() {
                if !found {
                    continue; // Whitespace before
                } else {
                    break; // Whitespace after
                }
            }

            if line.starts_with("water-to-light map:") {
                found = true;
                continue;
            }

            //Create AlmanacMap from a line like "50 98 2"
            let mut map_data = line.split_whitespace();
            water_light.push(
                AlmanacMap {
                    destination_start: map_data.next().expect("Failed to get soil").trim().parse().expect("Failed to parse soil"),
                    source_start: map_data.next().expect("Failed to get seed").trim().parse().expect("Failed to parse seed"),
                    range_length: map_data.next().expect("Failed to get range length").trim().parse().expect("Failed to parse range length"),
                }
            );
        }

        // Get light_temperature
        let mut found = false;
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read light_temperature line");

            if line.is_empty() {
                if !found {
                    continue; // Whitespace before
                } else {
                    break; // Whitespace after
                }
            }

            if line.starts_with("light-to-temperature map:") {
                found = true;
                continue;
            }

            //Create AlmanacMap from a line like "50 98 2"
            let mut map_data = line.split_whitespace();
            light_temperature.push(
                AlmanacMap {
                    destination_start: map_data.next().expect("Failed to get soil").trim().parse().expect("Failed to parse soil"),
                    source_start: map_data.next().expect("Failed to get seed").trim().parse().expect("Failed to parse seed"),
                    range_length: map_data.next().expect("Failed to get range length").trim().parse().expect("Failed to parse range length"),
                }
            );
        }

        // Get temperature_humidity
        let mut found = false;
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read temperature_humidity line");

            if line.is_empty() {
                if !found {
                    continue; // Whitespace before
                } else {
                    break; // Whitespace after
                }
            }

            if line.starts_with("temperature-to-humidity map:") {
                found = true;
                continue;
            }

            //Create AlmanacMap from a line like "50 98 2"
            let mut map_data = line.split_whitespace();
            temperature_humidity.push(
                AlmanacMap {
                    destination_start: map_data.next().expect("Failed to get soil").trim().parse().expect("Failed to parse soil"),
                    source_start: map_data.next().expect("Failed to get seed").trim().parse().expect("Failed to parse seed"),
                    range_length: map_data.next().expect("Failed to get range length").trim().parse().expect("Failed to parse range length"),
                }
            );
        }



        // Get humidity_location
        let mut found = false;
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read humidity_location line");

            if line.is_empty() {
                if !found {
                    continue; // Whitespace before
                } else {
                    break; // Whitespace after
                }
            }

            if line.starts_with("humidity-to-location map:") {
                found = true;
                continue;
            }

            //Create AlmanacMap from a line like "50 98 2"
            let mut map_data = line.split_whitespace();
            humidity_location.push(
                AlmanacMap {
                    destination_start: map_data.next().expect("Failed to get soil").trim().parse().expect("Failed to parse soil"),
                    source_start: map_data.next().expect("Failed to get seed").trim().parse().expect("Failed to parse seed"),
                    range_length: map_data.next().expect("Failed to get range length").trim().parse().expect("Failed to parse range length"),
                }
            );
        }

        Ok(Almanac {
            seeds,
            seed_soil,
            soil_fertilizer,
            fertilizer_water,
            water_light,
            light_temperature,
            temperature_humidity,
            humidity_location,
        })
    }
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    // let filename = "input/input1.txt";

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let almanac = Almanac::from_reader(reader).expect("Failed to parse almanac");

    let mut locations = Vec::new();

    for seed in almanac.seeds.iter() {
        let seed_soil = almanac.traverse_almanac_map(*seed, AlmanacMapType::SeedSoil);
        let soil_fertilizer = almanac.traverse_almanac_map(seed_soil, AlmanacMapType::SoilFertilizer);
        let fertilizer_water = almanac.traverse_almanac_map(soil_fertilizer, AlmanacMapType::FertilizerWater);
        let water_light = almanac.traverse_almanac_map(fertilizer_water, AlmanacMapType::WaterLight);
        let light_temperature = almanac.traverse_almanac_map(water_light, AlmanacMapType::LightTemperature);
        let temperature_humidity = almanac.traverse_almanac_map(light_temperature, AlmanacMapType::TemperatureHumidity);
        let humidity_location = almanac.traverse_almanac_map(temperature_humidity, AlmanacMapType::HumidityLocation);

        locations.push(humidity_location);
    }

    println!("Minimum location for all seeds in Almanac: {:?}", locations.iter().min());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
"
seeds: 79 14 55

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2

fertilizer-to-water map:
49 53 8
0 11 42

water-to-light map:
88 18 7

light-to-temperature map:
45 77 23

temperature-to-humidity map:
0 69 1

humidity-to-location map:
60 56 37
"
    }

    #[test]
    fn test_from_reader() {
        let input = test_data();
        let reader = input.as_bytes();
        let result = Almanac::from_reader(reader).unwrap();

        assert_eq!(result.seeds, vec![79, 14, 55]);

        let seed_soil = vec![
            AlmanacMap {source_start: 50, destination_start: 98, range_length: 2},
            AlmanacMap {source_start: 52, destination_start: 50, range_length: 48},
            ];
        assert_eq!(result.seed_soil, seed_soil);

        let soil_fertilizer = vec![
            AlmanacMap {source_start: 0, destination_start: 15, range_length: 37},
            AlmanacMap {source_start: 37, destination_start: 52, range_length: 2},
            ];
        assert_eq!(result.soil_fertilizer, soil_fertilizer);

        let fertilizer_water = vec![
            AlmanacMap {source_start: 49, destination_start: 53, range_length: 8},
            AlmanacMap {source_start: 0, destination_start: 11, range_length: 42},
            ];
        assert_eq!(result.fertilizer_water, fertilizer_water);

        let water_light = vec![
            AlmanacMap {source_start: 88, destination_start: 18, range_length: 7},
            ];
        assert_eq!(result.water_light, water_light);

        let light_temperature = vec![
            AlmanacMap {source_start: 45, destination_start: 77, range_length: 23},
            ];
        assert_eq!(result.light_temperature, light_temperature);

        let temperature_humidity = vec![
            AlmanacMap {source_start: 0, destination_start: 69, range_length: 1},
            ];
        assert_eq!(result.temperature_humidity, temperature_humidity);

        let humidity_location = vec![
            AlmanacMap {source_start: 60, destination_start: 56, range_length: 37},
            ];
        assert_eq!(result.humidity_location, humidity_location);
    }

    #[test]
    fn test_get_valid_dest() {
        let almanac_map = AlmanacMap {
            source_start: 10,
            destination_start: 100,
            range_length: 5,
        };

        assert_eq!(almanac_map.get_dest(10), Some(100));
        assert_eq!(almanac_map.get_dest(11), Some(101));
        assert_eq!(almanac_map.get_dest(14), Some(104));
    }

    #[test]
    fn test_get_invalid_dest() {
        let almanac_map = AlmanacMap {
            source_start: 10,
            destination_start: 100,
            range_length: 5,
        };

        assert_eq!(almanac_map.get_dest(9), None);
        assert_eq!(almanac_map.get_dest(15), None);
        assert_eq!(almanac_map.get_dest(20), None);
    }

    #[test]
    fn test_get_valid_source() {
        let almanac_map = AlmanacMap {
            source_start: 10,
            destination_start: 100,
            range_length: 5,
        };

        assert_eq!(almanac_map.get_source(100), Some(10));
        assert_eq!(almanac_map.get_source(101), Some(11));
        assert_eq!(almanac_map.get_source(104), Some(14));
    }

    #[test]
    fn test_get_invalid_source() {
        let almanac_map = AlmanacMap {
            source_start: 10,
            destination_start: 100,
            range_length: 5,
        };

        assert_eq!(almanac_map.get_source(99), None);
        assert_eq!(almanac_map.get_source(105), None);
        assert_eq!(almanac_map.get_source(200), None);
    }

//     #[test]
//     fn test_traverse_almanac_map_single() {
//         let input = 
// "
// seeds: 1

// seed-to-soil map:
// 10 20 5
// ";
//         let reader = input.as_bytes();
//         let almanac = Almanac::from_reader(reader).unwrap();

//         let result = almanac.traverse_almanac_map(vec![10], AlmanacMapType::SeedSoil);
//         assert_eq!(result, vec![20]);

//         let result = almanac.traverse_almanac_map(vec![11], AlmanacMapType::SeedSoil);
//         assert_eq!(result, vec![21]);

//         let result = almanac.traverse_almanac_map(vec![14], AlmanacMapType::SeedSoil);
//         assert_eq!(result, vec![24]);

//         let result = almanac.traverse_almanac_map(vec![15], AlmanacMapType::SeedSoil);
//         assert_eq!(result, vec![]);
//     }

//     #[test]
//     fn test_traverse_almanac_map_multiple() {
//         let input = 
// "
// seeds: 1

// seed-to-soil map:
// 10 20 5
// 10 30 5
// ";
//         let reader = input.as_bytes();
//         let almanac = Almanac::from_reader(reader).unwrap();

//         let result = almanac.traverse_almanac_map(vec![10], AlmanacMapType::SeedSoil);
//         assert_eq!(result, vec![20, 30]);

//         let result = almanac.traverse_almanac_map(vec![11, 12], AlmanacMapType::SeedSoil);
//         assert_eq!(result, vec![21, 22, 31, 32]);

//         let result = almanac.traverse_almanac_map(vec![09, 15], AlmanacMapType::SeedSoil);
//         assert_eq!(result, vec![]);
//     }
}