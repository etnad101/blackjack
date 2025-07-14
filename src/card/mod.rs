pub mod rank;
pub mod suit;

use rank::Rank;
use suit::Suit;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Soft(u8, u8),
    Hard(u8),
}

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}
