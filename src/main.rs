mod card;
mod game;
mod hand;
mod player;
mod shoe;
mod sprite_sheet;

use card::{Card, rank::Rank, suit::Suit};
use game::Game;
use macroquad::prelude::*;
use sprite_sheet::SpriteSheet;

const PLAYERS: usize = 5;
const DECKS: usize = 8;

const CARD_SPRITES_PATH: &str = "./assets/card_sprites.png";
const CARD_SPRITES_WIDTH: f32 = 3328.0;
const CARD_SPRITES_HEIGHT: f32 = 1424.0;
const CARD_SPRITES_COLS: usize = 13;
const CARD_SPRITES_ROWS: usize = 4;

#[macroquad::main("Blackjack")]
async fn main() {
    let card_sprite_sheet = SpriteSheet::new(
        CARD_SPRITES_PATH,
        CARD_SPRITES_WIDTH,
        CARD_SPRITES_HEIGHT,
        CARD_SPRITES_COLS,
        CARD_SPRITES_ROWS,
        0.25,
    )
    .await
    .unwrap();

    let mut game = Game::new(PLAYERS, DECKS).unwrap();
    game.new_turn();
    game.deal();

    for hand in game.hands() {
        println!("{}", hand[0]);
        println!("{}", hand[1]);
        println!("-------------")
    }

    loop {
        clear_background(BROWN);

        next_frame().await;
    }
}

#[cfg(test)]
mod test {
    use super::card::{Card, Value, rank::Rank, suit::Suit};
    #[test]
    fn test_add_values() {
        let ace = Card::new(Suit::Spade, Rank::Ace);
        let six = Card::new(Suit::Spade, Rank::Six);
        let ten = Card::new(Suit::Spade, Rank::Ten);

        let v1 = ace.value() + six.value();

        assert_eq!(v1, Value::Soft(7, 17));
        assert_eq!(v1 + ace.value(), Value::Soft(8, 18));
        assert_eq!(v1 + ace.value() + ten.value(), Value::Hard(18));
    }
}
