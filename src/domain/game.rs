use crate::domain::board::Board;
use crate::domain::card::{Card, HAND_SIZE, Pile};
use crate::domain::game::GameResult::{GameWin, InProgress, PlayerWin};
use crate::domain::player::Player;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    PlayerWin,
    GameWin,
    InProgress,
}

pub struct Game {
    pub player: Player,
    pub board: Board,
    movements_count: u8,
}

impl Game {
    pub fn new(player_name: String) -> Game {
        let mut player = Player::new(player_name);
        let mut board = Board::new();

        let cards = board.deal_hand(8);

        player.add_cards(cards);

        Game { player, board, movements_count:0 }
    }

    pub fn play_card(&mut self, card: u8, pile: usize) -> Result<(), String> {
        self.board.play_card(Card(card), pile)?;
        self.player.play_card(Card(card));
        self.movements_count += 1;
        Ok(())
    }

    pub fn can_finish_turn(&self) -> bool {
        let min_movements = if self.board.missing_cards().len() == 0 {1} else {2};
        self.movements_count >= min_movements
    }
    
    
    pub fn finnish_turn(&mut self) -> GameResult {
        if self.lose_condition() {
            return GameWin;
        }
        
        let cards_needed = HAND_SIZE - self.player.get_cards().len();

        let new_cards = self.board.deal_hand(cards_needed);
        self.player.add_cards(new_cards);

        self.current_status()
    }

    fn lose_condition(&mut self) -> bool {
        !self.can_finish_turn() && !self.board.any_move_available(self.player.get_cards())
    }

    pub fn current_status(&mut self) -> GameResult {
        if self.player.get_cards().len() == 0 && self.board.missing_cards().len() == 0 {
            return PlayerWin;
        }

        if self.lose_condition() {
            return GameWin;
        }

        InProgress
    }

    pub fn show_piles(&self) -> [Pile; 4] {
        self.board.piles()
    }
}
