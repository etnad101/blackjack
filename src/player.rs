use crate::{card::Value, hand::Hand};

use super::card::Card;

#[derive(Clone, Copy)]
pub enum Status {
    Playing,
    Waiting,
    Win,
    Lose,
    Push,
}

pub struct Player {
    hand: Hand,
    current_bet: usize,
    balance: usize,
    status: Status,
}

impl Player {
    pub fn new(balance: usize) -> Self {
        Self {
            hand: Hand::new(),
            current_bet: 0,
            balance,
            status: Status::Playing,
        }
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn update_status(&mut self, dealer_value: Value) {
        let dealer_val = dealer_value.raw();
        let val = self.hand_value().raw();
    }

    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    pub fn hand_value(&mut self) -> Value {
        self.hand.value()
    }

    pub fn hand(&self) -> &Hand {
        &self.hand
    }
}
