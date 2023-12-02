use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    max_blue: usize,
    max_red: usize,
    max_green: usize,
}

#[derive(Debug)]
struct Games {
    games: Vec<Game>,
}

impl Game {
    fn new(id: usize, max_blue: usize, max_red: usize, max_green: usize) -> Game {
        Game {
            id,
            max_blue,
            max_red,
            max_green,
        }
    }

    fn from_line(line: &str) -> Game {
        // Extract id
        let before_id = line.find(" ").unwrap_or(0);
        let after_id = line.find(":").unwrap_or(line.len());

        let id_str = line[before_id..after_id].trim();

        let id = id_str.parse().expect("Failed to parse id");

        // Extract hands
        let hands = line[after_id+1..].trim().split(";");

        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for hand in hands {
            let colours = hand.split(",");
            for colour in colours {
                let colour = colour.trim();
                let mut parts = colour.split(" ");
                let count = parts.next().expect("Failed to get count").parse::<usize>().expect("Failed to parse count");
                let colour = parts.next().expect("Failed to get colour").trim();
                match colour {
                    "blue" => if count > max_blue { max_blue = count },
                    "red" => if count > max_red { max_red = count },
                    "green" => if count > max_green { max_green = count },
                    _ => panic!("Unknown colour"),
                }
            }
        }

        Game::new(id, max_blue, max_red, max_green)
    }

    fn check_colours(&self, colours: &Vec<(&str, usize)>) -> bool {
        for (colour, count) in colours {
            match colour {
                &"blue" => if &self.max_blue > count  { return false },
                &"red" => if &self.max_red > count  { return false },
                &"green" => if &self.max_green > count { return false },
                _ => panic!("Unknown colour"),
            }
        }

        true
    }

    fn get_game_power(&self) -> usize {
        self.max_blue * self.max_red * self.max_green
    }
}

impl Games {
    fn new() -> Games {
        Games { games: Vec::new() }
    }

    // Create games by parsing file
    fn from_file(file: &str) -> Games {
        let file = File::open(file).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut games = Games::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let game = Game::from_line(&line);
            games.add_game(game);
        }

        games
    }

    fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }

    fn check_colours(&self, colours: &Vec<(&str, usize)>) -> usize {
        match self.games.iter()
            .filter(|game| game.check_colours(colours))
            .map(|game| game.id)
            .reduce(|a, b| a + b) {
            Some(id) => id,
            None => 0,
            }
    }

    fn get_total_power(&self) -> usize {
        self.games.iter()
            .map(|game| game.get_game_power())
            .reduce(|a, b| a + b)
            .unwrap_or(0)
    }
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let games = Games::from_file(filename);
    let result = games.check_colours(&vec![("red", 12), ("green", 13), ("blue", 14)]);
    println!("Result is: {}", result);

    let power = games.get_total_power();
    println!("Total game power is: {}", power);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_games_check_colours() {
        let game1 = Game::new(1, 3, 4, 2);
        let game2 = Game::new(2, 2, 3, 1);
        let game3 = Game::new(3, 1, 2, 3);

        let games = Games {
            games: vec![game1, game2, game3],
        };

        let colours1 = vec![("blue", 2), ("red", 3), ("green", 1)];
        assert_eq!(games.check_colours(&colours1), 2);

        let colours2 = vec![("blue", 1), ("red", 1), ("green", 1)];
        assert_eq!(games.check_colours(&colours2), 0);

        let colours3 = vec![("blue", 4), ("red", 2), ("green", 3)];
        assert_eq!(games.check_colours(&colours3), 3);
    }

    #[test]
    fn test_games_total_power() {
        let game1 = Game::new(1, 6, 4, 2);
        let game2 = Game::new(2, 4, 1, 3);

        let games = Games {
            games: vec![game1, game2],
        };

        assert_eq!(games.get_total_power(), 60);
    }

    #[test]
    fn test_game_from_line() {
        let line1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game1 = Game::new(1, 6, 4, 2);
        assert_eq!(Game::from_line(line1), game1);

        let line2 = "Game 2: 2 blue, 3 red; 1 red, 1 green; 1 green";
        let game2 = Game::new(2, 2, 3, 1);
        assert_eq!(Game::from_line(line2), game2);

        let line3 = "Game 3: 1 blue, 2 red; 3 green";
        let game3 = Game::new(3, 1, 2, 3);
        assert_eq!(Game::from_line(line3), game3);
    }

    #[test]
    fn test_game_power() {
        let game1 = Game::new(1, 6, 4, 2);
        assert_eq!(game1.get_game_power(), 48);

        let game2 = Game::new(2, 4, 1, 3);
        assert_eq!(game2.get_game_power(), 12);
    }
}