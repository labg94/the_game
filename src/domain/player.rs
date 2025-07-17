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
        self.cards.retain(|c| c.value() != card.value())
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn add_cards() {
        let mut player = Player::new("test".to_string());

        let cards = vec![Card::of(1), Card::of(2), Card::of(3)];
        player.add_cards(cards);

        player.play_card(Card::of(1));

        assert_eq!(player.get_cards(), vec![Card::of(2), Card::of(3)]);
    }
}
