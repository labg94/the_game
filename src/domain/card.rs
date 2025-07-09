const MIN_CARD: u8 = 2;
const MAX_CARD: u8 = 99;
pub const HAND_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card(pub u8);

#[derive(Debug,Clone)]
pub enum PileDirection {
    Ascending,
    Descending,
}

#[derive(Debug,Clone)]
pub struct Pile {
    pub direction: PileDirection,
    pub top: Card,
}

impl Pile {
    pub fn new_asc() -> Self {
        Self {
            direction: PileDirection::Ascending,
            top: Card(MIN_CARD - 1),
        }
    }

    pub fn new_desc() -> Self {
        Self {
            direction: PileDirection::Descending,
            top: Card(MAX_CARD + 1),
        }
    }

   pub fn can_play_card(&self, card: Card) -> bool {
        match self.direction {
            PileDirection::Ascending => self.top.0 < card.0 || self.top.0 -10 == card.0,
            PileDirection::Descending => self.top.0 > card.0 || self.top.0 + 10 == card.0,
        }
    }

    pub fn play_card(&mut self, card: Card) -> Result<(), String> {
        if !self.can_play_card(card) {
            Err(format!("Cannot play card {} on {:?} pile with top {}", card.0, self.direction, self.top.0))
        } else {
            self.top = card;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asc_start() {
        let pile = Pile::new_asc();
        let top_card_expected = MIN_CARD - 1;
        assert_eq!(
            pile.top.0, top_card_expected,
            "Top card expected: {}",
            top_card_expected
        );
    }

    #[test]
    fn test_desc_start() {
        let pile = Pile::new_desc();
        let top_card_expected = MAX_CARD + 1;
        assert_eq!(
            pile.top.0, top_card_expected,
            "Top card expected: {}",
            top_card_expected
        );
    }

    #[test]
    fn test_can_play_card_desc() {
        let pile = Pile::new_desc();
        assert!(pile.can_play_card(Card(10)), "Couldn't play card 10 on pile desc when top is {}",pile.top.0);
    }

    #[test]
    fn test_can_play_card_asc() {
        let pile = Pile::new_asc();
        assert!(pile.can_play_card(Card(10)), "Cannot play card 10 on pile");
    }

    #[test]
    fn test_play_card_desc() {
        let mut pile = Pile::new_desc();
        let card = Card(10);
        assert!(pile.play_card(card).is_ok(), "Cannot play card 10 on pile");
        assert_eq!(pile.top.0, card.0, "Top card expected: {}", card.0);
    }

    #[test]
    fn test_play_card_asc() {
        let mut pile = Pile::new_asc();
        let card = Card(10);
        assert!(pile.play_card(card).is_ok(), "Cannot play card 10 on pile");
        assert_eq!(pile.top.0, card.0, "Top card expected: {}", card.0);
    }

    #[test]
    fn test_play_card_fail_asc() {
        let mut pile = Pile::new_asc();
        let card = Card(10);
        pile.play_card(card);
        let result = pile.play_card(Card(9));
        assert!(result.is_err(), "Cannot play card 9 after 10 on asc pile");
    }
    
    #[test]
    fn test_play_card_fail_desc() {
        let mut pile = Pile::new_desc();
        let card = Card(9);
        pile.play_card(card);
        let result = pile.play_card(Card(10));
        assert!(result.is_err(), "Cannot play card 10 after 9 on desc pile");   
    }
}
