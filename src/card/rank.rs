use super::Value;

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub const ALL: [Rank; 13] = [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];

    pub fn value(&self) -> Value {
        match self {
            Rank::Ace => Value::Soft(1, 11),
            Rank::Two => Value::Hard(2),
            Rank::Three => Value::Hard(3),
            Rank::Four => Value::Hard(4),
            Rank::Five => Value::Hard(5),
            Rank::Six => Value::Hard(6),
            Rank::Seven => Value::Hard(7),
            Rank::Eight => Value::Hard(8),
            Rank::Nine => Value::Hard(9),
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => Value::Hard(10),
        }
    }

    pub fn weight(&self) -> isize {
        match self {
            Rank::Two => 1,
            Rank::Three => 1,
            Rank::Four => 1,
            Rank::Five => 1,
            Rank::Six => 1,
            Rank::Seven => 0,
            Rank::Eight => 0,
            Rank::Nine => 0,
            Rank::Ten => -1,
            Rank::Jack => -1,
            Rank::Queen => -1,
            Rank::King => -1,
            Rank::Ace => -1,
        }
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Ace => f.write_str("Ace"),
            Rank::Two => f.write_str("Two"),
            Rank::Three => f.write_str("Three"),
            Rank::Four => f.write_str("Four"),
            Rank::Five => f.write_str("Five"),
            Rank::Six => f.write_str("Six"),
            Rank::Seven => f.write_str("Seven"),
            Rank::Eight => f.write_str("Eight"),
            Rank::Nine => f.write_str("Nine"),
            Rank::Ten => f.write_str("Ten"),
            Rank::Jack => f.write_str("Jack"),
            Rank::Queen => f.write_str("Queen"),
            Rank::King => f.write_str("King"),
        }
    }
}
