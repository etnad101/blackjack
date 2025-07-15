mod card;
mod shoe;

use card::{Card, Value, rank::Rank, suit::Suit};
use shoe::Shoe;

// 6 decks more typical for games with sidebets, 8 for straight blackjack

fn main() {
    let decks = 8;
    let cards_to_draw = 52;

    println!("Creating shoe with {} decks", decks);
    let mut shoe = Shoe::new(decks);

    let card1 = Card::new(Suit::Spade, Rank::Ace);
    let card2 = shoe.draw_card().unwrap();
    let card3 = shoe.draw_card().unwrap();

    let hand_value = card1.value() + card2.value();
    let final_value = hand_value + card3.value();

    println!("-----------");
    println!("Your have a {} and {}", card1, card2);
    println!("Total: {}", hand_value);
    println!("your third card is a {}", card3);
    println!("Total: {}", final_value);
    println!("-----------");
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
