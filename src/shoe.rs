use super::card::{Card, rank::Rank, suit::Suit};
use rand::{Rng, rngs::ThreadRng};

pub struct Shoe {
    cards: Vec<Card>,
    decks: usize,
    count: isize,
    rng: ThreadRng,
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

        Shoe {
            cards,
            decks,
            count: 0,
            rng: rand::rng(),
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }
        let idx = self.rng.random_range(0..self.cards.len());
        let card = self.cards.swap_remove(idx);
        self.count += card.weight();
        Some(card)
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn count(&self) -> isize {
        self.count
    }

    pub fn true_count(&self) -> isize {
        self.count / (self.decks as isize)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
