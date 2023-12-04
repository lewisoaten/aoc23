use std::collections::{HashSet, HashMap};
use std::{env, fs::File};
use std::io::{BufRead, BufReader};

const WINNING_SIZE: usize = 10;
const SCRATCH_SIZE: usize = 25;

struct CardStack {
    cards: HashMap<usize, CopiedCard>,
}

struct CopiedCard {
    copies: usize,
    card: Option<Card>,
}

#[derive(Debug, PartialEq)]
struct Card {
    id: usize,
    winning_numbers: [usize; WINNING_SIZE],
    scratch_numbers: [usize; SCRATCH_SIZE],
}

impl CardStack {
    fn new() -> Self {
        CardStack {
            cards: HashMap::new(),
        }
    }

    fn add_card(&mut self, card: Card) {
        let id = card.id;
        let number_of_winning_cards = card.number_of_winning_cards();

        // panic if the CopiedCard already exists with Some card
        if let Some(copied_card) = self.cards.get_mut(&id) {
            if copied_card.card.is_some() {
                panic!("Card already exists");
            }

            copied_card.card = Some(card);
            copied_card.copies += 1;
        } else {
            let card = CopiedCard {
                // Add one to existing
                copies: 1,
                card: Some(card),
            };
            self.cards.insert(id, card);
        }
        let multipler = match self.cards.get(&id) {
            Some(card) => card.copies,
            None => 1,
        };
        self.recieve_copies(id, multipler, number_of_winning_cards);
    }

    fn recieve_copies(&mut self, id: usize, multiplier: usize, new_cards: usize) {
        let iter = id+1..id+new_cards+1;
        for i in iter {
            match self.cards.get_mut(&i) {
                Some(card) => card.copies += multiplier,
                None => {
                    let card = CopiedCard {
                        copies: multiplier,
                        card: None,
                    };
                    self.cards.insert(i, card);
                }
            }
        }
    }

    fn count_copies(&self) -> usize {
        self.cards.values()
            .map(|card| if card.card.is_some() { card.copies } else { 0 })
            .sum()
    }
}

impl TryFrom<String> for Card {
    type Error = String;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let mut parts = line.split_whitespace();
        let id_str = parts
            .nth(1)
            .ok_or_else(|| "Missing card id".to_string())?;

        let mut id_chars = id_str.chars();
        id_chars.next_back(); // Remove the colon

        let id = id_chars.as_str()
            .parse::<usize>()
            .expect("Unable to parse card id");

        let winning_numbers: [usize; WINNING_SIZE] = parts.clone()
            .take(WINNING_SIZE) // Get the WINNING_SIZE winning number parts
            .map(|n| n.parse::<usize>().expect(format!("Unable to parse the winning number: {}", n).as_str()))
            .collect::<Vec<usize>>()
            .try_into()
            .expect("Incorrect number of winning numbers");

        let mut scratch_numbers: [usize; SCRATCH_SIZE] = parts.rev()
            .take(SCRATCH_SIZE) // Get the SCRATCH_SIZE scratch number parts from the end
            .map(|n| n.parse::<usize>().expect(format!("Unable to parse the scratch number: {}", n).as_str()))
            .collect::<Vec<usize>>()
            .try_into()
            .expect("Incorrect number of scratch numbers");

        scratch_numbers.reverse(); // Reverse the scratch numbers to match the order in the string

        Ok(Card { id, winning_numbers, scratch_numbers })
    }
}

impl Card {
    fn number_of_winning_cards(&self) -> usize {
        // Find the intersection of the winning and scratch numbers
        let scratch_set: HashSet<usize> = self.scratch_numbers.iter().cloned().collect();
        let winning_set: HashSet<usize> = self.winning_numbers.iter().cloned().collect();
        let intersection: HashSet<&usize> = scratch_set.intersection(&winning_set).collect();

        intersection.len()
    }

    fn calculate_winnings(&self) -> usize {
        match self.number_of_winning_cards() {
            0 => return 0,
            1 => return 1,
            matches => usize::pow(2, (matches-1).try_into().expect("Can't calculate winnings")),
        }
    }
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut stack = CardStack::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let card = Card::try_from(line).expect("Unable to parse card");
        stack.add_card(card);
    }

    println!("Total number of cards: {}", stack.count_copies());

    // let total_winnings = cards.iter()
    //     .map(|card| card.calculate_winnings())
    //     .sum::<usize>();

    // println!("Result is: {:?}", total_winnings);

}

mod tests {
    use super::*;

    #[test]
    fn test_card_from_line() {
        let line = "Card 1: 1  2  3  4  5  6  7  8  9 10 | 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25".to_string();
        let card = Card::try_from(line).expect("Unable to parse card");
        assert_eq!(card, Card {
            id: 1,
            winning_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        });
    }

    #[test]
    fn test_card_calculate_winnings() {
        let card = Card {
            id: 1,
            winning_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        };
        assert_eq!(card.calculate_winnings(), 512);
    }

    #[test]
    fn test_card_calculate_winnings_scoring() {
        let card = Card {id: 1,
            winning_numbers: [91, 92, 93, 94, 95, 96, 97, 98, 99, 26],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        };
        assert_eq!(card.calculate_winnings(), 0);

        let card = Card {id: 1,
            winning_numbers: [1, 92, 93, 94, 95, 96, 97, 98, 99, 26],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        };
        assert_eq!(card.calculate_winnings(), 1);

        let card = Card {id: 1,
            winning_numbers: [1, 2, 93, 94, 95, 96, 97, 98, 99, 26],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        };
        assert_eq!(card.calculate_winnings(), 2);

        let card = Card {id: 1,
            winning_numbers: [1, 2, 3, 94, 95, 96, 97, 98, 99, 26],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        };
        assert_eq!(card.calculate_winnings(), 4);
    }


    #[test]
    fn test_recieve_copies() {
        let mut stack = CardStack::new();

        stack.add_card(Card {
            id: 1,
            winning_numbers: [1, 2, 3, 94, 95, 96, 97, 98, 99, 26],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        });
        assert_eq!(stack.cards.len(), 4);
        assert_eq!(stack.cards.get(&1).unwrap().copies, 1);
        assert_eq!(stack.cards.get(&2).unwrap().copies, 1);
        assert_eq!(stack.cards.get(&3).unwrap().copies, 1);
        assert_eq!(stack.cards.get(&4).unwrap().copies, 1);

        stack.add_card(Card {
            id: 2,
            winning_numbers: [1, 2, 93, 94, 95, 96, 97, 98, 99, 26],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        });
        assert_eq!(stack.cards.len(), 4);
        assert_eq!(stack.cards.get(&1).unwrap().copies, 1);
        assert_eq!(stack.cards.get(&2).unwrap().copies, 2);
        assert_eq!(stack.cards.get(&3).unwrap().copies, 3);
        assert_eq!(stack.cards.get(&4).unwrap().copies, 3);

        stack.add_card(Card {
            id: 3,
            winning_numbers: [1, 2, 93, 94, 95, 96, 97, 98, 99, 26],
            scratch_numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
        });
        assert_eq!(stack.cards.len(), 5);
        assert_eq!(stack.cards.get(&1).unwrap().copies, 1);
        assert_eq!(stack.cards.get(&2).unwrap().copies, 2);
        assert_eq!(stack.cards.get(&3).unwrap().copies, 4);
        assert_eq!(stack.cards.get(&4).unwrap().copies, 7);
        assert_eq!(stack.cards.get(&5).unwrap().copies, 4);
    }
}