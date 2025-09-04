use crate::card::{Card, Rank, Suit};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: &'static str = match self {
            HandRank::HighCard => "High Card",
            HandRank::OnePair => "One Pair",
            HandRank::TwoPair => "Two Pair",
            HandRank::ThreeOfAKind => "Three of a Kind",
            HandRank::Straight => "Straight",
            HandRank::Flush => "Flush",
            HandRank::FullHouse => "Full House",
            HandRank::FourOfAKind => "Four of a Kind",
            HandRank::StraightFlush => "Straight Flush",
            HandRank::RoyalFlush => "Royal Flush",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub rank: HandRank,
    pub rank_values: Vec<u8>,
}

impl Hand {
    pub fn new(mut cards: Vec<Card>, wild_cards: &[Card]) -> Self {
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));
        let (rank, rank_values) = Self::evaluate_hand(&cards, wild_cards);
        Hand {
            cards,
            rank,
            rank_values,
        }
    }

    fn evaluate_hand(cards: &[Card], wild_cards: &[Card]) -> (HandRank, Vec<u8>) {
        let mut working_cards = cards.to_vec();
        let mut wild_count = 0;

        // Count and remove wild cards
        working_cards.retain(|card| {
            if wild_cards.contains(card) {
                wild_count += 1;
                false
            } else {
                true
            }
        });

        let best_hand = Self::find_best_hand_with_wilds(&working_cards, wild_count);
        best_hand
    }

    fn find_best_hand_with_wilds(cards: &[Card], wild_count: usize) -> (HandRank, Vec<u8>) {
        if wild_count == 0 {
            return Self::evaluate_standard_hand(cards);
        }

        // Try all possible combinations of wild cards
        let mut best_rank = HandRank::HighCard;
        let mut best_values = vec![];

        let _all_ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        let _all_suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

        // wild cards: todo
        let mut test_cards = cards.to_vec();

        // Add wild cards as the best possible cards for common hands
        for _ in 0..wild_count {
            test_cards.push(Card::new(Rank::Ace, Suit::Spades));
        }

        let (rank, values) = Self::evaluate_standard_hand(&test_cards);
        if rank > best_rank {
            best_rank = rank;
            best_values = values;
        }

        (best_rank, best_values)
    }

    fn evaluate_standard_hand(cards: &[Card]) -> (HandRank, Vec<u8>) {
        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        let is_flush = Self::is_flush(&sorted_cards);
        let is_straight = Self::is_straight(&sorted_cards);
        let rank_counts = Self::get_rank_counts(&sorted_cards);

        if is_straight && is_flush {
            if sorted_cards[0].rank == Rank::Ace && sorted_cards[1].rank == Rank::King {
                return (HandRank::RoyalFlush, vec![14]);
            } else {
                return (HandRank::StraightFlush, vec![sorted_cards[0].rank as u8]);
            }
        }

        let counts: Vec<usize> = rank_counts.values().cloned().collect();
        let mut sorted_counts = counts.clone();
        sorted_counts.sort_by(|a, b| b.cmp(a));

        match sorted_counts.as_slice() {
            [4, 1] => {
                let four_kind = rank_counts
                    .iter()
                    .find(|(_, &count)| count == 4)
                    .map(|(&rank, _)| rank as u8)
                    .unwrap();
                (HandRank::FourOfAKind, vec![four_kind])
            }
            [3, 2] => {
                let three_kind = rank_counts
                    .iter()
                    .find(|(_, &count)| count == 3)
                    .map(|(&rank, _)| rank as u8)
                    .unwrap();
                (HandRank::FullHouse, vec![three_kind])
            }
            _ if is_flush => (HandRank::Flush, vec![sorted_cards[0].rank as u8]),
            _ if is_straight => (HandRank::Straight, vec![sorted_cards[0].rank as u8]),
            [3, 1, 1] => {
                let three_kind = rank_counts
                    .iter()
                    .find(|(_, &count)| count == 3)
                    .map(|(&rank, _)| rank as u8)
                    .unwrap();
                (HandRank::ThreeOfAKind, vec![three_kind])
            }
            [2, 2, 1] => {
                let mut pairs: Vec<u8> = rank_counts
                    .iter()
                    .filter(|(_, &count)| count == 2)
                    .map(|(&rank, _)| rank as u8)
                    .collect();
                pairs.sort_by(|a, b| b.cmp(a));
                (HandRank::TwoPair, pairs)
            }
            [2, 1, 1, 1] => {
                let pair = rank_counts
                    .iter()
                    .find(|(_, &count)| count == 2)
                    .map(|(&rank, _)| rank as u8)
                    .unwrap();
                (HandRank::OnePair, vec![pair])
            }

            //panics if card[0] w no check
            _ => {
                if sorted_cards.is_empty() {
                    (HandRank::HighCard, vec![])
                } else {
                    (HandRank::HighCard, vec![sorted_cards[0].rank as u8])
                }
            }
        }
    }

    fn is_flush(cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }
        let first_suit = cards[0].suit;
        cards.iter().take(5).all(|card| card.suit == first_suit)
    }

    fn is_straight(cards: &[Card]) -> bool {
        if cards.len() < 5 {
            return false;
        }

        let mut ranks: Vec<u8> = cards.iter().map(|card| card.rank as u8).collect();
        ranks.sort_by(|a, b| b.cmp(a));
        ranks.dedup();

        if ranks.len() < 5 {
            return false;
        }

        // Check for regular straight
        for i in 0..=ranks.len() - 5 {
            let mut is_consecutive = true;
            for j in 1..5 {
                if ranks[i + j] != ranks[i + j - 1] - 1 {
                    is_consecutive = false;
                    break;
                }
            }
            if is_consecutive {
                return true;
            }
        }

        // Check for wheel straight (A, 2, 3, 4, 5)
        if ranks.contains(&14)
            && ranks.contains(&5)
            && ranks.contains(&4)
            && ranks.contains(&3)
            && ranks.contains(&2)
        {
            return true;
        }

        false
    }

    fn get_rank_counts(cards: &[Card]) -> HashMap<Rank, usize> {
        let mut counts = HashMap::new();
        for card in cards {
            *counts.entry(card.rank).or_insert(0) += 1;
        }
        counts
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_str: Vec<String> = self.cards.iter().map(|card| card.to_string()).collect();
        write!(f, "{} ({})", cards_str.join(" "), self.rank)
    }
}

// Helper function to generate combinations
pub fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if items.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();
    let first = &items[0];
    let rest = &items[1..];

    // Include first item
    for mut combo in combinations(rest, k - 1) {
        combo.insert(0, first.clone());
        result.push(combo);
    }

    // Exclude first item
    result.extend(combinations(rest, k));

    result
}
