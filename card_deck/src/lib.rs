use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

fn get_random(min: u8, max: u8) -> u8 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let seed = now.subsec_nanos() as u64;
    min + (seed % (max - min + 1) as u64) as u8
}

impl Suit {
    pub fn random() -> Suit {
        let random_value = get_random(1, 4);
        Suit::translate(random_value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value: {}", value),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random_value = get_random(1, 13);
        Rank::translate(random_value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    matches!((card.suit, card.rank), (Suit::Spade, Rank::Ace))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_suit() {
        assert_eq!(Suit::translate(1), Suit::Heart);
        assert_eq!(Suit::translate(2), Suit::Diamond);
        assert_eq!(Suit::translate(3), Suit::Spade);
        assert_eq!(Suit::translate(4), Suit::Club);
    }

    #[test]
    #[should_panic(expected = "Invalid suit value")]
    fn test_translate_suit_invalid() {
        Suit::translate(0); // should panic
    }

    #[test]
    fn test_translate_rank() {
        assert_eq!(Rank::translate(1), Rank::Ace);
        assert_eq!(Rank::translate(5), Rank::Number(5));
        assert_eq!(Rank::translate(11), Rank::Jack);
        assert_eq!(Rank::translate(12), Rank::Queen);
        assert_eq!(Rank::translate(13), Rank::King);
    }

    #[test]
    #[should_panic(expected = "Invalid rank value")]
    fn test_translate_rank_invalid() {
        Rank::translate(14); // should panic
    }

    #[test]
    fn test_winner_card_true() {
        let card = Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        };
        assert!(winner_card(card));
    }

    #[test]
    fn test_winner_card_false() {
        let card = Card {
            suit: Suit::Heart,
            rank: Rank::Ace,
        };
        assert!(!winner_card(card));
    }

    #[test]
    fn test_random_suit_is_valid() {
        let suit = Suit::random();
        matches!(suit, Suit::Heart | Suit::Diamond | Suit::Spade | Suit::Club);
    }

    #[test]
    fn test_random_rank_is_valid() {
        let rank = Rank::random();
        matches!(
            rank,
            Rank::Ace | Rank::Jack | Rank::Queen | Rank::King | Rank::Number(_)
        );
    }
}
