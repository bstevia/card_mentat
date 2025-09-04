use crate::card::Card;
use crate::deck::Deck;
use crate::hand::HandRank;
use crate::player::Player;

#[derive(Debug)]
pub struct PokerSimulator {
    pub players: Vec<Player>,
    pub community_cards: Vec<Card>,
    pub wild_cards: Vec<Card>,
    pub hole_cards_per_player: usize,
    deck: Deck,
}

impl PokerSimulator {
    pub fn new(player_count: usize, hole_cards_per_player: usize, wild_cards: Vec<Card>) -> Self {
        let mut players = Vec::new();
        for i in 1..=player_count {
            players.push(Player::new(i));
        }

        PokerSimulator {
            players,
            community_cards: Vec::new(),
            wild_cards,
            hole_cards_per_player,
            deck: Deck::new(),
        }
    }

    pub fn deal_hand(&mut self) {
        self.deck = Deck::new();
        self.deck.shuffle();
        self.community_cards.clear();

        // Clear previous hands
        for player in &mut self.players {
            player.hole_cards.clear();
            player.best_hand = None;
        }

        // Deal hole cards
        for _ in 0..self.hole_cards_per_player {
            for player in &mut self.players {
                if let Some(card) = self.deck.deal() {
                    player.add_hole_card(card);
                }
            }
        }
    }

    pub fn deal_community_cards(&mut self, count: usize) {
        for _ in 0..count {
            if let Some(card) = self.deck.deal() {
                self.community_cards.push(card);
            }
        }
    }

    pub fn evaluate_all_hands(&mut self) {
        for player in &mut self.players {
            player.evaluate_hand(&self.community_cards, &self.wild_cards);
        }
    }

    pub fn get_winners(&self) -> Vec<&Player> {
        let mut winners = Vec::new();
        let mut best_rank = HandRank::HighCard;

        for player in &self.players {
            if let Some(ref hand) = player.best_hand {
                if hand.rank > best_rank {
                    best_rank = hand.rank;
                    winners.clear();
                    winners.push(player);
                } else if hand.rank == best_rank {
                    winners.push(player);
                }
            }
        }

        winners
    }

    pub fn simulate_complete_hand(&mut self) {
        self.deal_hand();

        // Deal community cards
        if self.hole_cards_per_player == 2 {
            self.deal_community_cards(5);
        }

        self.evaluate_all_hands();
    }

    pub fn get_hand_types(&self) -> Vec<HandRank> {
        self.players
            .iter()
            .filter_map(|player| player.best_hand.as_ref().map(|hand| hand.rank.clone()))
            .collect()
    }

    pub fn print_game_state(&self) {
        println!("=== Poker Hand Simulation ===");
        println!("Players: {}", self.players.len());
        println!("Hole cards per player: {}", self.hole_cards_per_player);

        if !self.wild_cards.is_empty() {
            let wild_cards_str: Vec<String> = self
                .wild_cards
                .iter()
                .map(|card| card.to_string())
                .collect();
            println!("Wild cards: {}", wild_cards_str.join(" "));
        }

        if !self.community_cards.is_empty() {
            let community_str: Vec<String> = self
                .community_cards
                .iter()
                .map(|card| card.to_string())
                .collect();
            println!("Community cards: {}", community_str.join(" "));
        }

        println!("\n--- Players ---");
        for player in &self.players {
            println!("{}", player);
        }

        let winners = self.get_winners();
        if !winners.is_empty() {
            println!("\n--- Winners ---");
            for winner in winners {
                println!("{}", winner);
            }
        }

        println!("\nCards remaining in deck: {}", self.deck.remaining());
    }
}
