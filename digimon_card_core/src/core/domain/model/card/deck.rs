use crate::core::domain::model::card::Card;

#[derive(Debug, PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn take_out_one_card(self) -> Self {
        let mut remaining_deck: Deck = self;

        remaining_deck.cards.pop();
        remaining_deck
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::card::Card::{AttackPlugin, RecoveryFloppy};
    use crate::core::domain::model::card::deck::Deck;

    #[test]
    fn test_take_out_one_card() {
        let src = Deck {
            cards: vec![
                RecoveryFloppy,
                AttackPlugin,
            ]
        };

        let actual = src.take_out_one_card();

        let expected = Deck {
            cards: vec![
                RecoveryFloppy,
            ]
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_take_out_one_card_no_rest() {
        let src = Deck {
            cards: vec![
            ]
        };

        let actual = src.take_out_one_card();

        let expected = Deck {
            cards: vec![
            ]
        };

        assert_eq!(actual, expected)
    }
}
