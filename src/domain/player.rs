use crate::domain::card::Card;

#[derive(Debug)]
pub struct Player {
    name: String,
    cards: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            cards: Vec::new(),
        }
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards);
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn play_card(&mut self, card: Card) {
        self.cards.retain(|c| c.0 != card.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn add_cards() {
        let mut player = Player::new("test".to_string());

        let cards = vec![Card(1), Card(2), Card(3)];
        player.add_cards(cards);
        
        player.play_card(Card(1));
        
        assert_eq!(player.get_cards(), vec![Card(2), Card(3)]);
        
    }
    
}