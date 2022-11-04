use crate::core::domain::model::card::Card;

#[derive(Debug, PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
    now: Option<Card>,
}

impl Deck {
    pub fn take_out_one_card(mut self) -> Self {
        let card = self.cards.pop();
        Deck {
            cards: self.cards,
            now: card,
        }
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
            ],
            now: Some(RecoveryFloppy),
        };

        let actual = src.take_out_one_card();

        let expected = Deck {
            cards: vec![
                RecoveryFloppy,
            ],
            now: Some(AttackPlugin),
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_take_out_one_card_no_rest() {
        let src = Deck {
            cards: vec![],
            now: Some(AttackPlugin),
        };

        let actual = src.take_out_one_card();

        let expected = Deck {
            cards: vec![],
            now: None,
        };

        assert_eq!(actual, expected)
    }
}
