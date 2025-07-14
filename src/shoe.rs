use super::card::{Card, rank::Rank, suit::Suit};

pub struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    pub fn new(decks: usize) -> Self {
        let mut cards = Vec::new();

        for _ in 0..decks {
            for suit in Suit::ALL {
                for rank in Rank::ALL {
                    cards.push(Card::new(suit, rank));
                }
            }
        }

        Shoe { cards }
    }
}
