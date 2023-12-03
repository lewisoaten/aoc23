use std::collections::HashSet;
use std::{env, fmt};
use std::{collections::HashMap, fs::File};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Schematic {
    components: IndexedComponentList,
}

type X = usize;
type Y = usize;

#[derive(Clone, Debug)]
struct IndexedComponentList {
    components: Vec<SchematicComponent>,
    coord_index: HashMap<(X, Y), SchematicComponent>,
    type_index: HashMap<ComponentType, Vec<SchematicComponent>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct SchematicComponent {
    x: X,
    y: Y,
    component: ComponentType,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum ComponentType {
    Part(usize),
    Symbol(char),
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentType::Part(part_number) => write!(f, "{}", part_number),
            ComponentType::Symbol(symbol) => write!(f, "{}", symbol),
        }
    }
}

impl Schematic {
    fn possible_symbols() -> [char; 11] {
        ['.', '&', '-', '=', '$', '+', '#', '%', '*', '/', '@']
    }
    fn new(components: Vec<SchematicComponent>) -> Schematic {
        let mut component_list = IndexedComponentList::new();
        component_list.set_component(components);
        Schematic {
            components: component_list,
        }
    }

    // Create schematic by parsing file
    fn from_file<R: BufRead>(reader: R) -> Schematic {
        let mut components = Vec::new();

        let mut y: Y = 0;

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let mut x: X = 0;

            let parsed_line = line.split_inclusive(Schematic::possible_symbols()).flat_map(|e| {
                // split_inclusive leaves the delimiter at the end of the string, unless it's the end of the line
                // If the length is > 1, and the last character is a possible_symbol, split it off
                if e.len() > 1 {
                    if let Some(last_character) = &e.chars().last() {
                        if Schematic::possible_symbols().contains(last_character) {
                            return vec![&e[..e.len()-1], &e[e.len()-1..]].into_iter()
                        }
                    }
                }

                vec![e].into_iter()
                
            });

            
            for component in parsed_line {
                match component.parse::<usize>() {
                    Ok(part_number) => {
                        components.push(SchematicComponent::new(x, y, ComponentType::Part(part_number)));
                    },
                    Err(_) => {
                        if component.len() != 1 {
                            panic!("Unparsable number, but not a single symbol: {}", component);
                        }

                        match component.chars().next() {
                            Some('.') => { },
                            Some(symbol) => {
                                if !Schematic::possible_symbols().contains(&symbol) {
                                    panic!("Unknown symbol type: {}", component);
                                }
                                components.push(SchematicComponent::new(x, y, ComponentType::Symbol(symbol)));
                            },
                            None => {
                                panic!("Unknown component type: {}", component);
                            }
                        }
                    },
                }
                x += component.len();
            }
            y += 1;
        }

        Schematic::new(components)
    }

    fn get_part_numbers(&self) -> HashSet<SchematicComponent> {
        let mut part_numbers = HashSet::new();

        for symbol in Schematic::possible_symbols() {
            for component in self.components.type_index.get(&ComponentType::Symbol(symbol)).into_iter().flatten() {
                part_numbers.extend(self.components.get_adjacent_parts(component.x, component.y));
            }
        }

        part_numbers
    }

    fn get_part_numbers_sum(&self) -> usize {
        self.get_part_numbers().iter().map(|component| {
            match component.component {
                ComponentType::Part(part_number) => part_number,
                _ => panic!("Expected part number"),
            }
        }).sum()
    }
    
}

impl IndexedComponentList {
    fn new() -> IndexedComponentList {
        IndexedComponentList {
            components: Vec::new(),
            coord_index: HashMap::new(),
            type_index: HashMap::new(),
        }
    }

    fn set_component(&mut self, components: Vec<SchematicComponent>) {
        self.components = components;
        self._rebuild_indexes();
    }

    fn get_adjacent_parts(&self, x: X, y: Y) -> HashSet<SchematicComponent> {
        let mut adjacent_parts = HashSet::new();

        let mut positions = vec![
            (x+1, y), // Right
            (x+1, y+1), // Below Right
            (x, y+1), // Below
        ];
        if y > 0 {
            positions.push((x, y-1)); // Above
            positions.push((x+1, y-1)); // Above Right
        }
        if x > 0 {
            positions.push((x-1, y)); // Left
            positions.push((x-1, y+1)); // Below Left
        }
        if x > 0 && y > 0 {
            positions.push((x-1, y-1)); // Above Left
        }        

        for (x, y) in positions {
            if let Some(part) = self.coord_index.get(&(x, y)) {
                adjacent_parts.insert(*part);
            }
        }

        adjacent_parts
    }

    fn _rebuild_indexes(&mut self) {
        self.coord_index.clear();
        self.type_index.clear();
        for (index, component) in self.components.iter().enumerate() {
            for (size, _) in component.component.to_string().chars().enumerate() {
                // Test if a component already exists at this point, if so, panic
                if self.coord_index.contains_key(&(component.x + size, component.y)) {
                    panic!("Component {:?} already exists at {}, {}", component.component, component.x + size, component.y);
                }

                self.coord_index.insert((component.x + size, component.y), self.components[index]);
            }
            self.type_index.entry(component.component.clone()).or_insert(Vec::new()).push(self.components[index]);
        }
    }
}

impl SchematicComponent {
    fn new(x: X, y: Y, component: ComponentType) -> SchematicComponent {
        SchematicComponent {
            x: x,
            y: y,
            component: component,
        }
    }
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let schematic = Schematic::from_file(reader);
    
    println!("Result is: {:?}", schematic.get_part_numbers_sum());
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn test_components() -> Vec<SchematicComponent> {
        vec![
            SchematicComponent::new(0, 0, ComponentType::Part(467)),
            SchematicComponent::new(5, 0, ComponentType::Part(114)),

            SchematicComponent::new(3, 1, ComponentType::Symbol('*')),

            SchematicComponent::new(2, 2, ComponentType::Part(35)),
            SchematicComponent::new(6, 2, ComponentType::Part(633)),

            SchematicComponent::new(6, 3, ComponentType::Symbol('#')),

            SchematicComponent::new(0, 4, ComponentType::Part(617)),
            SchematicComponent::new(3, 4, ComponentType::Symbol('*')),

            SchematicComponent::new(5, 5, ComponentType::Symbol('+')),
            SchematicComponent::new(7, 5, ComponentType::Part(58)),

            SchematicComponent::new(2, 6, ComponentType::Part(592)),

            SchematicComponent::new(6, 7, ComponentType::Part(755)),

            SchematicComponent::new(3, 8, ComponentType::Symbol('$')),
            SchematicComponent::new(5, 8, ComponentType::Symbol('*')),

            SchematicComponent::new(1, 9, ComponentType::Part(664)),
            SchematicComponent::new(5, 9, ComponentType::Part(598)),
        ]
    }

    fn test_string() -> Cursor<Vec<u8>> {
        let input = String::from(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        );
        Cursor::new(input.into_bytes())
    }

    #[test]
    fn test_schematic_from_file() {
        let schematic = Schematic::from_file(test_string());

        // Assert the expected components
        let expected_components = test_components();

        assert_eq!(schematic.components.components, expected_components);
    }

    #[test]
    fn test_get_adjacent_parts() {
        let schematic = Schematic::from_file(test_string());

        let adjacent_parts = schematic.components.get_adjacent_parts(3, 1);
        println!("Adjacent parts: {:?}", adjacent_parts);
        assert!(adjacent_parts.contains(&SchematicComponent::new(0, 0, ComponentType::Part(467))));
        assert!(adjacent_parts.contains(&SchematicComponent::new(2, 2, ComponentType::Part(35))));
        assert_eq!(adjacent_parts.len(), 2);
    }

    #[test]
    fn test_get_part_numbers() {
        let schematic = Schematic::from_file(test_string());

        let part_numbers = schematic.get_part_numbers();
        println!("Part numbers: {:?}", part_numbers);
        assert!(part_numbers.contains(&SchematicComponent::new(0, 0, ComponentType::Part(467))));
        assert!(part_numbers.contains(&SchematicComponent::new(2, 2, ComponentType::Part(35))));
        assert!(part_numbers.contains(&SchematicComponent::new(6, 2, ComponentType::Part(633))));
        assert!(part_numbers.contains(&SchematicComponent::new(0, 4, ComponentType::Part(617))));
        assert!(part_numbers.contains(&SchematicComponent::new(2, 6, ComponentType::Part(592))));
        assert!(part_numbers.contains(&SchematicComponent::new(6, 7, ComponentType::Part(755))));
        assert!(part_numbers.contains(&SchematicComponent::new(1, 9, ComponentType::Part(664))));
        assert!(part_numbers.contains(&SchematicComponent::new(5, 9, ComponentType::Part(598))));
        assert_eq!(part_numbers.len(), 8);
    }

    #[test]
    fn test_get_part_numbers_sum() {
        let schematic = Schematic::from_file(test_string());

        let part_numbers_sum = schematic.get_part_numbers_sum();
        println!("Part numbers sum: {:?}", part_numbers_sum);
        assert_eq!(part_numbers_sum, 4361);
    }

    #[test]
    fn test_get_part_numbers_sum_variations() {
        let tests = vec![
            (".", 0,),
            ("", 0,),
            ("&", 0,),
            ("&.", 0,),
            ("&.1", 0,),
            ("&1", 1,),
            ("-1", 1,),
            ("=1", 1,),
            ("$1", 1,),
            ("+1", 1,),
            ("#1", 1,),
            ("%1", 1,),
            ("*1", 1,),
            ("/1", 1,),
            ("@1", 1,),
            (
"...
...
...", 0,
            ),
            (
"...
.+.
...", 0,
            ),
            (
"1..
.+.
...", 1,
            ),
            (
".1.
.+.
...", 1,
            ),
            (
"..1
.+.
...", 1,
            ),
            (
"...
1+.
...", 1,
            ),
            (
"...
.+1
...", 1,
            ),
            (
"...
.+.
1..", 1,
            ),
            (
"...
.+.
.1.", 1,
            ),
            (
"...
.+.
..1", 1,
            ),
            (
"+.
..", 0,
            ),
            (
"+1
..", 1,
            ),
            (
"+.
1.", 1,
            ),
            (
"+.
.1", 1,
            ),
            (
"..
.+", 0,
            ),
            (
"1.
.+", 1,
            ),
            (
".1
.+", 1,
            ),
            (
"..
1+", 1,
            ),
            ("1+1", 2,),
            ("999+999", 1998,),
        ];

        for (input, expected) in tests {
            let schematic = Schematic::from_file(Cursor::new(String::from(input).into_bytes()));

            let part_numbers_sum = schematic.get_part_numbers_sum();
            assert_eq!(part_numbers_sum, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_get_part_numbers_sum_unknown_symbol() {
        Schematic::from_file(Cursor::new(String::from("?").into_bytes()));
    }
}