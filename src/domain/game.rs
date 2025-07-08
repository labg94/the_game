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
}

impl Game {
    pub fn new(player_name: String) -> Game {
        let mut player = Player::new(player_name);
        let mut board = Board::new();

        let cards = board.deal_hand(8);

        player.add_cards(cards);

        Game { player, board }
    }

    pub fn play_card(&mut self, card: u8, pile: usize) -> Result<(), String> {
        self.player.play_card(Card(card));
        self.board.play_card(Card(card), pile)
    }

    pub fn finnish_turn(&mut self) -> GameResult {
        let cards_needed = HAND_SIZE - self.player.get_cards().len();

        let new_cards = self.board.deal_hand(cards_needed);
        self.player.add_cards(new_cards);

        self.current_status()
    }

    pub fn current_status(&mut self) -> GameResult {
        if self.player.get_cards().len() == 0 && self.board.missing_cards().len() == 0 {
            return PlayerWin;
        }

        if !self.board.any_move_available(self.player.get_cards()) {
            return GameWin;
        }

        InProgress
    }

    pub fn show_piles(&self) -> [Pile; 4] {
        self.board.piles()
    }
}
