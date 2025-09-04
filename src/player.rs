use crate::card::Card;
use crate::hand::{combinations, Hand, HandRank};
use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub hole_cards: Vec<Card>,
    pub best_hand: Option<Hand>,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Player {
            id,
            hole_cards: Vec::new(),
            best_hand: None,
        }
    }

    pub fn add_hole_card(&mut self, card: Card) {
        self.hole_cards.push(card);
    }

    pub fn evaluate_hand(&mut self, community_cards: &[Card], wild_cards: &[Card]) {
        let mut all_cards = self.hole_cards.clone();
        all_cards.extend_from_slice(community_cards);

        if all_cards.len() >= 5 {
            let best_5_cards = self.find_best_5_card_combination(&all_cards, wild_cards);
            self.best_hand = Some(Hand::new(best_5_cards, wild_cards));
        }
    }

    fn find_best_5_card_combination(&self, cards: &[Card], wild_cards: &[Card]) -> Vec<Card> {
        if cards.len() == 5 {
            return cards.to_vec();
        }

        let mut best_hand = Vec::new();
        let mut best_rank = HandRank::HighCard;

        for combo in combinations(cards, 5) {
            let hand = Hand::new(combo.clone(), wild_cards);
            if hand.rank > best_rank {
                best_rank = hand.rank;
                best_hand = combo;
            }
        }

        best_hand
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hole_cards_str: Vec<String> = self
            .hole_cards
            .iter()
            .map(|card| card.to_string())
            .collect();
        write!(f, "Player {}: {}", self.id, hole_cards_str.join(" "))?;

        if let Some(ref hand) = self.best_hand {
            write!(f, " -> {}", hand)?;
        }

        Ok(())
    }
}
