use crate::domain::board::Board;
use crate::domain::card::{Card, HAND_SIZE, Pile};
use crate::domain::game::GameResult::{GameWin, InProgress, PlayerWin};
use crate::domain::player::Player;
use crate::error::GameError;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    PlayerWin,
    GameWin,
    InProgress,
}

#[derive(Debug)]
pub struct Game {
    player: Player,
    board: Board,
    movements_count: u8,
}

impl Game {
    pub fn new(player_name: String) -> Game {
        let mut player = Player::new(player_name);
        let mut board = Board::new();

        let cards = board.deal_hand(8);

        player.add_cards(cards);

        Game {
            player,
            board,
            movements_count: 0,
        }
    }

    pub fn play_card(&mut self, card: u8, pile: usize) -> Result<(), GameError> {
        self.board.play_card(Card::of(card), pile)?;
        self.player.play_card(Card::of(card));
        self.movements_count += 1;
        Ok(())
    }

    pub fn can_finish_turn(&self) -> bool {
        let min_movements = if self.board.missing_cards().len() == 0 {
            1
        } else {
            2
        };
        self.movements_count >= min_movements
    }

    pub fn finnish_turn(&mut self) -> GameResult {
        self.movements_count = 0;
        
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
    
    pub fn player_cards(&self) -> Vec<Card> {
        self.player.get_cards()
    }
    
    pub fn player_name(&self) -> String {
        self.player.get_name()
    }
    
    pub fn remaining_cards(&self) -> Vec<Card> {
        self.board.deck()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_win() {
        let mut game = Game::new("Player".to_string());
        game.play_card(99, 0).unwrap();
        game.play_card(98, 1).unwrap();
        game.play_card(2, 2).unwrap();
        game.play_card(3, 3).unwrap();
        game.finnish_turn();

        
        assert!(!game.can_finish_turn(), "should not be able to finish turn");
        assert!(game.lose_condition(), "should lose condition");
        
        assert_eq!(
            game.current_status(),
            GameWin,
            "Game should be over {:#?}",
            game
        );
    }
}
