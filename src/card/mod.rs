pub mod rank;
pub mod suit;

use std::cmp::min;

use rank::Rank;
use suit::Suit;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Soft(u8, u8),
    Hard(u8),
}

impl Value {
    fn try_harden(self) -> Value {
        match self {
            Value::Soft(a, b) => {
                if a == 21 || b == 21 {
                    return Value::Hard(21);
                }
                if a > 21 && b > 21 {
                    Value::Hard(min(a, b))
                } else if a > 21 {
                    Value::Hard(b)
                } else if b > 21 {
                    Value::Hard(a)
                } else {
                    self
                }
            }
            Value::Hard(_) => self,
        }
    }
}

impl std::ops::Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Hard(a), Value::Hard(b)) => Value::Hard(a + b),
            (Value::Soft(a1, a2), Value::Hard(b)) | (Value::Hard(b), Value::Soft(a1, a2)) => {
                let sum1 = a1 + b;
                let sum2 = a2 + b;
                let val = Value::Soft(sum1, sum2);
                val.try_harden()
            }
            (Value::Soft(a, _), Value::Soft(b1, b2)) => {
                let sum1 = a + b1;
                let sum2 = a + b2;
                let val = Value::Soft(sum1, sum2);
                val.try_harden()
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Hard(a) => f.write_fmt(format_args!("{a}")),
            Value::Soft(a, b) => f.write_fmt(format_args!("{a}/{b}")),
        }
    }
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

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn value(&self) -> Value {
        self.rank.value()
    }

    pub fn weight(&self) -> isize {
        self.rank.weight()
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} of {}", self.rank, self.suit))
    }
}
