use crate::{
    card::Card,
    hand::Hand,
    player::{Player, Status},
    shoe::Shoe,
};

pub enum Action {
    Hit,
    Stand,
    Split,
    Double,
}

pub struct Game {
    decks: usize,
    player_count: usize,
    shoe: Shoe,
    current_turn: usize,
    players: Vec<Player>,
    dealer_hand: Hand,
}

impl Game {
    pub fn new(player_count: usize, decks: usize) -> Result<Self, String> {
        if player_count == 0 {
            return Err("Must have at least 1 player".to_string());
        }
        if player_count > 5 {
            return Err("Cannot have more than 5 players".to_string());
        }

        let players: Vec<Player> = (0..player_count).map(|_| Player::new(100)).collect();

        Ok(Self {
            shoe: Shoe::new(decks),
            decks,
            current_turn: 0,
            player_count,
            players,
            dealer_hand: Hand::new(),
        })
    }

    pub fn shuffle(&mut self) {
        self.shoe = Shoe::new(self.decks)
    }

    pub fn new_turn(&mut self) {
        self.dealer_hand.clear();
        self.players.iter_mut().for_each(|p| p.clear_hand());
        self.current_turn = 0;
    }

    pub fn deal(&mut self) {
        for _ in 0..2 {
            for i in 0..=self.player_count {
                let card = self.shoe.draw_card().unwrap();

                if i == self.player_count {
                    self.dealer_hand.add_card(card);
                } else {
                    self.players[i].add_card(card);
                }
            }
        }
    }

    fn next_player(&mut self) {
        self.current_turn += 1;
        if self.current_turn > self.player_count {
            self.current_turn = 0;
        }
    }

    fn dealer_turn(&mut self) {}

    fn player_turn(&mut self, action: Action) {
        let player = &mut self.players[self.current_turn];

        match player.status() {
            Status::Waiting | Status::Lose => return,
            _ => {}
        }

        match action {
            Action::Hit => {
                let card = self.shoe.draw_card().unwrap();
                player.add_card(card);
            }
            Action::Stand => {
                player.set_status(Status::Waiting);
            }
            Action::Split => {
                todo!()
            }
            Action::Double => {
                todo!()
            }
        }
    }

    pub fn handle_turn(&mut self, action: Action) {
        if self.current_turn == self.player_count {
            self.dealer_turn();
        } else {
            self.player_turn(action);
        }
    }

    pub fn hands(&self) -> Vec<Hand> {
        let mut hands = Vec::new();
        for player in &self.players {
            hands.push(player.hand().clone())
        }
        hands.push(self.dealer_hand.clone());
        hands
    }
}
