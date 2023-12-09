use std::{
    env,
    fmt::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord)]
enum Rank {
    Ace = 14,
    King = 13,
    Queen = 12,
    Ten = 11,
    Nine = 10,
    Eight = 9,
    Seven = 8,
    Six = 7,
    Five = 6,
    Four = 5,
    Three = 4,
    Two = 3,
    Jack = 2,
}

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        match value {
            14 => Rank::Ace,
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Ten,
            10 => Rank::Nine,
            9 => Rank::Eight,
            8 => Rank::Seven,
            7 => Rank::Six,
            6 => Rank::Five,
            5 => Rank::Four,
            4 => Rank::Three,
            3 => Rank::Two,
            2 => Rank::Jack,
            _ => panic!("Invalid rank"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Card {
    rank: Rank,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
enum HandType {
    FiveOfAKind(Rank) = 6,
    FourOfAKind(Rank) = 5,
    FullHouse(Rank, Rank) = 4,
    ThreeOfAKind(Rank) = 3,
    TwoPair(Rank, Rank) = 2,
    OnePair(Rank) = 1,
    HighCard(Rank) = 0,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl HandType {
    fn hand(&self) -> HandType {
        match self {
            HandType::FiveOfAKind(_) => HandType::FiveOfAKind(Rank::Ace),
            HandType::FourOfAKind(_) => HandType::FourOfAKind(Rank::Ace),
            HandType::FullHouse(_, _) => HandType::FullHouse(Rank::Ace, Rank::Ace),
            HandType::ThreeOfAKind(_) => HandType::ThreeOfAKind(Rank::Ace),
            HandType::TwoPair(_, _) => HandType::TwoPair(Rank::Ace, Rank::Ace),
            HandType::OnePair(_) => HandType::OnePair(Rank::Ace),
            HandType::HighCard(_) => HandType::HighCard(Rank::Ace),
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        let rank_str = match self.rank {
            Rank::Ace => "A",
            Rank::King => "K",
            Rank::Queen => "Q",
            Rank::Ten => "T",
            Rank::Nine => "9",
            Rank::Eight => "8",
            Rank::Seven => "7",
            Rank::Six => "6",
            Rank::Five => "5",
            Rank::Four => "4",
            Rank::Three => "3",
            Rank::Two => "2",
            Rank::Jack => "J",
        };

        write!(f, "{}", rank_str)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();

        if self_hand_type.hand() == other_hand_type.hand() {
            Some(self.cards.cmp(&other.cards))
        } else {
            Some(self_hand_type.cmp(&other_hand_type))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();

        if self_hand_type.hand() == other_hand_type.hand() {
            self.cards.cmp(&other.cards)
        } else {
            self_hand_type.cmp(&other_hand_type)
        }
    }
}

impl TryFrom<String> for Hand {
    // Given a single-line string like "32T3K 765", parse it into a Hand.
    // The first five characters are the cards, and the rest is the bid.

    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut cards = [Card { rank: Rank::Ace }; 5];
        let mut i = 0;
        for c in value.chars() {
            if i >= 5 {
                break;
            }
            match c {
                'A' => cards[i].rank = Rank::Ace,
                'K' => cards[i].rank = Rank::King,
                'Q' => cards[i].rank = Rank::Queen,
                'T' => cards[i].rank = Rank::Ten,
                '9' => cards[i].rank = Rank::Nine,
                '8' => cards[i].rank = Rank::Eight,
                '7' => cards[i].rank = Rank::Seven,
                '6' => cards[i].rank = Rank::Six,
                '5' => cards[i].rank = Rank::Five,
                '4' => cards[i].rank = Rank::Four,
                '3' => cards[i].rank = Rank::Three,
                '2' => cards[i].rank = Rank::Two,
                'J' => cards[i].rank = Rank::Jack,
                _ => return Err("Invalid card"),
            }
            i += 1;
        }

        let bid_str = &value[6..];

        let bid = bid_str.parse().or(Err("Invalid bid"))?;

        Ok(Hand { cards, bid })
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        
        let mut ranks = [0; 15];

        for card in self.cards.iter() {
            ranks[card.rank as usize] += 1;
        }

        let mut num_pairs = 0;
        let mut num_triples = 0;
        let mut num_quads = 0;
        let mut num_quints = 0;
        let mut high_card_rank: Option<Rank> = None;
        let mut first_pair_rank: Option<Rank> = None;
        let mut second_pair_rank: Option<Rank> = None;
        let mut triple_rank: Option<Rank> = None;
        let mut quad_rank: Option<Rank> = None;
        let mut quint_rank: Option<Rank> = None;

        for (rank, count) in (2..15).zip(ranks[2..15].iter()).rev() {
            match count {
                1 => {
                    if high_card_rank.is_none() {
                        high_card_rank = Some(Rank::from(rank));
                    }
                }
                2 => {
                    num_pairs += 1;
                    if first_pair_rank.is_none() {
                        first_pair_rank = Some(Rank::from(rank));
                    } else {
                        second_pair_rank = Some(Rank::from(rank));
                    }
                }
                3 => {
                    num_triples += 1;
                    triple_rank = Some(Rank::from(rank));
                }
                4 => {
                    num_quads += 1;
                    quad_rank = Some(Rank::from(rank));
                }
                5 => {
                    num_quints += 1;
                    quint_rank = Some(Rank::from(rank));
                }
                _ => (),
            }
        }

        let hand_type = match num_quints {
            1 => HandType::FiveOfAKind(quint_rank.expect("No quint rank")),
            0 => match num_quads {
                1 => HandType::FourOfAKind(quad_rank.expect("No quad rank")),
                0 => match num_triples {
                    1 => match num_pairs {
                        1 => HandType::FullHouse(
                            triple_rank.expect("No triple rank"),
                            first_pair_rank.expect("No first pair rank"),
                        ),
                        0 => HandType::ThreeOfAKind(triple_rank.expect("No triple rank")),
                        _ => panic!("Invalid hand"),
                    },
                    0 => match num_pairs {
                        2 => HandType::TwoPair(
                            first_pair_rank.expect("No first pair rank"),
                            second_pair_rank.expect("No second pair rank"),
                        ),
                        1 => HandType::OnePair(first_pair_rank.expect("No first pair rank")),
                        0 => match high_card_rank {
                            Some(rank) => HandType::HighCard(rank),
                            None => panic!("Invalid hand, no high card: {:?}", self),
                        },
                        _ => panic!("Invalid hand"),
                    },
                    _ => panic!("Invalid hand"),
                },
                _ => panic!("Invalid hand"),
            },
            _ => panic!("Invalid hand"),
        };

        let joker_count = self.cards.iter().filter(|c| c.rank == Rank::Jack).count();

        //Get highest card that is not a Jack
        let mut highest_card = Rank::Two;
        for card in self.cards.iter() {
            if card.rank != Rank::Jack && card.rank > highest_card {
                highest_card = Rank::from(card.rank);
                break;
            }
        }

        if joker_count == 0 {
            return hand_type;
        } else {
            return match hand_type {
                HandType::FiveOfAKind(rank) => HandType::FiveOfAKind(rank),
                HandType::FourOfAKind(rank) => {
                    if rank == Rank::Jack {
                        HandType::FiveOfAKind(highest_card)
                    } else {
                        HandType::FiveOfAKind(rank)
                    }
                },
                HandType::FullHouse(big_rank, small_rank) => {
                    if big_rank == Rank::Jack {
                        HandType::FiveOfAKind(small_rank)
                    } else if small_rank == Rank::Jack {
                        HandType::FiveOfAKind(big_rank)
                    } else {
                        if joker_count >= 2 {
                            HandType::FiveOfAKind(big_rank)
                        } else {
                            HandType::FourOfAKind(big_rank)
                        }
                    }
                },
                HandType::ThreeOfAKind(rank) => {
                    if rank == Rank::Jack {
                        HandType::FourOfAKind(rank)
                    } else {
                        HandType::FourOfAKind(rank)
                    }
                },
                HandType::TwoPair(big_rank, small_rank) => {
                    if big_rank == Rank::Jack {
                        HandType::FourOfAKind(small_rank)
                    } else if small_rank == Rank::Jack {
                        HandType::FourOfAKind(big_rank)
                    } else {
                        HandType::FullHouse(big_rank, small_rank)
                    }
                },
                HandType::OnePair(rank) => {
                    if rank == Rank::Jack {
                        HandType::ThreeOfAKind(highest_card)
                    } else {
                        HandType::ThreeOfAKind(rank)
                    }
                },
                HandType::HighCard(rank) => {
                    if rank == Rank::Jack {
                        HandType::OnePair(highest_card)
                    } else {
                        HandType::OnePair(rank)
                    }
                },
            }
        }
    }
}


#[derive(Debug)]
enum ParseError {
    IoError(std::io::Error),
    OtherError(&'static str),
}

impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError::IoError(error)
    }
}

impl From<&'static str> for ParseError {
    fn from(error: &'static str) -> Self {
        ParseError::OtherError(error)
    }
}

fn parse_hands<R: BufRead>(reader: R) -> Result<Vec<Hand>, ParseError> {
    let mut hands = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let hand = Hand::try_from(line)?;
        hands.push(hand);
    }

    hands.sort();

    Ok(hands)
}

fn main() {
    // Get file name from command line
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide a filename");

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let hands = parse_hands(reader).expect("Can't parse hands");

    let mut rank = 1;

    let mut total_winnings = 0;

    for hand in hands {
        let winnings = hand.bid * rank as u32;

        total_winnings += winnings;

        println!(
            "{} {} {:?} bid: {} winnings: {}",
            rank,
            hand.cards.iter().map(|c| format!("{:?}", c)).collect::<String>(),
            hand.hand_type(),
            hand.bid,
            winnings
        );

        rank += 1;
    }

    println!("Total winnings: {}", total_winnings);
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_try_from_valid_input() {
        let input = String::from("32T3K 765");
        let expected_cards = [
            Card { rank: Rank::Three },
            Card { rank: Rank::Two },
            Card { rank: Rank::Ten },
            Card { rank: Rank::Three },
            Card { rank: Rank::King },
        ];
        let expected_bid = 765;

        let result = Hand::try_from(input);

        assert!(result.is_ok());
        let hand = result.unwrap();
        assert_eq!(hand.cards, expected_cards);
        assert_eq!(hand.bid, expected_bid);
    }

    #[test]
    fn test_try_from_invalid_card() {
        let input = String::from("32T3X 765");

        let result = Hand::try_from(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid card");
    }

    #[test]
    fn test_try_from_invalid_bid() {
        let input = String::from("32T3K abc");

        let result = Hand::try_from(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid bid");
    }

    #[test]
    fn test_parse_hands() {
        let input = "32T3K 765
AKQJT 0
98765 999"
            .to_string();
        let reader = Cursor::new(input);

        let expected_hands = vec![
            Hand {
                cards: [
                    Card { rank: Rank::Nine },
                    Card { rank: Rank::Eight },
                    Card { rank: Rank::Seven },
                    Card { rank: Rank::Six },
                    Card { rank: Rank::Five },
                ],
                bid: 999,
            },
            Hand {
                cards: [
                    Card { rank: Rank::Three },
                    Card { rank: Rank::Two },
                    Card { rank: Rank::Ten },
                    Card { rank: Rank::Three },
                    Card { rank: Rank::King },
                ],
                bid: 765,
            },
            Hand {
                cards: [
                    Card { rank: Rank::Ace },
                    Card { rank: Rank::King },
                    Card { rank: Rank::Queen },
                    Card { rank: Rank::Jack },
                    Card { rank: Rank::Ten },
                ],
                bid: 0,
            },
        ];

        match parse_hands(reader) {
            Ok(hands) => assert_eq!(hands, expected_hands),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_hand_type() {
        match Hand::try_from(String::from("32T3K 765")) {
            Ok(hand) => assert_eq!(hand.hand_type(), HandType::OnePair(Rank::Three)),
            Err(e) => panic!("Error: {:?}", e),
        }

        match Hand::try_from(String::from("T55J5 684")) {
            Ok(hand) => assert_eq!(hand.hand_type(), HandType::FourOfAKind(Rank::Five)),
            Err(e) => panic!("Error: {:?}", e),
        }

        match Hand::try_from(String::from("KK677 28")) {
            Ok(hand) => assert_eq!(hand.hand_type(), HandType::TwoPair(Rank::King, Rank::Seven)),
            Err(e) => panic!("Error: {:?}", e),
        }

        match Hand::try_from(String::from("KTJJT 220")) {
            Ok(hand) => assert_eq!(hand.hand_type(), HandType::FourOfAKind(Rank::Ten)),
            Err(e) => panic!("Error: {:?}", e),
        }

        match Hand::try_from(String::from("QQQJA 483")) {
            Ok(hand) => assert_eq!(hand.hand_type(), HandType::FourOfAKind(Rank::Queen)),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[test]
    fn test_hand_ordering() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();
        let reader = Cursor::new(input);

        let hands = parse_hands(reader).unwrap();

        assert_eq!(hands[0].hand_type(), HandType::OnePair(Rank::Three));
        assert_eq!(
            hands[1].hand_type(),
            HandType::TwoPair(Rank::King, Rank::Seven)
        );
        assert_eq!(
            hands[2].hand_type(),
            HandType::FourOfAKind(Rank::Five)
        );
        assert_eq!(hands[3].hand_type(), HandType::FourOfAKind(Rank::Queen));
        assert_eq!(hands[4].hand_type(), HandType::FourOfAKind(Rank::Ten));
    }

    #[test]
    fn test_hand_high_card_ordering() {
        let input = "74568 1
72654 1
76543 1
65432 1"
            .to_string();
        let reader = Cursor::new(input);

        let hands = parse_hands(reader).unwrap();

        assert_eq!(hands[0].cards.iter().map(|c| format!("{:?}", c)).collect::<String>(), "65432");
        assert_eq!(hands[1].cards.iter().map(|c| format!("{:?}", c)).collect::<String>(), "72654");
        assert_eq!(hands[2].cards.iter().map(|c| format!("{:?}", c)).collect::<String>(), "74568");
        assert_eq!(hands[3].cards.iter().map(|c| format!("{:?}", c)).collect::<String>(), "76543");
    }
}
