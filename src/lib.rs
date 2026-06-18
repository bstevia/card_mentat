pub mod card;
pub mod deck;
pub mod hand;
pub mod player;
pub mod rules;
pub mod simulator;

pub use card::{Card, Rank, Suit};
pub use deck::Deck;
pub use hand::{Hand, HandRank};
pub use player::Player;
pub use rules::{GameRules, GameRulesBuilder, RulesError};
pub use simulator::PokerSimulator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluation() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
        ];
        let hand = Hand::new(cards, &[]);
        assert_eq!(hand.rank, HandRank::RoyalFlush);
    }

    #[test]
    fn test_deck_creation() {
        assert_eq!(Deck::new(1).remaining(), 52);
        assert_eq!(Deck::new(2).remaining(), 104);
    }

    #[test]
    fn test_simulator_creation() {
        let simulator = PokerSimulator::new(GameRules::texas_holdem(4));
        assert_eq!(simulator.players.len(), 4);
        assert_eq!(simulator.rules.hole_cards_per_player, 2);
    }

    #[test]
    fn test_rules_builder_and_validation() {
        let rules = GameRules::builder()
            .players(6)
            .hole_cards(2)
            .community_cards(5)
            .decks(2)
            .build()
            .expect("valid rules");
        assert_eq!(rules.deck_size(), 104);
        assert_eq!(rules.cards_needed(), 6 * 2 + 5);

        let err = GameRules::builder()
            .players(30)
            .hole_cards(2)
            .community_cards(5)
            .decks(1)
            .build()
            .unwrap_err();
        assert!(matches!(err, RulesError::NotEnoughCards { .. }));

        let err = GameRules::builder()
            .hole_cards(2)
            .community_cards(0)
            .build()
            .unwrap_err();
        assert_eq!(err, RulesError::HandTooSmall { available: 2 });
    }

    #[test]
    fn test_hole_card_play_constraint() {
        let mut player = Player::new(1);
        player.add_hole_card(Card::new(Rank::King, Suit::Spades));
        player.add_hole_card(Card::new(Rank::King, Suit::Clubs));
        player.add_hole_card(Card::new(Rank::Queen, Suit::Spades));
        player.add_hole_card(Card::new(Rank::Queen, Suit::Clubs));

        let board = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
        ];

        player.evaluate_hand(&board, &[], &(0..=2));
        assert_eq!(player.best_hand.as_ref().unwrap().rank, HandRank::Flush);

        player.evaluate_hand(&board, &[], &(2..=2));
        assert_eq!(player.best_hand.as_ref().unwrap().rank, HandRank::OnePair);
    }

    #[test]
    fn test_hole_play_validation() {
        assert!(GameRules::omaha(6).validate().is_ok());

        assert!(GameRules::builder().hole_cards(4).build().is_ok());

        let err = GameRules::builder()
            .hole_cards(5)
            .community_cards(0)
            .hole_cards_to_play(0..=2)
            .build()
            .unwrap_err();
        assert!(matches!(err, RulesError::HolePlayUnsatisfiable { .. }));

        let err = GameRules::builder()
            .hole_cards_to_play(3..=1)
            .build()
            .unwrap_err();
        assert!(matches!(err, RulesError::EmptyHolePlayRange { .. }));
    }

    #[test]
    fn test_five_of_a_kind_with_multiple_decks() {
        let cards = vec![
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let hand = Hand::new(cards, &[]);
        assert_eq!(hand.rank, HandRank::FiveOfAKind);
    }

    #[test]
    fn test_card_creation() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(card.rank, Rank::Ace);
        assert_eq!(card.suit, Suit::Spades);
    }

    #[test]
    fn test_player_creation() {
        let player = Player::new(1);
        assert_eq!(player.id, 1);
        assert_eq!(player.hole_cards.len(), 0);
        assert!(player.best_hand.is_none());
    }
}
