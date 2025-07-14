#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Spade => f.write_str("Spades"),
            Suit::Heart => f.write_str("Hearts"),
            Suit::Club => f.write_str("Clubs"),
            Suit::Diamond => f.write_str("Diamonds"),
        }
    }
}
