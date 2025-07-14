mod card;
mod shoe;

use shoe::Shoe;

// 6 decks more typical for games with sidebets, 8 for straight blackjack

fn main() {
    let shoe = Shoe::new(1);
}
