use crate::domain::card::{Card, Pile};
use rand::seq::SliceRandom;
use rand::rng;

pub struct Board {
    piles: [Pile;4],
    deck: Vec<Card>,
}

impl Board{
    
    pub fn new()-> Self{
        let mut deck = (2..=99).map(Card).collect::<Vec<Card>>();
        deck.shuffle(&mut rng());
        
        Self{
            piles: [Pile::new_asc(), Pile::new_asc(), Pile::new_desc(), Pile::new_desc() ],
            deck,
        }
    }
    
    pub fn deal_hand(&mut self, number_of_cards: usize) -> Vec<Card>{
        let remaining_cards = self.deck.len();
        let cards_to_deal = number_of_cards.min(remaining_cards);

        self.deck.split_off(self.deck.len() - cards_to_deal)

    }
    
    pub fn play_card(&mut self, card: Card, pile_position: usize)-> Result<(), String>{
        self.piles[pile_position].play_card(card)
    }
    
    pub fn any_move_available(&self, cards: Vec<Card>) -> bool{
        self.piles.iter().any(|p| cards.iter().any(|c| p.can_play_card(*c)))
    }
    
    pub fn missing_cards(&self) -> Vec<Card>{
        self.deck.clone()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_new(){
        let board = Board::new();
        assert_eq!(board.piles.len(), 4);
        assert_eq!(board.deck.len(), 98);
    }
    
    #[test]
    fn test_deal_hand(){
        let mut board = Board::new();
        let hand = board.deal_hand(5);
        assert_eq!(hand.len(), 5);
        assert_eq!(board.deck.len(), 93);
    }
    
    
    #[test]
    fn test_not_more_available_moves(){
     
        let mut board = Board::new();
        
        board.play_card(Card(90), 0).expect("Error playing card");
        board.play_card(Card(80), 1).expect("Error playing card");
        board.play_card(Card(10), 2).expect("Error playing card");
        board.play_card(Card(11), 3).expect("Error playing card");


        let cards = vec![Card(20), Card(30), Card(40), Card(50)];
        
        
        assert!(!board.any_move_available(cards));
        
    }
        
    #[test]
    fn test_some_available_moves(){
     
        let mut board = Board::new();
        
        board.play_card(Card(90), 0).expect("Error playing card");
        board.play_card(Card(80), 1).expect("Error playing card");
        board.play_card(Card(10), 2).expect("Error playing card");
        board.play_card(Card(11), 3).expect("Error playing card");


        let cards = vec![Card(2), Card(30), Card(40), Card(50)];
        
        
        assert!(board.any_move_available(cards));
        
    }
    
    
}
