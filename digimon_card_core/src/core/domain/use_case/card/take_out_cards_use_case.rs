use crate::core::domain::model::card::Card;
use crate::core::domain::model::card::deck::Deck;

pub struct TakeOutCardsInput {
    count: i32,
    deck: Deck,
}

#[derive(Debug, PartialEq)]
pub struct TakeOutCardsOutput {
    cards: Vec<Card>,
    deck: Deck,
}

pub fn take_out_cards(input: TakeOutCardsInput) -> TakeOutCardsOutput {
    let mut cards = Vec::new();
    let mut deck = input.deck;

    for _ in 0..input.count {
        deck = deck.take_out_one_card();
        match deck.get_first() {
            Some(card) => { cards.push(card) }
            _ => {}
        }
    }

    TakeOutCardsOutput {
        cards,
        deck,
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::card::Card::{AttackPlugin, RecoveryFloppy};
    use crate::core::domain::model::card::deck::Deck;
    use crate::core::domain::use_case::card::take_out_cards_use_case::{take_out_cards, TakeOutCardsInput, TakeOutCardsOutput};

    #[test]
    fn test_take_out_cards() {
        let actual = take_out_cards(TakeOutCardsInput {
            count: 4,
            deck: Deck::new(vec![
                RecoveryFloppy,
                RecoveryFloppy,
                RecoveryFloppy,
                AttackPlugin,
                AttackPlugin,
                AttackPlugin,
            ]),
        });

        let expected = TakeOutCardsOutput {
            cards: vec![
                AttackPlugin,
                AttackPlugin,
                AttackPlugin,
                RecoveryFloppy,
            ],
            deck: Deck {
                cards: vec![
                    RecoveryFloppy,
                    RecoveryFloppy,
                ],
                first: Some(RecoveryFloppy),
            },
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_take_out_cards_few_rest() {
        let actual = take_out_cards(TakeOutCardsInput {
            count: 4,
            deck: Deck::new(vec![
                RecoveryFloppy,
                AttackPlugin,
            ]),
        });

        let expected = TakeOutCardsOutput {
            cards: vec![
                AttackPlugin,
                RecoveryFloppy,
            ],
            deck: Deck {
                cards: vec![],
                first: None,
            },
        };

        assert_eq!(actual, expected)
    }
}