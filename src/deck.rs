use crate::card::{Card, Rank, Suit};
use rand::rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(num_decks: usize) -> Self {
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let ranks = [
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

        let mut cards = Vec::with_capacity(52 * num_decks);
        for _ in 0..num_decks {
            for suit in &suits {
                for rank in &ranks {
                    cards.push(Card::new(*rank, *suit));
                }
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}
