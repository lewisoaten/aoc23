use std::cmp::Ordering;
use std::io::{BufRead, BufReader};
use std::{env, fs::File};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct AlmanacMap<S, D> {
    source_start: S,
    destination_start: D,
    range_length: i64,
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
    seeds: Vec<Range>,
    seed_soil: Vec<AlmanacMap<i64, i64>>,
    soil_fertilizer: Vec<AlmanacMap<i64, i64>>,
    fertilizer_water: Vec<AlmanacMap<i64, i64>>,
    water_light: Vec<AlmanacMap<i64, i64>>,
    light_temperature: Vec<AlmanacMap<i64, i64>>,
    temperature_humidity: Vec<AlmanacMap<i64, i64>>,
    humidity_location: Vec<AlmanacMap<i64, i64>>,
}

// impl AlmanacMap<i64, i64> {
//     fn get_dest(&self, source: Range) -> (Option<Vec<Range>>, Option<Vec<Range>>) {
//         // Check if source is out of range
//         if source.end < self.source_start || source.start > self.source_start + self.range_length - 1 {
//             return (None, Some(vec![source]));
//         }

//         let mut source = source;

//         let mut new_ranges = Vec::new();
//         let mut remaining_ranges = Vec::new();

//         let offset = self.destination_start as i64 - self.source_start as i64;

//         // Create new destination range(s)
//         if source.start < self.source_start {
//             // Source range starts before this map
//             remaining_ranges.push(Range {
//                 start: source.start,
//                 end: self.source_start - 1,
//             });
//             source.start = self.source_start;

//             if source.end < self.source_start + self.range_length {
//                 new_ranges.push(Range {
//                     start: (source.start as i64 + offset) as i64,
//                     end: (source.end as i64 + offset) as i64,
//                 });
//             } else {
//                 new_ranges.push(Range {
//                     start: (source.start as i64 + offset) as i64,
//                     end: (self.source_start as i64 + self.range_length as i64 - 1 as i64 + offset) as i64,
//                 });
//             }
//         } else if source.end < self.source_start + self.range_length {
//             // Source entirely within this map
//             new_ranges.push(Range {
//                 start: (source.start as i64 + offset) as i64,
//                 end: (source.end as i64 + offset) as i64,
//             });

//             // No remainder, so different return
//             return (Some(new_ranges), None)
//         } else {
//             // Source range ends after this map
//             new_ranges.push(Range {
//                 start: (source.start as i64 + offset) as i64,
//                 end: (self.source_start as i64 + self.range_length as i64 - 1 as i64 + offset) as i64,
//             });
//             remaining_ranges.push(Range {
//                 start: self.source_start + self.range_length,
//                 end: source.end,
//             });
//         }
//         (Some(new_ranges), Some(remaining_ranges))
//     }
// }

impl Almanac {
    fn traverse_almanac_map(
        &self,
        sources: Vec<Range>,
        almanac_map_type: AlmanacMapType,
    ) -> Vec<Range> {
        let almanac_map = match almanac_map_type {
            AlmanacMapType::SeedSoil => &self.seed_soil,
            AlmanacMapType::SoilFertilizer => &self.soil_fertilizer,
            AlmanacMapType::FertilizerWater => &self.fertilizer_water,
            AlmanacMapType::WaterLight => &self.water_light,
            AlmanacMapType::LightTemperature => &self.light_temperature,
            AlmanacMapType::TemperatureHumidity => &self.temperature_humidity,
            AlmanacMapType::HumidityLocation => &self.humidity_location,
        };

        let mut new_ranges: Vec<Range> = Vec::new();

        // Ran out of time, got working with the wonderful walkthrough here: https://nickymeuleman.netlify.app/garden/aoc2023-day05#part-2
        for range in &sources {
            let mut curr = range.clone();

            for rule in almanac_map {
                let offset = rule.destination_start as i64 - rule.source_start as i64;
                let rule_applies = curr.start <= curr.end
                    && curr.start <= rule.source_start + rule.range_length
                    && curr.end >= rule.source_start;

                if rule_applies {
                    if curr.start < rule.source_start {
                        new_ranges.push(Range {
                            start: curr.start,
                            end: rule.source_start - 1,
                        });
                        curr.start = rule.source_start;
                        if curr.end < rule.source_start + rule.range_length {
                            new_ranges.push(Range {
                                start: (curr.start as i64 + offset) as i64,
                                end: (curr.end as i64 + offset) as i64,
                            });
                            curr.start = curr.end + 1;
                        } else {
                            new_ranges.push(Range {
                                start: (curr.start as i64 + offset) as i64,
                                end: (rule.source_start as i64 + rule.range_length as i64
                                    - 1 as i64
                                    + offset) as i64,
                            });
                            curr.start = rule.source_start + rule.range_length;
                        }
                    } else if curr.end < rule.source_start + rule.range_length {
                        new_ranges.push(Range {
                            start: (curr.start as i64 + offset) as i64,
                            end: (curr.end as i64 + offset) as i64,
                        });
                        curr.start = curr.end + 1;
                    } else {
                        new_ranges.push(Range {
                            start: (curr.start as i64 + offset) as i64,
                            end: (rule.source_start as i64 + rule.range_length as i64 - 1 as i64
                                + offset) as i64,
                        });
                        curr.start = rule.source_start + rule.range_length;
                    }
                }
            }
            if curr.start <= curr.end {
                new_ranges.push(curr);
            }
        }
        new_ranges
    }

    fn from_reader<R: BufRead>(reader: R) -> Result<Self, String> {
        let mut seeds: Vec<Range> = Vec::new();
        let mut seed_soil: Vec<AlmanacMap<i64, i64>> = Vec::new();
        let mut soil_fertilizer: Vec<AlmanacMap<i64, i64>> = Vec::new();
        let mut fertilizer_water: Vec<AlmanacMap<i64, i64>> = Vec::new();
        let mut water_light: Vec<AlmanacMap<i64, i64>> = Vec::new();
        let mut light_temperature: Vec<AlmanacMap<i64, i64>> = Vec::new();
        let mut temperature_humidity: Vec<AlmanacMap<i64, i64>> = Vec::new();
        let mut humidity_location: Vec<AlmanacMap<i64, i64>> = Vec::new();

        let mut lines = reader.lines();

        // Get seeds
        while let Some(line) = lines.next() {
            let line = line.expect("Failed to read seeds line");

            if line.is_empty() {
                continue;
            }

            let seeds_string = line.trim_start_matches("seeds:");
            for seed_pair in seeds_string
                .split_whitespace()
                .map(|seed| {
                    seed.parse()
                        .map_err(|e| format!("Failed to parse seed: {}", e))
                })
                .collect::<Result<Vec<_>, _>>()?
                .chunks(2)
            {
                let start = seed_pair[0];
                let end = seed_pair[0] + seed_pair[1] - 1;
                seeds.push(Range { start, end });
            }

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
            seed_soil.push(AlmanacMap {
                destination_start: map_data
                    .next()
                    .expect("Failed to get soil")
                    .trim()
                    .parse()
                    .expect("Failed to parse soil"),
                source_start: map_data
                    .next()
                    .expect("Failed to get seed")
                    .trim()
                    .parse()
                    .expect("Failed to parse seed"),
                range_length: map_data
                    .next()
                    .expect("Failed to get range length")
                    .trim()
                    .parse()
                    .expect("Failed to parse range length"),
            });
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
            soil_fertilizer.push(AlmanacMap {
                destination_start: map_data
                    .next()
                    .expect("Failed to get soil")
                    .trim()
                    .parse()
                    .expect("Failed to parse soil"),
                source_start: map_data
                    .next()
                    .expect("Failed to get seed")
                    .trim()
                    .parse()
                    .expect("Failed to parse seed"),
                range_length: map_data
                    .next()
                    .expect("Failed to get range length")
                    .trim()
                    .parse()
                    .expect("Failed to parse range length"),
            });
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
            fertilizer_water.push(AlmanacMap {
                destination_start: map_data
                    .next()
                    .expect("Failed to get soil")
                    .trim()
                    .parse()
                    .expect("Failed to parse soil"),
                source_start: map_data
                    .next()
                    .expect("Failed to get seed")
                    .trim()
                    .parse()
                    .expect("Failed to parse seed"),
                range_length: map_data
                    .next()
                    .expect("Failed to get range length")
                    .trim()
                    .parse()
                    .expect("Failed to parse range length"),
            });
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
            water_light.push(AlmanacMap {
                destination_start: map_data
                    .next()
                    .expect("Failed to get soil")
                    .trim()
                    .parse()
                    .expect("Failed to parse soil"),
                source_start: map_data
                    .next()
                    .expect("Failed to get seed")
                    .trim()
                    .parse()
                    .expect("Failed to parse seed"),
                range_length: map_data
                    .next()
                    .expect("Failed to get range length")
                    .trim()
                    .parse()
                    .expect("Failed to parse range length"),
            });
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
            light_temperature.push(AlmanacMap {
                destination_start: map_data
                    .next()
                    .expect("Failed to get soil")
                    .trim()
                    .parse()
                    .expect("Failed to parse soil"),
                source_start: map_data
                    .next()
                    .expect("Failed to get seed")
                    .trim()
                    .parse()
                    .expect("Failed to parse seed"),
                range_length: map_data
                    .next()
                    .expect("Failed to get range length")
                    .trim()
                    .parse()
                    .expect("Failed to parse range length"),
            });
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
            temperature_humidity.push(AlmanacMap {
                destination_start: map_data
                    .next()
                    .expect("Failed to get soil")
                    .trim()
                    .parse()
                    .expect("Failed to parse soil"),
                source_start: map_data
                    .next()
                    .expect("Failed to get seed")
                    .trim()
                    .parse()
                    .expect("Failed to parse seed"),
                range_length: map_data
                    .next()
                    .expect("Failed to get range length")
                    .trim()
                    .parse()
                    .expect("Failed to parse range length"),
            });
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
            humidity_location.push(AlmanacMap {
                destination_start: map_data
                    .next()
                    .expect("Failed to get soil")
                    .trim()
                    .parse()
                    .expect("Failed to parse soil"),
                source_start: map_data
                    .next()
                    .expect("Failed to get seed")
                    .trim()
                    .parse()
                    .expect("Failed to parse seed"),
                range_length: map_data
                    .next()
                    .expect("Failed to get range length")
                    .trim()
                    .parse()
                    .expect("Failed to parse range length"),
            });
        }

        seed_soil.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        soil_fertilizer.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        fertilizer_water.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        water_light.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        light_temperature.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        temperature_humidity.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        humidity_location.sort_by(|a, b| a.source_start.cmp(&b.source_start));

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
    // let args: Vec<String> = env::args().collect();
    // let filename = args.get(1).expect("Please provide a filename");

    let filename = "input/input2.txt";

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let almanac = Almanac::from_reader(reader).expect("Failed to parse almanac");

    // let mut locations = Vec::new();

    let seed_soil = almanac.traverse_almanac_map(almanac.seeds.clone(), AlmanacMapType::SeedSoil);
    let soil_fertilizer = almanac.traverse_almanac_map(seed_soil, AlmanacMapType::SoilFertilizer);
    let fertilizer_water =
        almanac.traverse_almanac_map(soil_fertilizer, AlmanacMapType::FertilizerWater);
    let water_light = almanac.traverse_almanac_map(fertilizer_water, AlmanacMapType::WaterLight);
    let light_temperature =
        almanac.traverse_almanac_map(water_light, AlmanacMapType::LightTemperature);
    let temperature_humidity =
        almanac.traverse_almanac_map(light_temperature, AlmanacMapType::TemperatureHumidity);
    let humidity_location =
        almanac.traverse_almanac_map(temperature_humidity, AlmanacMapType::HumidityLocation);

    // locations.push(humidity_location);

    println!(
        "Minimum location for all seeds in Almanac: {:?}",
        humidity_location
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "
seeds: 79 14 55 2

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

        assert_eq!(
            result.seeds,
            vec![Range { start: 79, end: 92 }, Range { start: 55, end: 56 }]
        );

        let seed_soil = vec![
            AlmanacMap {
                source_start: 98,
                destination_start: 50,
                range_length: 2,
            },
            AlmanacMap {
                source_start: 50,
                destination_start: 52,
                range_length: 48,
            },
        ];
        assert_eq!(result.seed_soil, seed_soil);

        let soil_fertilizer = vec![
            AlmanacMap {
                source_start: 15,
                destination_start: 0,
                range_length: 37,
            },
            AlmanacMap {
                source_start: 52,
                destination_start: 37,
                range_length: 2,
            },
        ];
        assert_eq!(result.soil_fertilizer, soil_fertilizer);

        let fertilizer_water = vec![
            AlmanacMap {
                source_start: 53,
                destination_start: 49,
                range_length: 8,
            },
            AlmanacMap {
                source_start: 11,
                destination_start: 0,
                range_length: 42,
            },
        ];
        assert_eq!(result.fertilizer_water, fertilizer_water);

        let water_light = vec![AlmanacMap {
            source_start: 18,
            destination_start: 88,
            range_length: 7,
        }];
        assert_eq!(result.water_light, water_light);

        let light_temperature = vec![AlmanacMap {
            source_start: 77,
            destination_start: 45,
            range_length: 23,
        }];
        assert_eq!(result.light_temperature, light_temperature);

        let temperature_humidity = vec![AlmanacMap {
            source_start: 69,
            destination_start: 0,
            range_length: 1,
        }];
        assert_eq!(result.temperature_humidity, temperature_humidity);

        let humidity_location = vec![AlmanacMap {
            source_start: 56,
            destination_start: 60,
            range_length: 37,
        }];
        assert_eq!(result.humidity_location, humidity_location);
    }

    // #[test]
    // fn test_get_valid_dest() {
    //     let almanac_map = AlmanacMap {
    //         source_start: 10,
    //         destination_start: 100,
    //         range_length: 5,
    //     };

    //     assert_eq!(almanac_map.get_dest(Range{start:10,end:14}), (Some(vec![Range{start:100,end:104}]), None));
    //     assert_eq!(almanac_map.get_dest(Range{start:11,end:13}), (Some(vec![Range{start:101,end:103}]), None));
    //     assert_eq!(almanac_map.get_dest(Range{start:5,end:9}), (None, Some(vec![Range{start:5,end:9}])));
    //     assert_eq!(almanac_map.get_dest(Range{start:15,end:16}), (None, Some(vec![Range{start:15,end:16}])));
    //     assert_eq!(almanac_map.get_dest(Range{start:5,end:14}), (Some(vec![Range{start:100,end:104}]), Some(vec![Range{start:5,end:9}])));
    //     assert_eq!(almanac_map.get_dest(Range{start:10,end:16}), (Some(vec![Range{start:100,end:104}]), Some(vec![Range{start:15,end:16}])));
    // }

    // #[test]
    // fn test_get_invalid_dest() {
    //     let almanac_map = AlmanacMap {
    //         source_start: 10,
    //         destination_start: 100,
    //         range_length: 5,
    //     };

    //     assert_eq!(almanac_map.get_dest(9), None);
    //     assert_eq!(almanac_map.get_dest(15), None);
    //     assert_eq!(almanac_map.get_dest(20), None);
    // }

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
