mod card;
mod shoe;

use card::{Card, rank::Rank, suit::Suit};
use shoe::Shoe;

fn main() {
    let decks = 8; // 6 decks more typical for games with sidebets, 8 for straight blackjack

    println!("Creating shoe with {} decks", decks);
    let mut shoe = Shoe::new(decks);

    for _ in 0..10 {
        println!("{}", shoe.draw_card().unwrap());
    }

    let mut buff = String::new();
    std::io::stdin()
        .read_line(&mut buff)
        .expect("Failed to read line");

    let guess = buff.trim().parse::<isize>().expect("Failed to parse guess");

    if guess == shoe.count() {
        println!("Good job!");
    } else {
        println!("Oops, the actual count is {}", shoe.count())
    }
}

#[cfg(test)]
mod test {
    use super::card::{Card, Value, rank::Rank, suit::Suit};
    #[test]
    fn test_add_values() {
        let ace = Card::new(Suit::Spade, Rank::Ace);
        let six = Card::new(Suit::Spade, Rank::Six);
        let v1 = ace.value() + six.value();
        assert_eq!(v1, Value::Soft(7, 17));
        assert_eq!(v1 + ace.value(), Value::Soft(8, 18));
    }
}
