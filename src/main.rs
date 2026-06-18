mod card;
mod deck;
mod hand;
mod player;
mod rules;
mod simulator;

use hand::HandRank;
use rayon::prelude::*;
use rules::GameRules;
use simulator::PokerSimulator;
use std::collections::HashMap;

fn main() {
    let num_simulations = 100000;

    // Swap with GameRules::omaha(n), GameRules::five_card_draw(n), etc to change rules
    let rules = GameRules::omaha(8);

    println!(
        "Running {} simulations across {} threads",
        num_simulations,
        rayon::current_num_threads()
    );
    println!(
        "Rules: {} players, {} hole + {} community cards, {} deck(s), {} wild card(s)\n",
        rules.num_players,
        rules.hole_cards_per_player,
        rules.community_cards,
        rules.num_decks,
        rules.wild_cards.len(),
    );

    let hand_type_counts: HashMap<HandRank, usize> = (0..num_simulations)
        .into_par_iter()
        .fold(HashMap::new, |mut counts, _| {
            let mut simulator = PokerSimulator::new(rules.clone());
            simulator.simulate_complete_hand();
            for hand_type in simulator.get_hand_types() {
                *counts.entry(hand_type).or_insert(0) += 1;
            }
            counts
        })
        .reduce(HashMap::new, |mut acc, partial| {
            for (hand_type, count) in partial {
                *acc.entry(hand_type).or_insert(0) += count;
            }
            acc
        });

    let total_hands = num_simulations * rules.num_players;
    println!("Hand Type Distribution ({} hands):", total_hands);
    for (hand_type, count) in &hand_type_counts {
        let percentage = (*count as f64) / (total_hands as f64) * 100.0;
        println!("  {:?}: {:.2}%", hand_type, percentage);
    }
}
