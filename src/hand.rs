use crate::card::{Card, Value};

#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl std::ops::Index<usize> for Hand {
    type Output = Card;
    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn value(&mut self) -> Value {
        let mut value = Value::Hard(0);
        for card in &self.cards {
            value = value + card.value();
        }
        value
    }
}
