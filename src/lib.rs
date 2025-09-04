pub mod card;
pub mod deck;
pub mod hand;
pub mod player;
pub mod simulator;

pub use card::{Card, Rank, Suit};
pub use deck::Deck;
pub use hand::{Hand, HandRank};
pub use player::Player;
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
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_simulator_creation() {
        let simulator = PokerSimulator::new(4, 2, vec![]);
        assert_eq!(simulator.players.len(), 4);
        assert_eq!(simulator.hole_cards_per_player, 2);
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
