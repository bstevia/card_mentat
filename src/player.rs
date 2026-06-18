use crate::card::Card;
use crate::hand::{combinations, Hand};
use std::fmt;
use std::ops::RangeInclusive;

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

    pub fn evaluate_hand(
        &mut self,
        community_cards: &[Card],
        wild_cards: &[Card],
        hole_cards_to_play: &RangeInclusive<usize>,
    ) {
        let mut best: Option<Hand> = None;

        // A 5-card hand uses `h` hole cards and `5 - h` board cards.
        let min_hole = *hole_cards_to_play.start();
        let max_hole = (*hole_cards_to_play.end()).min(self.hole_cards.len()).min(5);

        for h in min_hole..=max_hole {
            let board_needed = 5 - h;
            if board_needed > community_cards.len() {
                continue;
            }

            for hole_combo in combinations(&self.hole_cards, h) {
                for board_combo in combinations(community_cards, board_needed) {
                    let mut five = hole_combo.clone();
                    five.extend_from_slice(&board_combo);
                    let hand = Hand::new(five, wild_cards);
                    if best.as_ref().map_or(true, |b| hand.rank > b.rank) {
                        best = Some(hand);
                    }
                }
            }
        }

        self.best_hand = best;
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
