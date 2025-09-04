mod card;
mod deck;
mod hand;
mod player;
mod simulator;

use card::{Card, Rank, Suit};
use hand::HandRank;
use simulator::PokerSimulator;
use std::collections::HashMap;

fn main() {
    println!("Poker Hand Simulator in Rust");
    println!("============================\n");

    let mut hand_type_counts: HashMap<HandRank, usize> = HashMap::new();
    let num_simulations = 10000;
    let num_players = 4;
    let num_hole_cards = 2;
    println!(
        "\nRunning {} simulations for Texas Hold'em...",
        num_simulations
    );

    for _ in 0..num_simulations {
        let mut simulator = PokerSimulator::new(num_players, num_hole_cards, vec![]);
        simulator.simulate_complete_hand();
        for hand_type in simulator.get_hand_types() {
            *hand_type_counts.entry(hand_type).or_insert(0) += 1;
        }
    }

    println!("\nHand Type Percentages (Texas Hold'em, 1000 runs):");
    let total_hands = num_simulations * num_players;
    for (hand_type, count) in &hand_type_counts {
        let percentage = (*count as f64) / (total_hands as f64) * 100.0;
        println!("{:?}: {:.2}%", hand_type, percentage);
    }

    // Example 2: 5-card draw with wild cards
    println!("Example 2: 5-Card Draw with Deuces Wild (3 players)");
    let wild_cards = vec![
        Card::new(Rank::Two, Suit::Hearts),
        Card::new(Rank::Two, Suit::Diamonds),
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Two, Suit::Spades),
    ];
    let mut simulator2 = PokerSimulator::new(3, 5, wild_cards);
    simulator2.deal_hand();
    simulator2.evaluate_all_hands();
    simulator2.print_game_state();
    println!("\n{}\n", "=".repeat(50));

    println!("\n{}", "=".repeat(50));
    println!("Simulation complete! You can customize:");
    println!("- Number of players");
    println!("- Number of hole cards per player");
    println!("- Wild cards");
    println!("- Community cards");
}
