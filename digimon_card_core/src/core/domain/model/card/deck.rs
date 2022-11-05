use crate::core::domain::model::card::Card;

#[derive(Debug, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub first: Option<Card>,
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        Deck {
            cards,
            first: None,
        }
    }

    pub fn take_out_one_card(mut self) -> Self {
        let card = self.cards.pop();
        Deck {
            cards: self.cards,
            first: card,
        }
    }

    pub fn get_first(&self) -> Option<Card> {
        self.first.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::card::Card::{AttackPlugin, RecoveryFloppy};
    use crate::core::domain::model::card::deck::Deck;

    #[test]
    fn test_new() {
        let actual = Deck::new(vec![
            AttackPlugin,
            RecoveryFloppy,
            AttackPlugin,
        ]);

        let expected = Deck {
            cards: vec![
                AttackPlugin,
                RecoveryFloppy,
                AttackPlugin,
            ],
            first: None,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_take_out_one_card() {
        let src = Deck {
            cards: vec![
                RecoveryFloppy,
                AttackPlugin,
            ],
            first: Some(RecoveryFloppy),
        };

        let actual = src.take_out_one_card();

        let expected = Deck {
            cards: vec![
                RecoveryFloppy,
            ],
            first: Some(AttackPlugin),
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_take_out_one_card_no_rest() {
        let src = Deck {
            cards: vec![],
            first: Some(AttackPlugin),
        };

        let actual = src.take_out_one_card();

        let expected = Deck {
            cards: vec![],
            first: None,
        };

        assert_eq!(actual, expected)
    }
}
